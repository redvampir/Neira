/* neira:meta
id: NEI-20280502-120800-interaction-verbs-test
intent: chore
summary: Проверяет детектор глаголов взаимодействия и публикацию события SynapseHub.
*/

use std::sync::{Arc, Mutex};

use backend::action::chat_cell::EchoChatCell;
use backend::action::diagnostics_cell::DiagnosticsCell;
use backend::action::metrics_collector_cell::MetricsCollectorCell;
use backend::cell_registry::CellRegistry;
use backend::config::Config;
use backend::context::context_storage::FileContextStorage;
use backend::event_bus::{Event, Subscriber};
use backend::memory_cell::MemoryCell;
use backend::persona::interaction_verbs::{
    InteractionVerb, InteractionVerbActor, InteractionVerbDetector,
};
use backend::synapse_hub::SynapseHub;
use serde_json::Value;

mod common;
use common::init_recorder;

struct CaptureSubscriber {
    events: Arc<Mutex<Vec<Value>>>,
}

impl Subscriber for CaptureSubscriber {
    fn on_event(&self, event: &dyn Event) {
        if event.name() == "persona.interaction_verb.observed" {
            if let Some(data) = event.data() {
                self.events.lock().unwrap().push(data);
            }
        }
    }
}

#[tokio::test]
async fn detector_recognizes_variants_and_synapse_hub_emits_event() {
    let detector = InteractionVerbDetector::default();
    let verbs = detector.detect("Дайте, пожалуйста, объясните и повторите примеры.");
    assert!(verbs.contains(&InteractionVerb::Give));
    assert!(verbs.contains(&InteractionVerb::Explain));
    assert!(verbs.contains(&InteractionVerb::Repeat));

    let metrics_data = init_recorder();

    let templates_dir = tempfile::tempdir().unwrap();
    let registry = Arc::new(CellRegistry::new(templates_dir.path()).expect("registry"));
    let memory = Arc::new(MemoryCell::new());
    let (metrics, rx) = MetricsCollectorCell::channel();
    let (diagnostics, _dev_rx, _alert_rx) = DiagnosticsCell::new(rx, 5, metrics.clone());
    let cfg = Config::default();
    let hub = SynapseHub::new(registry.clone(), memory, metrics, diagnostics, &cfg);
    hub.add_auth_token("secret");
    registry.register_chat_cell(Arc::new(EchoChatCell::default()));

    let storage_dir = tempfile::tempdir().unwrap();
    let storage = FileContextStorage::new(storage_dir.path().join("context"));

    let events = Arc::new(Mutex::new(Vec::new()));
    hub.subscribe_event(Arc::new(CaptureSubscriber {
        events: events.clone(),
    }));

    let phrase = "Покажи, пожалуйста, список и найди последние записи.";

    let response = hub
        .chat(
            "echo.chat",
            "chat1",
            Some("session1".to_string()),
            phrase,
            &storage,
            "secret",
            true,
            None,
            None,
            None,
        )
        .await
        .expect("chat success");

    assert_eq!(response.response, phrase);

    let recorded = events.lock().unwrap();
    assert!(!recorded.is_empty(), "expected interaction verb event");
    let show_event = recorded
        .iter()
        .find(|value| value["verb"] == "show")
        .expect("show verb recorded");
    assert_eq!(show_event["actor"], InteractionVerbActor::User.as_str());
    assert_eq!(show_event["chat_id"], "chat1");
    drop(recorded);

    let metrics_snapshot = metrics_data.lock().unwrap();
    assert!(metrics_snapshot
        .iter()
        .any(|(name, _)| name == "interaction_verbs_detected_total"));
}
