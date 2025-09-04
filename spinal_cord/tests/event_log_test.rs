/* neira:meta
id: NEI-20270310-120400-event-log-tests
intent: chore
summary: Проверка записи и выборки событий из EventLog.
*/
use backend::event_bus::{Event, EventBus, OrganBuilt};
use backend::event_log;
use serial_test::serial;
use std::env;
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
    let events = event_log::query(None, None, Some(ts), None, None);
    assert_eq!(events.len(), 1);
    assert!(events[0].ts_ms >= ts);
}

#[derive(serde::Serialize)]
struct TestEvent {
    name: &'static str,
    value: u32,
}

impl Event for TestEvent {
    fn name(&self) -> &str {
        self.name
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn to_json(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self).ok()
    }
}

#[test]
#[serial]
fn filter_by_name_and_persist_ids() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("events.ndjson");
    env::set_var("EVENT_LOG_FILE", &file);
    env::set_var("EVENT_LOG_MAX_SIZE", "100000");
    event_log::reset();
    let bus = EventBus::new();
    bus.publish(&OrganBuilt { id: "x".into() });
    bus.publish(&TestEvent {
        name: "TestEvent",
        value: 42,
    });
    let filtered = event_log::query(None, None, None, None, Some(&vec!["TestEvent".into()]));
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name, "TestEvent");
    assert!(filtered[0].data.is_some());

    // имитируем перезапуск: сбрасываем счётчик, но оставляем файл
    event_log::reset_counter_only();
    bus.publish(&OrganBuilt { id: "y".into() });
    let events = event_log::query(None, None, None, None, None);
    assert_eq!(events.last().unwrap().id, 3);
}
