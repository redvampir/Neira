use backend::action::chat_node::{ChatNode, EchoChatNode};
use std::sync::{Arc, Mutex};
use std::io::Write;

fn init_subscriber() -> (Arc<Mutex<Vec<u8>>>, tracing::subscriber::DefaultGuard) {
    struct VecWriter(Arc<Mutex<Vec<u8>>>);
    impl Write for VecWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let writer_buffer = buffer.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_writer(move || VecWriter(writer_buffer.clone()))
        .with_ansi(false)
        .finish();
    let guard = tracing::subscriber::set_default(subscriber);
    (buffer, guard)
}

#[tokio::test]
async fn chat_node_logs_request_and_response() {
    let (buffer, _guard) = init_subscriber();
    let node = EchoChatNode::default();
    let input = "hello";
    let response = node.chat(input).await;
    assert_eq!(response, input);
    let logs = String::from_utf8(buffer.lock().unwrap().clone()).unwrap();
    assert!(logs.contains(&format!("chat request: {}", input)));
    assert!(logs.contains(&format!("chat response: {}", input)));
}

#[tokio::test]
async fn chat_node_handles_empty_input() {
    let (buffer, _guard) = init_subscriber();
    let node = EchoChatNode::default();
    let input = "";
    let response = node.chat(input).await;
    assert!(response.is_empty());
    let logs = String::from_utf8(buffer.lock().unwrap().clone()).unwrap();
    assert!(logs.contains("chat request: "));
    assert!(logs.contains("chat response: "));
}
