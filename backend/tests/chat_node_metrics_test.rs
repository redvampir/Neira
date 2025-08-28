use backend::action::chat_node::{ChatNode, EchoChatNode};
use backend::context::context_storage::FileContextStorage;

mod common;
use common::init_recorder;

#[tokio::test]
async fn chat_node_records_duration_metric() {
    std::env::set_var("CONTEXT_FLUSH_MS", "0");
    let tmp = tempfile::tempdir().expect("tmpdir");

    let data = init_recorder();

    let node = EchoChatNode::default();
    let storage = FileContextStorage::new(tmp.path().join("context"));
    let resp = node.chat("test_chat", None, "hi", &storage).await;
    assert_eq!(resp, "hi");

    let records = data.lock().unwrap();
    assert!(records.iter().any(|(n, _)| n == "chat_node_request_duration_ms"), "no histogram recorded");
}
