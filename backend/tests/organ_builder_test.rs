/* neira:meta
id: NEI-20251010-organ-builder-test
intent: test
summary: Проверяет переходы стадий органа, ручное обновление, удержание статуса `Failed`,
  очистку шаблонов по TTL и восстановление счётчика идентификаторов при рестарте.
*/
use backend::organ_builder::{OrganBuilder, OrganState};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn organ_builder_progresses_and_updates() {
    std::env::set_var("ORGANS_BUILDER_ENABLED", "true");
    let dir = tempfile::tempdir().unwrap();
    std::env::set_var("ORGANS_BUILDER_TEMPLATES_DIR", dir.path());
    let builder = OrganBuilder::new();
    let id = builder.start_build(serde_json::json!({"kind": "test"}));
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_eq!(builder.status(&id), Some(OrganState::Stable));
    builder.update_status(&id, OrganState::Failed);
    assert_eq!(builder.status(&id), Some(OrganState::Failed));
    assert!(dir.path().join(format!("{id}.json")).exists());
    std::env::remove_var("ORGANS_BUILDER_ENABLED");
    std::env::remove_var("ORGANS_BUILDER_TEMPLATES_DIR");
}

#[tokio::test]
#[serial]
async fn organ_builder_failure_persists() {
    std::env::set_var("ORGANS_BUILDER_ENABLED", "true");
    let dir = tempfile::tempdir().unwrap();
    std::env::set_var("ORGANS_BUILDER_TEMPLATES_DIR", dir.path());
    let builder = OrganBuilder::new();
    let id = builder.start_build(serde_json::json!({"kind": "test"}));
    tokio::time::sleep(std::time::Duration::from_millis(120)).await;
    builder.update_status(&id, OrganState::Failed);
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_eq!(builder.status(&id), Some(OrganState::Failed));
    std::env::remove_var("ORGANS_BUILDER_ENABLED");
    std::env::remove_var("ORGANS_BUILDER_TEMPLATES_DIR");
}

#[tokio::test]
#[serial]
async fn organ_builder_removes_template_after_ttl() {
    std::env::set_var("ORGANS_BUILDER_ENABLED", "true");
    std::env::set_var("ORGANS_BUILDER_TTL_SECS", "1");
    let dir = tempfile::tempdir().unwrap();
    std::env::set_var("ORGANS_BUILDER_TEMPLATES_DIR", dir.path());
    let builder = OrganBuilder::new();
    let id = builder.start_build(serde_json::json!({"kind": "test"}));
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert!(dir.path().join(format!("{id}.json")).exists());
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    assert!(!dir.path().join(format!("{id}.json")).exists());
    assert_eq!(builder.status(&id), Some(OrganState::Stable));
    std::env::remove_var("ORGANS_BUILDER_ENABLED");
    std::env::remove_var("ORGANS_BUILDER_TEMPLATES_DIR");
    std::env::remove_var("ORGANS_BUILDER_TTL_SECS");
}

#[tokio::test]
#[serial]
async fn organ_builder_restores_statuses_from_disk() {
    std::env::set_var("ORGANS_BUILDER_ENABLED", "true");
    let dir = tempfile::tempdir().unwrap();
    std::env::set_var("ORGANS_BUILDER_TEMPLATES_DIR", dir.path());
    let builder = OrganBuilder::new();
    let id = builder.start_build(serde_json::json!({"kind": "test"}));
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    drop(builder);
    let builder = OrganBuilder::new();
    assert_eq!(builder.status(&id), Some(OrganState::Stable));
    std::env::remove_var("ORGANS_BUILDER_ENABLED");
    std::env::remove_var("ORGANS_BUILDER_TEMPLATES_DIR");
}

#[tokio::test]
#[serial]
async fn organ_builder_resumes_counter_from_disk() {
    std::env::set_var("ORGANS_BUILDER_ENABLED", "true");
    let dir = tempfile::tempdir().unwrap();
    std::env::set_var("ORGANS_BUILDER_TEMPLATES_DIR", dir.path());
    {
        let builder = OrganBuilder::new();
        let _ = builder.start_build(serde_json::json!({"kind": "one"}));
        let id2 = builder.start_build(serde_json::json!({"kind": "two"}));
        std::fs::rename(
            dir.path().join(format!("{id2}.json")),
            dir.path().join("organ-5.json"),
        )
        .unwrap();
    }
    let builder = OrganBuilder::new();
    let new_id = builder.start_build(serde_json::json!({"kind": "new"}));
    assert_eq!(new_id, "organ-6");
    std::env::remove_var("ORGANS_BUILDER_ENABLED");
    std::env::remove_var("ORGANS_BUILDER_TEMPLATES_DIR");
}

/* neira:meta
id: NEI-20250317-organ-builder-update-missing-test
intent: test
summary: records error metric when updating status for unknown organ.
*/
#[tokio::test]
#[serial]
async fn organ_builder_records_status_update_error() {
    std::env::set_var("ORGANS_BUILDER_ENABLED", "true");

    use metrics::{
        Counter, Gauge, Histogram, Key, KeyName, Metadata, Recorder, SharedString, Unit,
    };
    use std::sync::{Arc, Mutex};

    struct CounterRecorder {
        data: Arc<Mutex<Vec<(String, u64)>>>,
    }

    impl Recorder for CounterRecorder {
        fn describe_counter(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}
        fn describe_gauge(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}
        fn describe_histogram(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}

        fn register_counter(&self, key: &Key, _meta: &Metadata<'_>) -> Counter {
            let name = key.name().to_string();
            let data = self.data.clone();
            let ctr = TestCounter { name, data };
            Counter::from_arc(Arc::new(ctr))
        }

        fn register_gauge(&self, _key: &Key, _: &Metadata<'_>) -> Gauge {
            Gauge::noop()
        }

        fn register_histogram(&self, _key: &Key, _: &Metadata<'_>) -> Histogram {
            Histogram::noop()
        }
    }

    struct TestCounter {
        name: String,
        data: Arc<Mutex<Vec<(String, u64)>>>,
    }

    impl metrics::CounterFn for TestCounter {
        fn increment(&self, value: u64) {
            self.data.lock().unwrap().push((self.name.clone(), value));
        }
    }

    let data = Arc::new(Mutex::new(Vec::new()));
    let recorder = CounterRecorder { data: data.clone() };
    metrics::set_global_recorder(recorder).expect("set recorder");

    let builder = OrganBuilder::new();
    assert!(builder
        .update_status("missing", OrganState::Failed)
        .is_none());

    let records = data.lock().unwrap();
    assert!(records
        .iter()
        .any(|(n, v)| n == "organ_build_status_errors_total" && *v == 1));

    std::env::remove_var("ORGANS_BUILDER_ENABLED");
}
