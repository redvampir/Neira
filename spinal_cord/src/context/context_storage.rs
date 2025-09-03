use crate::nervous_system::anti_idle;
use chrono::{Datelike, Utc};
use flate2::write::GzEncoder;
use flate2::Compression;
use fs2::{available_space, total_space, FileExt};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use tokio::sync::mpsc;
use tokio::task::spawn_blocking;
use tokio::time::{interval, Duration};
// metrics integration can be added via `metrics` crate if desired

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Clone)]
pub struct RuntimeMaskCfg {
    pub enabled: bool,
    pub regex: Vec<Regex>,
    pub roles: Vec<Role>,
}

static RUNTIME_MASK: Lazy<RwLock<Option<RuntimeMaskCfg>>> = Lazy::new(|| RwLock::new(None));

pub fn set_runtime_mask_config(
    enabled: Option<bool>,
    regex: Option<Vec<String>>,
    roles: Option<Vec<String>>,
) -> Result<(), String> {
    let mut guard = RUNTIME_MASK.write().map_err(|_| "lock".to_string())?;
    let current = guard.clone();
    let mut cfg = current.unwrap_or(RuntimeMaskCfg {
        enabled: true,
        regex: Vec::new(),
        roles: vec![Role::User],
    });
    if let Some(e) = enabled {
        cfg.enabled = e;
    }
    if let Some(list) = regex {
        let mut out = Vec::new();
        for p in list {
            if let Ok(r) = Regex::new(&p) {
                out.push(r);
            }
        }
        cfg.regex = out;
    }
    if let Some(rs) = roles {
        let mut out = Vec::new();
        for r in rs {
            match r.to_lowercase().as_str() {
                "user" => out.push(Role::User),
                "assistant" => out.push(Role::Assistant),
                "system" => out.push(Role::System),
                _ => {}
            }
        }
        if !out.is_empty() {
            cfg.roles = out;
        }
    }
    *guard = Some(cfg);
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskConfigPublic {
    pub enabled: bool,
    pub regex: Vec<String>,
    pub roles: Vec<String>,
}

pub fn get_runtime_mask_config() -> MaskConfigPublic {
    if let Ok(guard) = RUNTIME_MASK.read() {
        if let Some(cfg) = guard.as_ref() {
            return MaskConfigPublic {
                enabled: cfg.enabled,
                regex: cfg.regex.iter().map(|r| r.as_str().to_string()).collect(),
                roles: cfg
                    .roles
                    .iter()
                    .map(|r| match r {
                        Role::User => "user".into(),
                        Role::Assistant => "assistant".into(),
                        Role::System => "system".into(),
                    })
                    .collect(),
            };
        }
    }
    MaskConfigPublic {
        enabled: true,
        regex: vec![],
        roles: vec!["user".into()],
    }
}

pub fn mask_preview(
    text: &str,
    custom_regex: Option<Vec<String>>,
    roles: Option<Vec<String>>,
) -> Result<String, String> {
    // Build regex list: custom or from runtime
    let regexes: Vec<Regex> = if let Some(list) = custom_regex {
        let mut out = Vec::new();
        for p in list {
            out.push(Regex::new(&p).map_err(|e| e.to_string())?);
        }
        out
    } else if let Ok(guard) = RUNTIME_MASK.read() {
        if let Some(cfg) = guard.as_ref() {
            cfg.regex.clone()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };
    // roles currently unused for preview, but kept for future specificity
    let _roles = roles;
    Ok(FileContextStorage::mask_content_custom(text, &regexes))
}

pub fn load_mask_preset(name: &str) -> Result<Vec<String>, String> {
    let dir = std::env::var("MASK_PRESETS_DIR").unwrap_or_else(|_| "config/mask_presets".into());
    let path = std::path::Path::new(&dir).join(format!("{}.txt", name));
    let data =
        std::fs::read_to_string(&path).map_err(|e| format!("read {}: {}", path.display(), e))?;
    let mut out = Vec::new();
    for line in data.lines() {
        let lt = line.trim();
        if lt.is_empty() || lt.starts_with('#') {
            continue;
        }
        out.push(lt.to_string());
    }
    Ok(out)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
    pub timestamp_ms: i64,
    pub source: Option<String>,
    pub message_id: Option<u64>,
    pub thread_id: Option<String>,
    pub parent_id: Option<String>,
}

pub trait ContextStorage: Send + Sync {
    fn save_message(
        &self,
        chat_id: &str,
        session_id: &str,
        message: &ChatMessage,
    ) -> Result<(), String>;

    fn load_session(&self, chat_id: &str, session_id: &str) -> Result<Vec<ChatMessage>, String>;
}

#[derive(Clone)]
pub struct FileContextStorage {
    root: PathBuf,
    cfg: Config,
    tx: Option<mpsc::Sender<(String, String, ChatMessage)>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StorageMetrics {
    disk_total: u64,
    disk_available: u64,
    avg_msg_bytes: u64,
    max_lines: usize,
    max_bytes: u64,
    updated_ms: i64,
}

#[derive(Clone)]
struct Config {
    max_lines: usize,
    max_bytes: u64,
    daily_rotation: bool,
    archive_gz: bool,
    flush_interval_ms: u64,
    mask_enabled: bool,
    mask_regex: Vec<Regex>,
    mask_roles: Vec<Role>,
    metrics_path: PathBuf,
}

fn load_or_init_metrics(root: &Path) -> StorageMetrics {
    let path = root.join("storage_metrics.json");
    if let Ok(data) = fs::read_to_string(&path) {
        if let Ok(m) = serde_json::from_str::<StorageMetrics>(&data) {
            return m;
        }
    }
    fs::create_dir_all(root).ok();
    let total = total_space(root).unwrap_or(0);
    let free = available_space(root).unwrap_or(0);
    let avg_msg_bytes: u64 = 1024; // initial guess, updated later
    let max_bytes = (free / 100).max(avg_msg_bytes);
    let max_lines = (max_bytes / avg_msg_bytes.max(1)) as usize;
    let metrics = StorageMetrics {
        disk_total: total,
        disk_available: free,
        avg_msg_bytes,
        max_lines,
        max_bytes,
        updated_ms: Utc::now().timestamp_millis(),
    };
    let _ = fs::write(&path, serde_json::to_string_pretty(&metrics).unwrap());
    metrics
}

fn update_storage_metrics(cfg: &Config, added_bytes: u64, lines: usize) {
    if lines == 0 {
        return;
    }
    let root = cfg.metrics_path.parent().unwrap_or(Path::new("."));
    let mut metrics = load_or_init_metrics(root);
    let new_avg = added_bytes / lines as u64;
    metrics.avg_msg_bytes = if metrics.avg_msg_bytes == 0 {
        new_avg
    } else {
        (metrics.avg_msg_bytes + new_avg) / 2
    };
    metrics.disk_total = total_space(root).unwrap_or(metrics.disk_total);
    metrics.disk_available = available_space(root).unwrap_or(metrics.disk_available);
    let suggested = metrics.disk_available / 100;
    metrics.max_bytes = suggested.max(metrics.avg_msg_bytes * 100);
    metrics.max_lines = (metrics.max_bytes / metrics.avg_msg_bytes.max(1)).max(1) as usize;
    metrics.updated_ms = Utc::now().timestamp_millis();
    let _ = fs::write(
        &cfg.metrics_path,
        serde_json::to_string_pretty(&metrics).unwrap(),
    );
}

impl FileContextStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let root = std::env::var("CONTEXT_DIR")
            .ok()
            .map(PathBuf::from)
            .unwrap_or_else(|| root.into());
        let cfg = Config::from_env(&root);
        if cfg.flush_interval_ms > 0 {
            let (tx, mut rx) = mpsc::channel::<(String, String, ChatMessage)>(1024);
            let root_clone = root.clone();
            let cfg_clone = cfg.clone();
            tokio::spawn(async move {
                let mut tick = interval(Duration::from_millis(cfg_clone.flush_interval_ms));
                let mut pending: Vec<(String, String, ChatMessage)> = Vec::new();
                loop {
                    tokio::select! {
                        _ = tick.tick() => {
                            // flush pending
                            if !pending.is_empty() {
                                let batch = std::mem::take(&mut pending);
                                let _ = flush_batch(&root_clone, &cfg_clone, batch);
                            }
                        }
                        msg = rx.recv() => {
                            if let Some(item) = msg { pending.push(item); }
                            else { break; }
                        }
                    }
                }
            });
            Self {
                root,
                cfg,
                tx: Some(tx),
            }
        } else {
            Self {
                root,
                cfg,
                tx: None,
            }
        }
    }
}

impl ContextStorage for FileContextStorage {
    fn save_message(
        &self,
        chat_id: &str,
        session_id: &str,
        message: &ChatMessage,
    ) -> Result<(), String> {
        // Anti-Idle: отметим активность при записи контекста
        anti_idle::mark_activity();
        let chat = chat_id.to_string();
        let sess = session_id.to_string();
        let mut msg = message.clone();
        // runtime mask override if present, else static cfg
        if let Ok(guard) = RUNTIME_MASK.read() {
            if let Some(rm) = guard.as_ref() {
                if rm.enabled && rm.roles.contains(&msg.role) {
                    msg.content = Self::mask_content_custom(&msg.content, &rm.regex);
                }
            } else if self.cfg.mask_enabled && self.cfg.mask_roles.contains(&msg.role) {
                msg.content = Self::mask_content_custom(&msg.content, &self.cfg.mask_regex);
            }
        }

        metrics::counter!("messages_saved").increment(1);

        if let Some(tx) = &self.tx {
            // buffered mode
            let _ = tx.try_send((chat, sess, msg));
        } else {
            // direct write
            let root = self.root.clone();
            let cfg = self.cfg.clone();
            spawn_blocking(move || write_one(&root, &cfg, &chat, &sess, msg));
        }

        Ok(())
    }

    fn load_session(&self, chat_id: &str, session_id: &str) -> Result<Vec<ChatMessage>, String> {
        // Anti-Idle: отметим активность при чтении контекста
        anti_idle::mark_activity();
        metrics::counter!("context_loads").increment(1);
        // Read all (possibly rotated) files
        let dir = self.root.join(chat_id);
        let mut files: Vec<PathBuf> = Vec::new();
        let prefix = format!("{}-", session_id);
        if let Ok(rd) = fs::read_dir(&dir) {
            for e in rd.flatten() {
                let p = e.path();
                let fname = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if fname == format!("{}.ndjson", session_id)
                    || (fname.starts_with(&prefix)
                        && (fname.ends_with(".ndjson") || fname.ends_with(".ndjson.gz")))
                {
                    files.push(p);
                }
            }
        }
        files.sort();
        let mut out = Vec::new();
        for p in files {
            if p.extension().and_then(|s| s.to_str()) == Some("gz") {
                let data = fs::read(&p).map_err(|e| e.to_string())?;
                let mut d = flate2::read::GzDecoder::new(&data[..]);
                let mut s = String::new();
                d.read_to_string(&mut s).map_err(|e| e.to_string())?;
                for l in s.lines() {
                    if l.trim().is_empty() {
                        continue;
                    }
                    if let Ok(msg) = serde_json::from_str::<ChatMessage>(l) {
                        out.push(msg);
                    }
                }
            } else if p.extension().and_then(|s| s.to_str()) == Some("ndjson") {
                let file = fs::File::open(&p).map_err(|e| e.to_string())?;
                let reader = std::io::BufReader::new(file);
                for l in reader.lines().map_while(Result::ok) {
                    if l.trim().is_empty() {
                        continue;
                    }
                    if let Ok(msg) = serde_json::from_str::<ChatMessage>(&l) {
                        out.push(msg);
                    }
                }
            }
        }
        if out.is_empty() {
            metrics::counter!("context_misses").increment(1);
        }
        Ok(out)
    }
}

impl FileContextStorage {
    pub fn import_messages(
        &self,
        chat_id: &str,
        session_id: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<(), String> {
        let root = self.root.clone();
        let cfg = self.cfg.clone();
        let chat = chat_id.to_string();
        let sess = session_id.to_string();
        spawn_blocking(move || {
            let dir = root.join(&chat);
            fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let path = if cfg.daily_rotation {
                let date = format!(
                    "{:04}{:02}{:02}",
                    Utc::now().year(),
                    Utc::now().month(),
                    Utc::now().day()
                );
                dir.join(format!("{}-{}.ndjson", sess, date))
            } else {
                dir.join(format!("{}.ndjson", sess))
            };
            append_messages_and_update_index(&cfg, &path, messages)
        });
        Ok(())
    }
    pub fn mask_content_custom(content: &str, custom: &[Regex]) -> String {
        let mut s = content.to_string();
        // Mask emails
        if let Ok(re) = Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}") {
            s = re.replace_all(&s, "[email]").to_string();
        }
        // Mask phone-like digits
        if let Ok(re) = Regex::new(r"\+?\d[\d\s().-]{7,}\d") {
            s = re.replace_all(&s, "[phone]").to_string();
        }
        for re in custom {
            s = re.replace_all(&s, "[pii]").to_string();
        }
        s
    }
}

