/* neira:meta
id: NEI-20270610-120200-lymphatic-event-test
intent: chore
summary: Проверяет публикацию lymphatic_filter.activated и реакцию подписчика.
*/
use backend::event_bus::{
    Event, EventBus, LymphaticDecision, LymphaticFilterActivated, Subscriber,
};
use backend::event_log;
use serial_test::serial;
use std::sync::{Arc, Mutex};

struct ImmuneSystem {
    last: Mutex<Option<LymphaticDecision>>,
}

impl Subscriber for ImmuneSystem {
    fn on_event(&self, ev: &dyn Event) {
        if let Some(ev) = ev.as_any().downcast_ref::<LymphaticFilterActivated>() {
            *self.last.lock().unwrap() = Some(ev.decision);
        }
    }
}

#[test]
#[serial]
fn publishes_and_logs_event() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("events.ndjson");
    std::env::set_var("EVENT_LOG_FILE", &file);
    event_log::reset();
    let bus = EventBus::new();
    let immune = Arc::new(ImmuneSystem {
        last: Mutex::new(None),
    });
    bus.subscribe(immune.clone());
    let ev = LymphaticFilterActivated {
        function_id: "f1".into(),
        similarity: 0.91,
        decision: LymphaticDecision::Remove,
    };
    bus.publish(&ev);
    event_log::flush();
    assert_eq!(
        *immune.last.lock().unwrap(),
        Some(LymphaticDecision::Remove)
    );
    let events = event_log::query(None, None, None, None, Some("lymphatic_filter.activated"));
    assert_eq!(events.len(), 1);
    let data = events[0].data.as_ref().unwrap();
    assert_eq!(data["function_id"], "f1");
    assert_eq!(data["decision"], "remove");
}
