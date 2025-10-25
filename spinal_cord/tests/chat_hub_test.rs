use std::sync::Arc;

use backend::action::chat_cell::EchoChatCell;
use backend::action::diagnostics_cell::DiagnosticsCell;
use backend::action::metrics_collector_cell::MetricsCollectorCell;
use backend::cell_registry::CellRegistry;
use backend::config::Config;
use backend::context::context_storage::FileContextStorage;
use backend::memory_cell::MemoryCell;
use backend::persona::tone_state::ToneMood;
use backend::synapse_hub::SynapseHub;

#[tokio::test]
async fn chat_hub_rejects_empty_message() {
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

    let res = hub
        .chat(
            "echo.chat",
            "test_chat",
            Some("sess1".to_string()),
            "   ",
            &storage,
            "secret",
            false,
            None,
            None,
            None,
        )
        .await;
    assert!(res.is_err());
}

#[tokio::test]
async fn chat_positive_message_updates_tone_state() {
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
    let phrase = "Спасибо огромное, ты сегодня супер!";

    let response = hub
        .chat(
            "echo.chat",
            "tone_chat",
            Some("sess1".to_string()),
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

    let snapshot = hub
        .tone_state_snapshot()
        .expect("tone state enabled");
    assert_eq!(snapshot.mood, ToneMood::Supportive);
    assert!(snapshot.intensity > 0.0);
}

/* neira:meta
id: NEI-20280501-120150-chat-hub-tone-test
intent: chore
summary: |
  Расширены интеграционные тесты чата проверкой обновления тонального состояния
  и приведён вызов `SynapseHub::chat` к актуальной сигнатуре.
*/
