use std::sync::Arc;

use backend::action::chat_cell::EchoChatCell;
use backend::action::diagnostics_cell::DiagnosticsCell;
use backend::action::metrics_collector_cell::MetricsCollectorCell;
use backend::config::Config;
use backend::context::context_storage::FileContextStorage;
use backend::synapse_hub::SynapseHub;
use backend::memory_cell::MemoryCell;
use backend::cell_registry::CellRegistry;

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

/* neira:meta
id: NEI-20250227-chat-hub-test-update
intent: tests
summary: |
  Обновлен вызов `SynapseHub::chat` в тесте в соответствии с новым
  интерфейсом (добавлены `source` и `thread_id`).
*/
