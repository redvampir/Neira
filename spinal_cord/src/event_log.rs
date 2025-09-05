/* neira:meta
id: NEI-20270310-120000-event-log
intent: feature
summary: |-
  Запись событий EventBus в файл NDJSON и выборка по диапазону.
*/
/* neira:meta
id: NEI-20270310-rotating-log
intent: feature
summary: |-
  Добавлена ротация журнала с gzip‑сжатием и настройкой пути через переменные окружения.
*/
/* neira:meta
id: NEI-20270501-event-log-async
intent: refactor
summary: Запись событий через асинхронный канал и поддержка flush().
*/
/* neira:meta
id: NEI-20270501-event-log-name-filter
intent: feature
summary: query фильтрует события по имени.
*/
/* neira:meta
id: NEI-20270610-120100-event-log-payload
intent: feature
summary: LoggedEvent хранит произвольные данные события в поле data.
*/
use crate::event_bus::Event;
use chrono::Utc;
use flate2::write::GzEncoder;
use flate2::Compression;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{self, File, OpenOptions};
use std::io::{copy, Error as IoError, ErrorKind, Result as IoResult, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;
use tokio::sync::broadcast;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggedEvent {
    pub id: u64,
    pub ts_ms: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

fn log_path() -> PathBuf {
    std::env::var("EVENT_LOG_PATH")
        .or_else(|_| std::env::var("EVENT_LOG_FILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("logs/events.ndjson"))
}

fn rotate_limit() -> u64 {
    std::env::var("EVENT_LOG_ROTATE_SIZE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5 * 1024 * 1024)
}

/* neira:meta
id: NEI-20270408-000000-rotate-seq
intent: fix
summary: |-
  Ротация журнала использует метку времени в миллисекундах
  и последовательный счётчик в имени файла.
*/
/// Сохраняет архив `events.ndjson` в формате
/// `{stem}-{timestamp_ms}-{seq}.ndjson.gz` при превышении лимита.
fn rotate_if_needed(path: &Path) {
    let limit = rotate_limit();
    if let Ok(meta) = fs::metadata(path) {
        if meta.len() >= limit {
            let ts = Utc::now().timestamp_millis();
            let seq = ROTATE_SEQ.fetch_add(1, Ordering::SeqCst) + 1;
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("events");
            let rotated = path.with_file_name(format!("{stem}-{ts}-{seq}.ndjson.gz"));
            if let Ok(mut input) = File::open(path) {
                if let Ok(out) = File::create(&rotated) {
                    let mut enc = GzEncoder::new(out, Compression::default());
                    let _ = copy(&mut input, &mut enc);
                    let _ = enc.finish();
                }
            }
            let _ = fs::remove_file(path);
        }
    }
}

static ROTATE_SEQ: AtomicU64 = AtomicU64::new(0);
static COUNTER: AtomicU64 = AtomicU64::new(0);
static PENDING: AtomicU64 = AtomicU64::new(0);

static SENDER: Lazy<Sender<LoggedEvent>> = Lazy::new(|| {
    let (tx, rx) = mpsc::channel::<LoggedEvent>();
    thread::spawn(move || {
        for entry in rx {
            let path = log_path();
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            rotate_if_needed(&path);
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) {
                if let Ok(line) = serde_json::to_string(&entry) {
                    let _ = writeln!(file, "{}", line);
                }
            }
            let _ = BROADCAST.send(entry.clone());
            PENDING.fetch_sub(1, Ordering::SeqCst);
        }
    });
    tx
});

/* neira:meta
id: NEI-20270505-event-log-broadcast
intent: feature
summary: |-
  Подписчики получают новые события через broadcast-канал.
*/
static BROADCAST: Lazy<broadcast::Sender<LoggedEvent>> = Lazy::new(|| {
    let cap = std::env::var("EVENT_LOG_BROADCAST_CAPACITY")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1024);
    let (tx, _rx) = broadcast::channel(cap);
    tx
});

pub fn subscribe() -> broadcast::Receiver<LoggedEvent> {
    BROADCAST.subscribe()
}

pub fn append(event: &dyn Event) -> IoResult<()> {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    let entry = LoggedEvent {
        id,
        ts_ms: Utc::now().timestamp_millis(),
        name: event.name().to_string(),
        data: event.data(),
    };
    PENDING.fetch_add(1, Ordering::SeqCst);
    SENDER.send(entry).map_err(|e| {
        PENDING.fetch_sub(1, Ordering::SeqCst);
        IoError::new(ErrorKind::Other, e)
    })
}

pub fn query(
    start_id: Option<u64>,
    end_id: Option<u64>,
    start_ts_ms: Option<i64>,
    end_ts_ms: Option<i64>,
    name: Option<&str>,
) -> Vec<LoggedEvent> {
    let path = log_path();
    let Ok(data) = fs::read_to_string(path) else {
        return Vec::new();
    };
    data.lines()
        .filter_map(|ln| serde_json::from_str::<LoggedEvent>(ln).ok())
        .filter(|ev| {
            if let Some(s) = start_id {
                if ev.id < s {
                    return false;
                }
            }
            if let Some(e) = end_id {
                if ev.id > e {
                    return false;
                }
            }
            if let Some(s) = start_ts_ms {
                if ev.ts_ms < s {
                    return false;
                }
            }
            if let Some(e) = end_ts_ms {
                if ev.ts_ms > e {
                    return false;
                }
            }
            if let Some(n) = name {
                if ev.name != n {
                    return false;
                }
            }
            true
        })
        .collect()
}

pub fn reset() {
    flush();
    COUNTER.store(0, Ordering::SeqCst);
    let path = log_path();
    let _ = fs::remove_file(path);
}

pub fn flush() {
    while PENDING.load(Ordering::SeqCst) > 0 {
        thread::sleep(Duration::from_millis(1));
    }
}
