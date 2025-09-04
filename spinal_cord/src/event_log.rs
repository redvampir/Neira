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
use crate::event_bus::Event;
use chrono::Utc;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{copy, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggedEvent {
    pub id: u64,
    pub ts_ms: i64,
    pub name: String,
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

fn rotate_if_needed(path: &Path) {
    let limit = rotate_limit();
    if let Ok(meta) = fs::metadata(path) {
        if meta.len() >= limit {
            let ts = Utc::now().format("%Y%m%d%H%M%S");
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("events");
            let rotated = path.with_file_name(format!("{}-{}.ndjson.gz", stem, ts));
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

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn append(event: &dyn Event) {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    let entry = LoggedEvent {
        id,
        ts_ms: Utc::now().timestamp_millis(),
        name: event.name().to_string(),
    };
    if let Ok(line) = serde_json::to_string(&entry) {
        let path = log_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        rotate_if_needed(&path);
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(file, "{}", line);
        }
    }
}

pub fn query(
    start_id: Option<u64>,
    end_id: Option<u64>,
    start_ts_ms: Option<i64>,
    end_ts_ms: Option<i64>,
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
            true
        })
        .collect()
}

pub fn reset() {
    COUNTER.store(0, Ordering::SeqCst);
    let path = log_path();
    let _ = fs::remove_file(path);
}
