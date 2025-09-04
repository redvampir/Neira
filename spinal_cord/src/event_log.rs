/* neira:meta
id: NEI-20270310-120000-event-log
intent: feature
summary: |-
  Запись событий EventBus в файл NDJSON с ротацией, метриками и фильтрацией.
*/
use crate::event_bus::Event;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use metrics::{counter, histogram};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggedEvent {
    pub id: u64,
    pub ts_ms: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

fn log_path() -> PathBuf {
    std::env::var("EVENT_LOG_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("logs/events.ndjson"))
}

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn init_counter() {
    if COUNTER.load(Ordering::SeqCst) != 0 {
        return;
    }
    let path = log_path();
    let last_id = fs::File::open(&path)
        .ok()
        .and_then(|f| {
            BufReader::new(f)
                .lines()
                .filter_map(|ln| serde_json::from_str::<LoggedEvent>(&ln.ok()?).ok())
                .map(|ev| ev.id)
                .last()
        })
        .unwrap_or(0);
    let _ = COUNTER.compare_exchange(0, last_id, Ordering::SeqCst, Ordering::SeqCst);
}

fn rotate_if_needed(path: &Path) {
    let max = std::env::var("EVENT_LOG_MAX_SIZE")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(1_048_576); // 1MB по умолчанию
    if let Ok(meta) = fs::metadata(path) {
        if meta.len() > max {
            let ts = Utc::now().format("%Y%m%d%H%M%S");
            let rotated = path.with_file_name(format!("events-{}.ndjson", ts));
            let _ = fs::rename(path, rotated);
        }
    }
}

pub fn append(event: &dyn Event) {
    init_counter();
    let start = Instant::now();
    let id = COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    let entry = LoggedEvent {
        id,
        ts_ms: Utc::now().timestamp_millis(),
        name: event.name().to_string(),
        data: event.to_json(),
    };
    if let Ok(line) = serde_json::to_string(&entry) {
        let path = log_path();
        rotate_if_needed(&path);
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) {
            let _ = writeln!(file, "{}", line);
            let name_label = event.name().to_string();
            counter!("event_log_appended_total", "name" => name_label).increment(1);
            histogram!("event_log_append_ms").record(start.elapsed().as_millis() as f64);
        }
    }
}

pub fn query(
    start_id: Option<u64>,
    end_id: Option<u64>,
    start_ts_ms: Option<i64>,
    end_ts_ms: Option<i64>,
    names: Option<&[String]>,
) -> Vec<LoggedEvent> {
    let path = log_path();
    let Ok(data) = fs::read_to_string(path) else {
        return Vec::new();
    };
    let events: Vec<LoggedEvent> = data
        .lines()
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
            if let Some(names) = names {
                if !names.iter().any(|n| n == &ev.name) {
                    return false;
                }
            }
            true
        })
        .collect();
    counter!("event_log_queries_total").increment(events.len() as u64);
    events
}

pub fn reset() {
    COUNTER.store(0, Ordering::SeqCst);
    let path = log_path();
    let _ = fs::remove_file(path);
}

pub fn reset_counter_only() {
    COUNTER.store(0, Ordering::SeqCst);
}