impl Config {
    fn from_env(root: &Path) -> Self {
        let metrics = load_or_init_metrics(root);
        let max_lines = std::env::var("CONTEXT_MAX_LINES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(metrics.max_lines);
        let max_bytes = std::env::var("CONTEXT_MAX_BYTES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(metrics.max_bytes);
        let daily_rotation = std::env::var("CONTEXT_DAILY_ROTATION")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        let archive_gz = std::env::var("CONTEXT_ARCHIVE_GZ")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        let flush_interval_ms = std::env::var("CONTEXT_FLUSH_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        let mask_enabled = std::env::var("MASK_PII")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        let mask_regex = std::env::var("MASK_REGEX")
            .ok()
            .map(|s| s.split(';').filter_map(|p| Regex::new(p).ok()).collect())
            .unwrap_or_default();
        let mask_roles = std::env::var("MASK_ROLES")
            .ok()
            .map(|s| {
                s.split(',')
                    .filter_map(|r| match r.trim().to_lowercase().as_str() {
                        "user" => Some(Role::User),
                        "assistant" => Some(Role::Assistant),
                        "system" => Some(Role::System),
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_else(|| vec![Role::User]);
        Self {
            max_lines,
            max_bytes,
            daily_rotation,
            archive_gz,
            flush_interval_ms,
            mask_enabled,
            mask_regex,
            mask_roles,
            metrics_path: root.join("storage_metrics.json"),
        }
    }
}

fn write_one(
    root: &Path,
    cfg: &Config,
    chat: &str,
    sess: &str,
    msg: ChatMessage,
) -> Result<(), String> {
    let dir = root.join(chat);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    ensure_daily_archive(cfg, sess, &dir)?;
    let path = if cfg.daily_rotation {
        let date = format!(
            "{:04}{:02}{:02}",
            Utc::now().year(),
            Utc::now().month(),
            Utc::now().day()
        );
        dir.join(format!("{}-{}.ndjson", sess, date))
    } else {
        dir.join(format!("{}.ndjson", sess))
    };
    append_messages_and_update_index(cfg, &path, vec![msg])
}

fn flush_batch(
    root: &Path,
    cfg: &Config,
    batch: Vec<(String, String, ChatMessage)>,
) -> Result<(), String> {
    // group by (chat, session)
    use std::collections::BTreeMap;
    let mut groups: BTreeMap<(String, String, PathBuf), Vec<ChatMessage>> = BTreeMap::new();
    for (chat, sess, msg) in batch {
        let dir = root.join(&chat);
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        ensure_daily_archive(cfg, &sess, &dir)?;
        let path = if cfg.daily_rotation {
            let date = format!(
                "{:04}{:02}{:02}",
                Utc::now().year(),
                Utc::now().month(),
                Utc::now().day()
            );
            dir.join(format!("{}-{}.ndjson", sess, date))
        } else {
            dir.join(format!("{}.ndjson", sess))
        };
        groups
            .entry((chat.clone(), sess.clone(), path))
            .or_default()
            .push(msg);
    }
    for ((_chat, _sess, path), msgs) in groups {
        append_messages_and_update_index(cfg, &path, msgs)?;
    }
    Ok(())
}

fn append_messages_and_update_index(
    cfg: &Config,
    path: &Path,
    mut msgs: Vec<ChatMessage>,
) -> Result<(), String> {
    let dir = path.parent().unwrap();
    let index_path = dir.join("index.json");
    let mut index: serde_json::Value = if index_path.exists() {
        let s = fs::read_to_string(&index_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&s).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };
    if !index.is_object() {
        index = serde_json::json!({});
    }
    let sess_key = session_name_from_path(path).unwrap_or_default();
    let map = index.as_object_mut().unwrap();
    if !map.contains_key(&sess_key) {
        map.insert(
            sess_key.clone(),
            serde_json::json!({
                "updated_ms": 0_i64,
                "message_count": 0_u64,
                "approx_bytes": 0_u64,
                "keywords": [],
                "last_id": 0_u64,
                "kw_updated_ms": 0_i64,
            }),
        );
    }
    let entry = map.get_mut(&sess_key).unwrap().as_object_mut().unwrap();
    let mut last_id = entry.get("last_id").and_then(|v| v.as_u64()).unwrap_or(0);

    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.lock_exclusive().map_err(|e| e.to_string())?;
    let mut approx_bytes = entry
        .get("approx_bytes")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let mut cnt = entry
        .get("message_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let mut added_bytes = 0_u64;
    for m in msgs.iter_mut() {
        if m.message_id.is_none() {
            last_id += 1;
            m.message_id = Some(last_id);
        } else {
            last_id = last_id.max(m.message_id.unwrap());
        }
        let line = serde_json::to_string(m).map_err(|e| e.to_string())? + "\n";
        approx_bytes += line.len() as u64;
        added_bytes += line.len() as u64;
        cnt += 1;
        f.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
        metrics::counter!("context_bytes_written").increment(line.len() as u64);
        // metrics: increment counters and histograms here if needed
    }
    let _ = f.unlock();

    let updated_ms = Utc::now().timestamp_millis();
    entry.insert("updated_ms".into(), serde_json::json!(updated_ms));
    entry.insert("message_count".into(), serde_json::json!(cnt));
    entry.insert("approx_bytes".into(), serde_json::json!(approx_bytes));
    entry.insert("last_id".into(), serde_json::json!(last_id));
    // keywords maintenance with TTL
    let now_ms = Utc::now().timestamp_millis();
    let ttl_days: i64 = std::env::var("INDEX_KW_TTL_DAYS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(90);
    let ttl_ms = ttl_days.max(0) * 86_400_000;
    if let Some(kw_ts) = entry.get("kw_updated_ms").and_then(|v| v.as_i64()) {
        if ttl_ms > 0 && now_ms.saturating_sub(kw_ts) > ttl_ms {
            entry.insert("keywords".into(), serde_json::Value::Array(Vec::new()));
        }
    }
    // naive keywords from last message
    let mut kws = entry
        .get("keywords")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let new_kws = super_keywords(msgs.last().map(|m| m.content.as_str()).unwrap_or(""));
    for k in new_kws {
        if !kws.contains(&serde_json::json!(k)) && kws.len() < 32 {
            kws.push(serde_json::json!(k));
        }
    }
    entry.insert("keywords".into(), serde_json::Value::Array(kws));
    entry.insert("kw_updated_ms".into(), serde_json::json!(now_ms));
    fs::write(&index_path, serde_json::to_string_pretty(&index).unwrap())
        .map_err(|e| e.to_string())?;

    update_storage_metrics(cfg, added_bytes, msgs.len());

    // size-based trim
    if cfg.max_bytes > 0 {
        if let Ok(meta) = fs::metadata(path) {
            if meta.len() > cfg.max_bytes {
                let file = fs::File::open(path).map_err(|e| e.to_string())?;
                let reader = std::io::BufReader::new(file);
                let mut lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
                if lines.len() > cfg.max_lines {
                    lines = lines.split_off(lines.len().saturating_sub(cfg.max_lines));
                }
                fs::write(path, lines.join("\n") + "\n").map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

fn ensure_daily_archive(cfg: &Config, sess: &str, dir: &Path) -> Result<(), String> {
    if !cfg.daily_rotation || !cfg.archive_gz {
        return Ok(());
    }
    let today = format!(
        "{:04}{:02}{:02}",
        Utc::now().year(),
        Utc::now().month(),
        Utc::now().day()
    );
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                if name.starts_with(&format!("{}-", sess))
                    && name.ends_with(".ndjson")
                    && !name.contains(&today)
                {
                    let data = fs::read(&p).map_err(|e| e.to_string())?;
                    let mut gz = GzEncoder::new(Vec::new(), Compression::default());
                    gz.write_all(&data).map_err(|e| e.to_string())?;
                    let out = gz.finish().map_err(|e| e.to_string())?;
                    fs::write(p.with_extension("ndjson.gz"), out).map_err(|e| e.to_string())?;
                    let _ = fs::remove_file(&p);
                    metrics::counter!("gz_rotate_count").increment(1);
                }
            }
        }
    }
    Ok(())
}

fn session_name_from_path(path: &Path) -> Option<String> {
    let name = path.file_name()?.to_str()?;
    if let Some(n) = name.strip_suffix(".ndjson") {
        return Some(n.to_string());
    }
    if let Some(n) = name.strip_suffix(".ndjson.gz") {
        return Some(n.to_string());
    }
    Some(name.to_string())
}

fn super_keywords(content: &str) -> Vec<String> {
    content
        .split(|c: char| !c.is_alphanumeric())
        .filter_map(|w| {
            let lw = w.to_lowercase();
            if lw.len() >= 4 {
                Some(lw)
            } else {
                None
            }
        })
        .take(16)
        .collect()
}
/* neira:meta
id: NEI-20250829-setup-meta-storage
intent: docs
scope: backend/storage
summary: |
  Файловое хранилище контекста (ndjson + дневная ротация + gzip), индекс index.json,
  TTL ключевых слов, адаптивные лимиты по диску (storage_metrics.json), маскирование
  (runtime + пресеты), буферизация записи, импорта.
links:
  - docs/reference/env.md
  - docs/reference/metrics.md
env:
  - CONTEXT_DIR
  - CONTEXT_MAX_LINES
  - CONTEXT_MAX_BYTES
  - CONTEXT_DAILY_ROTATION
  - CONTEXT_ARCHIVE_GZ
  - CONTEXT_FLUSH_MS
  - MASK_PII
  - MASK_REGEX
  - MASK_ROLES
  - INDEX_KW_TTL_DAYS
  - MASK_PRESETS_DIR
metrics:
  - messages_saved
  - context_loads
  - context_misses
  - context_bytes_written
  - gz_rotate_count
risks: low
safe_mode:
  affects_write: true
  requires_admin: false
i18n:
  reviewer_note: |
    Важные файлы и индекс. Не забывай обновлять ENV‑референс при добавлении флагов.
*/
/* neira:meta
id: NEI-20240513-storage-lints
intent: chore
summary: Убраны предупреждения Clippy в файловом хранилище контекста: лишние приведения, manual_flatten и др.
*/
