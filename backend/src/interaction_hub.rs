use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::action::diagnostics_node::DiagnosticsNode;
use crate::action::metrics_collector_node::{MetricsCollectorNode, MetricsRecord};
use crate::config::Config;
use crate::context::context_storage::{ContextStorage, ChatMessage, Role};
use crate::idempotent_store::IdempotentStore;
use crate::security::integrity_checker_node::IntegrityCheckerNode;
use crate::security::quarantine_node::QuarantineNode;
use crate::security::safe_mode_controller::SafeModeController;
use crate::system::{host_metrics::HostMetrics, io_watcher::IoWatcher, SystemProbe};
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::task::{spawn_blocking, JoinHandle};
use tokio::time::{interval, sleep};
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::analysis_node::{AnalysisResult, NodeStatus};
use crate::memory_node::MemoryNode;
use crate::node_registry::NodeRegistry;
use crate::task_scheduler::{Queue, TaskScheduler};
use crate::trigger_detector::TriggerDetector;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope { Read, Write, Admin }

#[derive(Clone, Debug)]
struct TokenInfo { scopes: Vec<Scope> }

pub struct InteractionHub {
    pub registry: Arc<NodeRegistry>,
    pub memory: Arc<MemoryNode>,
    metrics: Arc<MetricsCollectorNode>,
    diagnostics: Arc<DiagnosticsNode>,
    trigger_detector: Arc<TriggerDetector>,
    scheduler: RwLock<TaskScheduler>,
    allowed_tokens: RwLock<std::collections::HashMap<String, TokenInfo>>,
    rate: RwLock<std::collections::HashMap<String, (u64, u32)>>,
    rate_limit_per_min: u32,
    rate_key_mode: RateKeyMode,
    requests: RwLock<LruCache<String, String>>,
    idem: Option<IdempotentStore>,
    persist_require_session_id: bool,
    probe_handles: RwLock<std::collections::HashMap<String, JoinHandle<()>>>,
    io_watcher_threshold_ms: u64,
    safe_mode: Arc<SafeModeController>,
    cancels: RwLock<std::collections::HashMap<(String, String), tokio_util::sync::CancellationToken>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RateKeyMode {
    Auth,
    Chat,
    Session,
}

pub struct ChatOutput {
    pub response: String,
    pub session_id: Option<String>,
    pub idempotent: bool,
}

impl InteractionHub {
    pub fn new(
        registry: Arc<NodeRegistry>,
        memory: Arc<MemoryNode>,
        metrics: Arc<MetricsCollectorNode>,
        diagnostics: Arc<DiagnosticsNode>,
        config: &Config,
    ) -> Self {
        let rate_limit_per_min = std::env::var("CHAT_RATE_LIMIT_PER_MIN")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(120);
        let rate_key = std::env::var("CHAT_RATE_KEY").unwrap_or_else(|_| "auth".into());
        let rate_key_mode = match rate_key.to_lowercase().as_str() {
            "auth" => RateKeyMode::Auth,
            "chat" => RateKeyMode::Chat,
            "session" => RateKeyMode::Session,
            _ => RateKeyMode::Auth,
        };
        let idem_persist = std::env::var("IDEMPOTENT_PERSIST")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let idem = if idem_persist {
            let dir = std::env::var("IDEMPOTENT_STORE_DIR").unwrap_or_else(|_| "context".into());
            let ttl = std::env::var("IDEMPOTENT_TTL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(86_400);
            Some(IdempotentStore::new(dir, ttl))
        } else {
            None
        };
        let persist_require_session_id = std::env::var("PERSIST_REQUIRE_SESSION_ID")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let io_watcher_threshold_ms = std::env::var("IO_WATCHER_THRESHOLD_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        let host_metrics_enabled = config
            .probes
            .get("host_metrics")
            .map_or(true, |p| p.enabled);
        let io_watcher_enabled = config.probes.get("io_watcher").map_or(false, |p| p.enabled);

        registry.register_action_node(metrics.clone());
        registry.register_action_node(diagnostics.clone());
        registry.register_action_node(Arc::new(
            crate::system::base_path_resolver::BasePathResolverNode::new(),
        ));
        let safe_mode = SafeModeController::new();
        let (quarantine, quarantine_tx, _dev_rx) = QuarantineNode::new(safe_mode.clone());
        registry.register_action_node(quarantine);
        registry.register_action_node(IntegrityCheckerNode::new(memory.clone(), quarantine_tx));

        let hub = Self {
            registry,
            memory,
            metrics: metrics.clone(),
            diagnostics,
            trigger_detector: Arc::new(TriggerDetector::default()),
            scheduler: RwLock::new(TaskScheduler::default()),
            allowed_tokens: RwLock::new(std::collections::HashMap::new()),
            rate: RwLock::new(std::collections::HashMap::new()),
            rate_limit_per_min,
            rate_key_mode,
            requests: RwLock::new(LruCache::new(NonZeroUsize::new(10_000).unwrap())),
            idem,
            persist_require_session_id,
            probe_handles: RwLock::new(std::collections::HashMap::new()),
            io_watcher_threshold_ms,
            safe_mode,
            cancels: RwLock::new(std::collections::HashMap::new()),
        };

        // Spawn host metrics polling loop
        if host_metrics_enabled {
            let mut host_metrics = HostMetrics::new(hub.metrics.clone());
            let handle = tokio::spawn(async move {
                host_metrics.start().await;
            });
            hub.probe_handles
                .write()
                .unwrap()
                .insert("host_metrics".into(), handle);
        }

        // Optionally spawn IO watcher
        if io_watcher_enabled {
            let mut watcher = IoWatcher::new(hub.metrics.clone(), io_watcher_threshold_ms);
            let handle = tokio::spawn(async move {
                watcher.start().await;
            });
            hub.probe_handles
                .write()
                .unwrap()
                .insert("io_watcher".into(), handle);
        }

        hub
    }

    pub fn toggle_probe(&self, name: &str) -> Result<bool, String> {
        let mut probes = self.probe_handles.write().unwrap();
        if let Some(handle) = probes.remove(name) {
            handle.abort();
            return Ok(false);
        }
        let handle = match name {
            "host_metrics" => {
                let mut probe = HostMetrics::new(self.metrics.clone());
                tokio::spawn(async move { probe.start().await })
            }
            "io_watcher" => {
                let mut watcher =
                    IoWatcher::new(self.metrics.clone(), self.io_watcher_threshold_ms);
                tokio::spawn(async move { watcher.start().await })
            }
            _ => return Err(format!("unknown probe {name}")),
        };
        probes.insert(name.to_string(), handle);
        Ok(true)
    }

    pub fn add_auth_token(&self, token: impl Into<String>) {
        // backwards compatible: full scopes
        self.add_token_with_scopes(token, &[Scope::Read, Scope::Write, Scope::Admin]);
    }

    pub fn add_token_with_scopes(&self, token: impl Into<String>, scopes: &[Scope]) {
        let t = token.into();
        self.allowed_tokens
            .write()
            .unwrap()
            .insert(t, TokenInfo { scopes: scopes.to_vec() });
    }

    fn authorize(&self, token: &str) -> bool {
        self.allowed_tokens.read().unwrap().contains_key(token)
    }

    pub fn check_auth(&self, token: &str) -> bool {
        self.authorize(token)
    }

    pub fn check_scope(&self, token: &str, scope: Scope) -> bool {
        if let Some(info) = self.allowed_tokens.read().unwrap().get(token) {
            if scope == Scope::Write && self.safe_mode.is_safe_mode() {
                // in safe mode only admin can write
                return info.scopes.contains(&Scope::Admin);
            }
            info.scopes.contains(&scope) || info.scopes.contains(&Scope::Admin)
        } else { false }
    }

    pub fn add_trigger_keyword(&self, keyword: impl Into<String>) {
        self.trigger_detector.add_keyword(keyword.into());
    }

    pub async fn chat(
        &self,
        node_id: &str,
        chat_id: &str,
        session_id: Option<String>,
        message: &str,
        storage: &dyn ContextStorage,
        auth: &str,
        persist: bool,
        request_id: Option<String>,
        source: Option<String>,
        thread_id: Option<String>,
    ) -> Result<ChatOutput, String> {
        metrics::counter!("chat_requests_total").increment(1);
        if !self.authorize(auth) {
            metrics::counter!("chat_errors_total").increment(1);
            return Err("unauthorized".into());
        }
        // export safe mode status as gauge 0/1 for nervous system
        metrics::gauge!("safe_mode").set(if self.safe_mode.is_safe_mode() { 1.0 } else { 0.0 });
        let will_write = persist || session_id.is_some();
        if will_write && !self.check_scope(auth, Scope::Write) {
            metrics::counter!("chat_errors_total").increment(1);
            return Err("forbidden: write scope required".into());
        }
        if message.trim().is_empty() {
            metrics::counter!("chat_errors_total").increment(1);
            return Err("empty message".into());
        }

        // rate limiting (per minute)
        let key = match self.rate_key_mode {
            RateKeyMode::Auth => format!("auth:{}", auth),
            RateKeyMode::Chat => format!("chat:{}", chat_id),
            RateKeyMode::Session => match &session_id {
                Some(s) => format!("session:{}:{}", chat_id, s),
                None => format!("chat:{}", chat_id),
            },
        };
        let now_min = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs())
            / 60;
        let remaining = {
            let mut map = self.rate.write().unwrap();
            let entry = map.entry(key).or_insert((now_min, 0));
            if entry.0 != now_min {
                *entry = (now_min, 0);
            }
            if entry.1 >= self.rate_limit_per_min {
                metrics::counter!("chat_errors_total").increment(1);
                return Err("rate limited".into());
            }
            entry.1 += 1;
            self.rate_limit_per_min.saturating_sub(entry.1)
        };

        // Parse training command like: "train script=... dry_run=true"
        let mut triggers = self.trigger_detector.detect(message);
        if message.to_lowercase().starts_with("train") {
            triggers.push("train".into());
            // parse key=value with quotes
            fn parse_kv(input: &str) -> Vec<(String, String)> {
                let mut out = Vec::new();
                let mut key = String::new();
                let mut val = String::new();
                let mut in_key = true;
                let mut in_val = false;
                let mut quote: Option<char> = None;
                let mut it = input.chars().peekable();
                while let Some(ch) = it.next() {
                    if in_key {
                        if ch.is_whitespace() {
                            continue;
                        }
                        key.push(ch);
                        // read until '='
                        while let Some(c2) = it.next() {
                            if c2 == '=' {
                                in_key = false;
                                in_val = true;
                                break;
                            } else {
                                key.push(c2);
                            }
                        }
                    }
                    if in_val {
                        // skip possible spaces
                        while let Some(' ') = it.peek().copied() {
                            it.next();
                        }
                        // detect quote
                        if let Some(c) = it.peek().copied() {
                            if c == '"' || c == '\'' {
                                quote = Some(c);
                                it.next();
                            }
                        }
                        while let Some(c2) = it.next() {
                            if let Some(q) = quote {
                                if c2 == q {
                                    break;
                                }
                            } else if c2.is_whitespace() {
                                break;
                            }
                            val.push(c2);
                        }
                        out.push((key.trim().to_string(), val.clone()));
                        key.clear();
                        val.clear();
                        in_key = true;
                        in_val = false;
                        quote = None;
                    }
                }
                out
            }
            for (k, v) in parse_kv(&message[5..]) {
                // after 'train'
                match k.to_lowercase().as_str() {
                    "script" => std::env::set_var("TRAINING_SCRIPT", v),
                    "dry_run" | "dry" => std::env::set_var(
                        "TRAINING_DRY_RUN",
                        if v.eq_ignore_ascii_case("true") || v == "1" {
                            "true"
                        } else {
                            "false"
                        },
                    ),
                    _ => {}
                }
            }
        }
        // Triggers integration: preload action nodes
        for node in self.registry.action_nodes() {
            node.preload(&triggers, &self.memory);
        }

        // Metrics for incoming message
        // metrics could be recorded here via `metrics` crate

        let node = self.registry.get_chat_node(node_id).ok_or_else(|| {
            metrics::counter!("chat_errors_total").increment(1);
            "chat node not found".to_string()
        })?;

        if let Some(req_id) = &request_id {
            let cache_key = format!(
                "{}|{}|{}",
                chat_id,
                session_id.clone().unwrap_or_else(|| "<none>".into()),
                req_id
            );
            if let Some(resp) = self.requests.write().unwrap().get(&cache_key).cloned() {
                metrics::counter!("requests_idempotent_hits").increment(1);
                return Ok(ChatOutput {
                    response: resp,
                    session_id: session_id.clone(),
                    idempotent: true,
                });
            }
            if let Some(store) = &self.idem {
                if let Some(resp) = store.get(&cache_key) {
                    metrics::counter!("requests_idempotent_hits").increment(1);
                    // also warm LRU
                    self.requests
                        .write()
                        .unwrap()
                        .put(cache_key.clone(), resp.clone());
                    return Ok(ChatOutput {
                        response: resp,
                        session_id: session_id.clone(),
                        idempotent: true,
                    });
                }
            }
        }

        if persist && session_id.is_none() && self.persist_require_session_id {
            metrics::counter!("chat_errors_total").increment(1);
            return Err("session_id required for persist".into());
        }

        let sid_effective = if persist {
            Some(session_id.unwrap_or_else(|| {
                metrics::counter!("sessions_created_total").increment(1);
                metrics::gauge!("sessions_active").increment(1.0);
                metrics::counter!("sessions_autocreated_total").increment(1);
                format!(
                    "auto-{}-{:x}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                    NEXT_ID.fetch_add(1, Ordering::Relaxed)
                )
            }))
        } else {
            session_id
        };

        // Save incoming user message when writing to a session
        if let Some(ref sid) = sid_effective {
            let msg = ChatMessage {
                role: Role::User,
                content: message.to_string(),
                timestamp_ms: (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()) as i64,
                source: Some(source.clone().unwrap_or_else(|| "user".into())),
                message_id: None,
                thread_id: thread_id.clone(),
                parent_id: None,
            };
            let _ = storage.save_message(chat_id, sid, &msg);
            tracing::info!(
                safe_mode = self.safe_mode.is_safe_mode(),
                chat_id = %chat_id,
                session_id = %sid,
                source = %msg.source.clone().unwrap_or_default(),
                thread_id = %msg.thread_id.clone().unwrap_or_default(),
                trace_id = %request_id.clone().unwrap_or_else(|| "<none>".into()),
                "user message saved"
            );
        }

        let t0 = Instant::now();

        let response = node
            .chat(chat_id, sid_effective.clone(), message, storage)
            .await;

        metrics::histogram!("chat_response_time_ms")
            .record((t0.elapsed().as_micros() as f64) / 1000.0);

        if let Some(req_id) = &request_id {
            let key = format!(
                "{}|{}|{}",
                chat_id,
                sid_effective.clone().unwrap_or_else(|| "<none>".into()),
                req_id
            );
            self.requests
                .write()
                .unwrap()
                .put(key.clone(), response.clone());
            if let Some(store) = &self.idem {
                store.put(&key, &response);
            }
        }

        // Metrics for response
        // metrics could be recorded here via `metrics` crate

        tracing::info!(rate_limit=self.rate_limit_per_min, rate_remaining=%remaining, "chat rate updated");
        Ok(ChatOutput {
            response,
            session_id: sid_effective,
            idempotent: false,
        })
    }

    pub async fn analyze(
        &self,
        id: &str,
        input: &str,
        auth: &str,
        cancel_token: &CancellationToken,
    ) -> Option<AnalysisResult> {
        metrics::counter!("analysis_requests_total").increment(1);
        if !self.authorize(auth) {
            metrics::counter!("analysis_errors_total").increment(1);
            return None;
        }

        let triggers = self.trigger_detector.detect(input);
        for node in self.registry.action_nodes() {
            node.preload(&triggers, &self.memory);
        }

        let priority = self.memory.get_priority(id);
        let avg_time = self.memory.average_time_ms(id).unwrap_or(0);
        let queue = if avg_time < 100 {
            Queue::Fast
        } else if avg_time < 1000 {
            Queue::Standard
        } else {
            Queue::Long
        };
        self.scheduler.write().unwrap().enqueue(
            queue,
            id.to_string(),
            input.to_string(),
            priority,
            None,
            vec![id.to_string()],
        );

        let (task_id, task_input) = self.scheduler.write().unwrap().next()?;
        let node = self.registry.get_analysis_node(&task_id)?;
        let cancel = cancel_token.clone();

        let handle = spawn_blocking(move || node.analyze(&task_input, &cancel));

        let start = Instant::now();
        let checkpoint_mem = self.memory.clone();
        let checkpoint_id = id.to_string();
        let checkpoint_cancel = cancel_token.clone();
        let cfg = self.scheduler.read().unwrap().config.clone();
        let mut interval_timer = interval(Duration::from_millis(cfg.checkpoint_interval_ms));
        tokio::spawn(async move {
            loop {
                interval_timer.tick().await;
                if checkpoint_cancel.is_cancelled() {
                    break;
                }
                let r = AnalysisResult::new(&checkpoint_id, "", vec![]);
                checkpoint_mem.save_checkpoint(&checkpoint_id, &r);
            }
        });

        tokio::select! {
            _ = sleep(Duration::from_millis(cfg.global_time_budget)) => {
                cancel_token.cancel();
                let mut r = AnalysisResult::new(id, "", vec![]);
                r.status = NodeStatus::Error;
                self.memory.save_checkpoint(id, &r);
                metrics::counter!("analysis_errors_total").increment(1);
                info!("analysis {} timed out", id);
                Some(r)
            }
            _ = cancel_token.cancelled() => {
                let mut r = AnalysisResult::new(id, "", vec![]);
                r.status = NodeStatus::Error;
                self.memory.save_checkpoint(id, &r);
                metrics::counter!("analysis_errors_total").increment(1);
                info!("analysis {} cancelled", id);
                Some(r)
            }
            res = handle => {
                if let Ok(result) = res {
                    let elapsed = start.elapsed().as_millis();
                    if result.status == NodeStatus::Error {
                        metrics::counter!("analysis_errors_total").increment(1);
                        self.memory.save_checkpoint(id, &result);
                    } else {
                        self.memory.push_metrics(&result);
                        self.metrics.record(MetricsRecord {
                            id: result.id.clone(),
                            metrics: result.quality_metrics.clone(),
                        });
                        self.memory.update_time(id, elapsed);
                        let mem = self.memory.clone();
                        let rid = id.to_string();
                        mem.recalc_priority_async(rid);
                    }
                    metrics::histogram!("analysis_node_request_duration_ms")
                        .record(elapsed as f64);
                    metrics::histogram!("analysis_node_request_duration_ms_p95")
                        .record(elapsed as f64);
                    metrics::histogram!("analysis_node_request_duration_ms_p99")
                        .record(elapsed as f64);
                    info!(analysis_id=%id, duration_ms=elapsed, "analysis completed");
                    Some(result)
                } else {
                    metrics::counter!("analysis_errors_total").increment(1);
                    None
                }
            }
        }
    }

    pub fn resume(&self, id: &str, auth: &str) -> Option<AnalysisResult> {
        if !self.authorize(auth) {
            return None;
        }
        self.memory.load_checkpoint(id)
    }

    pub fn rate_info(
        &self,
        auth: &str,
        chat_id: &str,
        session_id: Option<&str>,
    ) -> (u32, u32, u32, String) {
        let key = match self.rate_key_mode {
            RateKeyMode::Auth => format!("auth:{}", auth),
            RateKeyMode::Chat => format!("chat:{}", chat_id),
            RateKeyMode::Session => match session_id {
                Some(s) => format!("session:{}:{}", chat_id, s),
                None => format!("chat:{}", chat_id),
            },
        };
        let now_min = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs())
            / 60;
        let used = if let Some((minute, count)) = self.rate.read().unwrap().get(&key) {
            if *minute == now_min { *count } else { 0 }
        } else { 0 } as u32;
        let limit = self.rate_limit_per_min;
        let remaining = limit.saturating_sub(used);
        (limit, remaining, used, key)
    }

