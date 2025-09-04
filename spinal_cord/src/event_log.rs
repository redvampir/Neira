/* neira:meta
id: NEI-20270310-120000-event-log
intent: feature
summary: |-
  Запись событий EventBus в файл NDJSON и выборка по диапазону.
*/
use crate::event_bus::Event;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggedEvent {
    pub id: u64,
    pub ts_ms: i64,
    pub name: String,
}

fn log_path() -> PathBuf {
    std::env::var("EVENT_LOG_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("logs/events.ndjson"))
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
