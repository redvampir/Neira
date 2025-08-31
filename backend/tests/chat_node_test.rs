use backend::action::chat_node::{ChatNode, EchoChatNode};
use backend::context::context_storage::FileContextStorage;
use std::path::PathBuf;

fn install_file_subscriber(
    dir: &tempfile::TempDir,
) -> (
    tracing::subscriber::DefaultGuard,
    PathBuf,
    tracing_appender::non_blocking::WorkerGuard,
) {
    let log_path = dir.path().join("test.log");
    // Use a non-rolling file appender to a temp file we control
    let file_appender = tracing_appender::rolling::never(dir.path(), "test.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(false)
        .finish();
    let default_guard = tracing::subscriber::set_default(subscriber);
    (default_guard, log_path, guard)
}

#[tokio::test]
async fn chat_node_echo_logs_request_and_response() {
    std::env::set_var("CONTEXT_FLUSH_MS", "0");
    let tmp = tempfile::tempdir().expect("tmpdir");
    let (_sub_guard, log_path, writer_guard) = install_file_subscriber(&tmp);

    let node = EchoChatNode::default();
    let storage = FileContextStorage::new(tmp.path().join("context"));
    let chat_id = "test_chat";
    let session_id = "sess1";
    let input = "hello";
    let resp = node
        .chat(chat_id, Some(session_id.to_string()), input, &storage)
        .await;
    assert_eq!(resp, input);

    // Ensure logs are flushed
    drop(writer_guard);

    let contents = std::fs::read_to_string(&log_path).expect("read log");
    assert!(
        contents.contains("chat request:"),
        "no request log: {contents}"
    );
    assert!(
        contents.contains("chat response:"),
        "no response log: {contents}"
    );
    assert!(
        contents.contains("hello"),
        "payload not found in logs: {contents}"
    );
}

#[tokio::test]
async fn chat_node_handles_empty_input() {
    std::env::set_var("CONTEXT_FLUSH_MS", "0");
    let tmp = tempfile::tempdir().expect("tmpdir");
    let (_sub_guard, log_path, writer_guard) = install_file_subscriber(&tmp);

    let node = EchoChatNode::default();
    let storage = FileContextStorage::new(tmp.path().join("context"));
    let chat_id = "test_chat";
    let session_id = "sess_empty";
    let input = "";
    let resp = node
        .chat(chat_id, Some(session_id.to_string()), input, &storage)
        .await;
    // Current behavior: echo input as-is (empty string)
    assert_eq!(resp, input);

    drop(writer_guard);

    let contents = std::fs::read_to_string(&log_path).expect("read log");
    assert!(
        contents.contains("chat request:"),
        "no request log for empty input: {contents}"
    );
    assert!(
        contents.contains("chat response:"),
        "no response log for empty input: {contents}"
    );
}
