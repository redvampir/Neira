/* neira:meta
id: NEI-20250317-chat-cell-metrics-recorder
intent: test
summary: reuse shared recorder to avoid resetting global metrics.
*/
use backend::action::chat_cell::{ChatCell, EchoChatCell};
use backend::context::context_storage::FileContextStorage;

mod common;
use common::init_recorder;

#[tokio::test]
async fn chat_cell_records_duration_metric() {
    std::env::set_var("CONTEXT_FLUSH_MS", "0");
    let tmp = tempfile::tempdir().expect("tmpdir");

    let data = init_recorder();

    let cell = EchoChatCell::default();
    let storage = FileContextStorage::new(tmp.path().join("context"));
    let resp = cell.chat("test_chat", None, "hi", &storage).await;
    assert_eq!(resp, "hi");

    let records = data.lock().unwrap();
    assert!(
        records
            .iter()
            .any(|(n, _)| n == "chat_cell_request_duration_ms"),
        "no histogram recorded"
    );
    assert!(
        records
            .iter()
            .any(|(n, _)| n == "chat_cell_request_duration_ms_p95"),
        "no p95 histogram recorded"
    );
    assert!(
        records
            .iter()
            .any(|(n, _)| n == "chat_cell_request_duration_ms_p99"),
        "no p99 histogram recorded"
    );
}
