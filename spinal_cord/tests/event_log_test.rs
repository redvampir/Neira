/* neira:meta
id: NEI-20270310-120400-event-log-tests
intent: chore
summary: Проверка записи и выборки событий из EventLog.
*/
use backend::event_bus::{Event, EventBus, OrganBuilt};
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
    env::remove_var("EVENT_LOG_ROTATE_SIZE");
    event_log::reset();
    let bus = EventBus::new();
    bus.publish(&OrganBuilt { id: "one".into() });
    bus.publish(&OrganBuilt { id: "two".into() });
    event_log::flush();
    let events = event_log::query(Some(2), Some(2), None, None, None);
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
    event_log::flush();
    let events = event_log::query(None, None, Some(ts), None, None);
    assert_eq!(events.len(), 1);
    assert!(events[0].ts_ms >= ts);
}

/* neira:meta
id: NEI-20270408-000000-rotate-unique
intent: test
summary: Проверка двух последовательных ротаций и уникальности имён gzip-файлов.
*/
#[test]
#[serial]
fn rotates_twice_with_unique_names() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("events.ndjson");
    env::set_var("EVENT_LOG_PATH", &file);
    env::set_var("EVENT_LOG_ROTATE_SIZE", "1");
    event_log::reset();
    let bus = EventBus::new();
    for id in ["a", "b", "c"] {
        bus.publish(&OrganBuilt { id: id.into() });
    }
    event_log::flush();
    let mut gz_files: Vec<_> = fs::read_dir(dir.path())
        .unwrap()
        .filter_map(|e| {
            let p = e.ok()?.path();
            if p.extension().map(|s| s == "gz").unwrap_or(false) {
                Some(p.file_name().unwrap().to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();
    gz_files.sort();
    assert_eq!(gz_files.len(), 2, "expected two rotated files");
    assert_ne!(gz_files[0], gz_files[1], "rotated file names must differ");
}

/* neira:meta
id: NEI-20270501-event-log-name-filter-test
intent: test
summary: Проверка фильтрации событий по имени.
*/
#[test]
#[serial]
fn filter_by_name() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("events.ndjson");
    env::set_var("EVENT_LOG_FILE", &file);
    event_log::reset();
    let bus = EventBus::new();

    struct Foo;
    impl Event for Foo {
        fn name(&self) -> &str {
            "Foo"
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    bus.publish(&Foo);
    bus.publish(&OrganBuilt { id: "x".into() });
    event_log::flush();
    let events = event_log::query(None, None, None, None, Some("Foo"));
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].name, "Foo");
}
