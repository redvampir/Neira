/* neira:meta
id: NEI-20270618-000000-lymphatic-filter-tests
intent: feature
summary: Юнит-тесты лимфатического фильтра на поиск дубликатов и работу флага.
*/
use backend::event_bus::{CellCreated, Event, EventBus, LymphaticDuplicateFound, Subscriber};
use backend::factory::{StemCellRecord, StemCellState};
use backend::immune_system::{lymphatic_filter, ImmuneSystemSubscriber};
use chrono::Utc;
use serial_test::serial;
use std::sync::{Arc, Mutex};

struct Capture {
    count: Mutex<usize>,
}

impl Subscriber for Capture {
    fn on_event(&self, ev: &dyn Event) {
        if ev
            .as_any()
            .downcast_ref::<LymphaticDuplicateFound>()
            .is_some()
        {
            *self.count.lock().unwrap() += 1;
        }
    }
}

#[test]
fn detects_identical_functions() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.rs"), "/// add\nfn foo() -> i32 {1}\n").unwrap();
    std::fs::write(dir.path().join("b.rs"), "/// add\nfn foo() -> i32 {1}\n").unwrap();
    let prev = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    let reports = lymphatic_filter::scan_workspace();
    std::env::set_current_dir(prev).unwrap();
    assert!(!reports.is_empty());
    assert!(reports.iter().any(|r| r.gene_id == "foo"));
}

#[test]
fn ignores_low_semantic_similarity() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.rs"), "/// add\nfn calc() -> i32 {1}\n").unwrap();
    std::fs::write(
        dir.path().join("b.rs"),
        "/// subtract\nfn calc() -> i32 {1}\n",
    )
    .unwrap();
    let prev = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    let reports = lymphatic_filter::scan_workspace();
    std::env::set_current_dir(prev).unwrap();
    assert!(reports.is_empty());
}

#[test]
#[serial]
fn respects_env_flag() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.rs"), "fn foo() -> i32 {1}\n").unwrap();
    std::fs::write(dir.path().join("b.rs"), "fn foo() -> i32 {1}\n").unwrap();
    let prev = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();

    std::env::set_var("LYMPHATIC_FILTER_ENABLED", "false");
    let bus = EventBus::new();
    let cap = Arc::new(Capture {
        count: Mutex::new(0),
    });
    bus.subscribe(cap.clone());
    bus.subscribe(Arc::new(ImmuneSystemSubscriber::new(bus.clone())));

    let record = StemCellRecord {
        id: "1".into(),
        backend: "b".into(),
        template_id: "t".into(),
        state: StemCellState::Draft,
        created_at: Utc::now(),
    };
    bus.publish(&CellCreated { record });

    std::env::set_current_dir(prev).unwrap();
    std::env::remove_var("LYMPHATIC_FILTER_ENABLED");
    assert_eq!(*cap.count.lock().unwrap(), 0);
}

#[test]
fn scan_dir_and_ignore_and_patch() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.rs"), "fn foo() {}\n").unwrap();
    std::fs::write(dir.path().join("b.rs"), "fn foo() {}\n").unwrap();
    std::fs::create_dir(dir.path().join("ignored")).unwrap();
    std::fs::write(dir.path().join("ignored/c.rs"), "fn foo() {}\n").unwrap();
    std::fs::write(dir.path().join(".lymphaticignore"), "ignored\n").unwrap();
    std::env::set_var("LYMPHATIC_SCAN_DIR", dir.path());
    let reports = lymphatic_filter::scan_workspace();
    std::env::remove_var("LYMPHATIC_SCAN_DIR");
    assert_eq!(reports.len(), 1);
    assert!(reports[0]
        .patch
        .as_ref()
        .map(|p| p.exists())
        .unwrap_or(false));
}
