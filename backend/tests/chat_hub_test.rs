use std::sync::Arc;

use backend::interaction_hub::InteractionHub;
use backend::action::metrics_collector_node::MetricsCollectorNode;
use backend::memory_node::MemoryNode;
use backend::node_registry::NodeRegistry;
use backend::action::chat_node::EchoChatNode;
use backend::context::context_storage::FileContextStorage;

#[tokio::test]
async fn chat_hub_rejects_empty_message() {
    let templates_dir = tempfile::tempdir().unwrap();
    let registry = Arc::new(NodeRegistry::new(templates_dir.path()).expect("registry"));
    let memory = Arc::new(MemoryNode::new());
    let (metrics, _rx) = MetricsCollectorNode::channel();
    let hub = InteractionHub::new(registry.clone(), memory, metrics);
    hub.add_auth_token("secret");
    registry.register_chat_node(Arc::new(EchoChatNode::default()));

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
        )
        .await;
    assert!(res.is_err());
}
