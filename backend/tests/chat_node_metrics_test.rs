use backend::action::chat_node::{ChatNode, EchoChatNode};
use backend::context::context_storage::FileContextStorage;
use metrics::Recorder;
use metrics::{Counter, Gauge, Histogram, Key, KeyName, SharedString, Unit};
use std::sync::{Arc, Mutex};

struct TestRecorder {
    data: Arc<Mutex<Vec<(String, f64)>>>,
}

impl Recorder for TestRecorder {
    fn describe_counter(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}
    fn describe_gauge(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}
    fn describe_histogram(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}

    fn register_counter(&self, _key: &Key) -> Counter {
        Counter::noop()
    }
    fn register_gauge(&self, _key: &Key) -> Gauge {
        Gauge::noop()
    }
    fn register_histogram(&self, key: &Key) -> Histogram {
        let name = key.name().to_string();
        let data = self.data.clone();
        let hist = TestHistogram { name, data };
        Histogram::from_arc(Arc::new(hist))
    }
}

struct TestHistogram {
    name: String,
    data: Arc<Mutex<Vec<(String, f64)>>>,
}

impl metrics::HistogramFn for TestHistogram {
    fn record(&self, value: f64) {
        self.data.lock().unwrap().push((self.name.clone(), value));
    }
}

#[tokio::test]
async fn chat_node_records_duration_metric() {
    std::env::set_var("CONTEXT_FLUSH_MS", "0");
    let tmp = tempfile::tempdir().expect("tmpdir");

    let data = Arc::new(Mutex::new(Vec::new()));
    let recorder = TestRecorder { data: data.clone() };
    metrics::set_boxed_recorder(Box::new(recorder)).expect("set recorder");

    let node = EchoChatNode::default();
    let storage = FileContextStorage::new(tmp.path().join("context"));
    let resp = node.chat("test_chat", None, "hi", &storage).await;
    assert_eq!(resp, "hi");

    let records = data.lock().unwrap();
    assert!(records.iter().any(|(n, _)| n == "chat_node_request_duration_ms"), "no histogram recorded");
}
