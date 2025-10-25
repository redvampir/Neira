/* neira:meta
id: NEI-20280401-120040-curriculum-test
intent: feature
summary: Проверяет загрузку курса русской грамоты: память, событие и данные.
*/
use std::sync::{Arc, Mutex};

use backend::action::diagnostics_cell::DiagnosticsCell;
use backend::action::metrics_collector_cell::MetricsCollectorCell;
use backend::cell_registry::CellRegistry;
use backend::config::Config;
use backend::digestive_pipeline::ParsedInput;
use backend::event_bus::{Event, Subscriber};
use backend::memory_cell::MemoryCell;
use backend::training::curriculum::RUSSIAN_CURRICULUM_ID;
use backend::synapse_hub::SynapseHub;

struct CaptureSubscriber {
    events: Arc<Mutex<Vec<serde_json::Value>>>,
}

impl Subscriber for CaptureSubscriber {
    fn on_event(&self, event: &dyn Event) {
        if event.name() == "training.curriculum.loaded" {
            if let Some(data) = event.data() {
                if let Ok(mut guard) = self.events.lock() {
                    guard.push(data);
                }
            }
        }
    }
}

#[tokio::test]
async fn literacy_curriculum_is_loaded_into_memory_and_event_bus() {
    let templates_dir = tempfile::tempdir().expect("templates dir");
    let registry = Arc::new(CellRegistry::new(templates_dir.path()).expect("registry"));
    let memory = Arc::new(MemoryCell::new());
    let (metrics, rx) = MetricsCollectorCell::channel();
    let (diagnostics, _dev_rx, _alert_rx) = DiagnosticsCell::new(rx, 8, metrics.clone());
    let cfg = Config::default();
    let hub = SynapseHub::new(registry, memory.clone(), metrics, diagnostics, &cfg);

    let captured = Arc::new(Mutex::new(Vec::new()));
    hub.subscribe_event(Arc::new(CaptureSubscriber {
        events: captured.clone(),
    }));

    let curriculum = hub
        .train_russian_literacy(None)
        .expect("curriculum loaded");

    assert_eq!(curriculum.id(), RUSSIAN_CURRICULUM_ID);
    assert_eq!(curriculum.words.len(), 100);
    assert_eq!(curriculum.summary().letters, 33);

    let parsed = memory.parsed_inputs();
    let last = parsed.last().expect("parsed input stored");
    match last {
        ParsedInput::Json(value) => {
            assert_eq!(value.get("id"), Some(&serde_json::Value::String(RUSSIAN_CURRICULUM_ID.into())));
        }
        ParsedInput::Text(_) => panic!("expected json payload"),
    }

    let events = captured.lock().expect("event lock");
    assert_eq!(events.len(), 1);
    let data = &events[0];
    assert_eq!(data.get("curriculum_id"), Some(&serde_json::Value::String(RUSSIAN_CURRICULUM_ID.into())));
    assert_eq!(data.get("letters"), Some(&serde_json::Value::from(33)));  
    assert_eq!(data.get("words"), Some(&serde_json::Value::from(100)));
}

