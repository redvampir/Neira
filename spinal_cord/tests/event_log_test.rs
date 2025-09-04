/* neira:meta
id: NEI-20270310-120400-event-log-tests
intent: chore
summary: Проверка записи и выборки событий из EventLog.
*/
use backend::event_bus::{EventBus, OrganBuilt};
use backend::event_log;
use serial_test::serial;
use std::env;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
use tempfile::tempdir;

#[test]
#[serial]
fn publish_and_query_by_id() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("events.ndjson");
    env::set_var("EVENT_LOG_FILE", &file);
    event_log::reset();
    let bus = EventBus::new();
    bus.publish(&OrganBuilt { id: "one".into() });
    bus.publish(&OrganBuilt { id: "two".into() });
    let events = event_log::query(Some(2), Some(2), None, None);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].name, "OrganBuilt");
    assert_eq!(events[0].id, 2);
}

#[test]
#[serial]
fn query_by_time_range() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("events.ndjson");
    env::set_var("EVENT_LOG_FILE", &file);
    event_log::reset();
    let bus = EventBus::new();
    bus.publish(&OrganBuilt { id: "a".into() });
    sleep(Duration::from_millis(2));
    let ts = chrono::Utc::now().timestamp_millis();
    bus.publish(&OrganBuilt { id: "b".into() });
    let events = event_log::query(None, None, Some(ts), None);
    assert_eq!(events.len(), 1);
    assert!(events[0].ts_ms >= ts);
}

/* neira:meta
id: NEI-20270310-rotate-test
intent: test
summary: Проверка ротации и gzip-сжатия EventLog.
*/
#[test]
#[serial]
fn rotates_and_compresses() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("events.ndjson");
    env::set_var("EVENT_LOG_PATH", &file);
    env::set_var("EVENT_LOG_ROTATE_SIZE", "200");
    event_log::reset();
    let bus = EventBus::new();
    for i in 0..50 {
        bus.publish(&OrganBuilt { id: format!("{i}") });
    }
    let has_gz = fs::read_dir(dir.path()).unwrap().any(|e| {
        e.unwrap()
            .path()
            .extension()
            .map(|s| s == "gz")
            .unwrap_or(false)
    });
    assert!(has_gz, "rotated gzip not found");
}
