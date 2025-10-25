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

#[derive(Clone, Debug)]
struct CapturedEvent {
    name: String,
    data: serde_json::Value,
}

struct CaptureSubscriber {
    events: Arc<Mutex<Vec<CapturedEvent>>>,
}

impl Subscriber for CaptureSubscriber {
    fn on_event(&self, event: &dyn Event) {
        if let Some(data) = event.data() {
            if let Ok(mut guard) = self.events.lock() {
                guard.push(CapturedEvent {
                    name: event.name().to_string(),
                    data,
                });
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
    assert!(parsed.len() >= 2, "ожидаем курс и словарь в памяти");
    let curriculum_payload = parsed.iter().find_map(|input| match input {
        ParsedInput::Json(value)
            if value.get("id")
                == Some(&serde_json::Value::String(RUSSIAN_CURRICULUM_ID.into())) =>
        {
            Some(value)
        }
        _ => None,
    })
    .expect("curriculum payload stored");
    assert_eq!(
        curriculum_payload.get("language"),
        Some(&serde_json::Value::String(String::from("ru")))
    );

    let seed_payload = parsed
        .iter()
        .find_map(|input| match input {
            ParsedInput::Json(value)
                if value.get("purpose")
                    == Some(&serde_json::Value::String(String::from("inquiry_vocabulary"))) =>
            {
                Some(value)
            }
            _ => None,
        })
        .expect("inquiry vocabulary stored");
    let words_array = seed_payload
        .get("words")
        .and_then(|value| value.as_array())
        .expect("seed words array");
    assert!(
        (10..=30).contains(&words_array.len()),
        "ожидаем от 10 до 30 слов"
    );
    assert!(words_array.iter().any(|entry| {
        entry
            .get("word")
            .and_then(|value| value.as_str())
            .map(|word| word == "мама")
            .unwrap_or(false)
    }));

    let events = captured.lock().expect("event lock");
    assert_eq!(events.len(), 2);
    let loaded = events
        .iter()
        .find(|event| event.name == "training.curriculum.loaded")
        .expect("loaded event");
    assert_eq!(
        loaded
            .data
            .get("curriculum_id")
            .and_then(|value| value.as_str()),
        Some(RUSSIAN_CURRICULUM_ID)
    );
    assert_eq!(
        loaded.data.get("letters").and_then(|value| value.as_u64()),
        Some(33)
    );
    assert_eq!(
        loaded.data.get("words").and_then(|value| value.as_u64()),
        Some(100)
    );

    let seeded = events
        .iter()
        .find(|event| event.name == "training.curriculum.vocabulary_seeded")
        .expect("seeded event");
    assert_eq!(
        seeded
            .data
            .get("purpose")
            .and_then(|value| value.as_str()),
        Some("inquiry_vocabulary")
    );
    let seeded_words = seeded
        .data
        .get("words")
        .and_then(|value| value.as_array())
        .expect("words list in event");
    assert!(
        (10..=30).contains(&seeded_words.len()),
        "event word count mismatch"
    );
}

