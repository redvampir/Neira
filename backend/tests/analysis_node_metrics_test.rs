use backend::analysis_node::{AnalysisNode, AnalysisResult, NodeStatus};
use backend::interaction_hub::InteractionHub;
use backend::action::metrics_collector_node::MetricsCollectorNode;
use backend::action::diagnostics_node::DiagnosticsNode;
use backend::memory_node::MemoryNode;
use backend::node_registry::NodeRegistry;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

mod common;
use common::init_recorder;

struct TestAnalysisNode;

impl AnalysisNode for TestAnalysisNode {
    fn id(&self) -> &str { "test.analysis" }
    fn analysis_type(&self) -> &str { "test" }
    fn status(&self) -> NodeStatus { NodeStatus::Active }
    fn links(&self) -> &[String] { &[] }
    fn confidence_threshold(&self) -> f32 { 0.0 }
    fn analyze(&self, input: &str, _cancel: &CancellationToken) -> AnalysisResult {
        AnalysisResult::new(self.id(), input, vec![])
    }
    fn explain(&self) -> String { "test".into() }
}

#[tokio::test]
async fn interaction_hub_records_analysis_metric() {
    let data = init_recorder();
    let tmp = tempfile::tempdir().expect("tmpdir");
    let registry = Arc::new(NodeRegistry::new(tmp.path()).expect("registry"));
    registry.register_analysis_node(Arc::new(TestAnalysisNode));
    let memory = Arc::new(MemoryNode::new());
    let (metrics, rx) = MetricsCollectorNode::channel();
    let (diagnostics, _dev_rx) = DiagnosticsNode::new(rx, 5);
    let hub = InteractionHub::new(registry, memory, metrics, diagnostics);
    hub.add_auth_token("token");
    let cancel = CancellationToken::new();
    let _ = hub
        .analyze("test.analysis", "input", "token", &cancel)
        .await
        .expect("analysis");
    let records = data.lock().unwrap();
    assert!(
        records
            .iter()
            .any(|(n, _)| n == "analysis_node_request_duration_ms"),
        "no histogram recorded"
    );
    assert!(
        records
            .iter()
            .any(|(n, _)| n == "analysis_node_request_duration_ms_p95"),
        "no p95 histogram recorded"
    );
    assert!(
        records
            .iter()
            .any(|(n, _)| n == "analysis_node_request_duration_ms_p99"),
        "no p99 histogram recorded"
    );
}