    // SSE cancellation registry
    pub fn register_stream_cancel(
        &self,
        chat_id: &str,
        session_id: &str,
        token: tokio_util::sync::CancellationToken,
    ) {
        self.cancels
            .write()
            .unwrap()
            .insert((chat_id.to_string(), session_id.to_string()), token);
    }

    pub fn cancel_stream(&self, chat_id: &str, session_id: &str) -> bool {
        if let Some(tok) = self
            .cancels
            .write()
            .unwrap()
            .remove(&(chat_id.to_string(), session_id.to_string()))
        {
            tok.cancel();
            return true;
        }
        false
    }
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);
/* neira:meta
id: NEI-20250829-setup-meta-hub
intent: docs
scope: backend/hub
summary: |
  Политики чата: скоупы read/write/admin, idempotency (LRU+file TTL), rate-limit с ключом,
  safe-mode (write=admin), сохранение входящего user-сообщения с метаданными (source/thread_id).
links:
  - docs/backend-api.md
  - docs/reference/env.md
  - docs/reference/metrics.md
env:
  - CHAT_RATE_LIMIT_PER_MIN
  - CHAT_RATE_KEY
  - IDEMPOTENT_PERSIST
  - IDEMPOTENT_STORE_DIR
  - IDEMPOTENT_TTL_SECS
  - PERSIST_REQUIRE_SESSION_ID
metrics:
  - chat_requests_total
  - chat_errors_total
  - chat_response_time_ms
  - safe_mode
  - sessions_autocreated_total
  - requests_idempotent_hits
risks: low
safe_mode:
  affects_write: true
  requires_admin: true
i18n:
  reviewer_note: |
    Центр координации политик и лимитов. Следить за скоупами и idempotency.
*/
