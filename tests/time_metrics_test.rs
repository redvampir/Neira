use std::sync::Arc;
use std::time::Duration;

use backend::analysis_node::{AnalysisNode, AnalysisResult, NodeStatus};
use backend::interaction_hub::InteractionHub;
use backend::action::metrics_collector_node::MetricsCollectorNode;
use backend::memory_node::MemoryNode;
use backend::node_registry::NodeRegistry;
use tokio_util::sync::CancellationToken;

struct SleepNode;

impl AnalysisNode for SleepNode {
    fn id(&self) -> &str { "sleep" }
    fn analysis_type(&self) -> &str { "test" }
    fn status(&self) -> NodeStatus { NodeStatus::Active }
    fn links(&self) -> &[String] { &[] }
    fn confidence_threshold(&self) -> f32 { 0.0 }
    fn analyze(&self, _input: &str, _cancel: &CancellationToken) -> AnalysisResult {
        std::thread::sleep(Duration::from_millis(10));
        AnalysisResult::new(self.id(), "done", vec![])
    }
    fn explain(&self) -> String { String::new() }
}

#[tokio::test]
async fn hub_tracks_time_metrics() {
    let dir = tempfile::tempdir().unwrap();
    let registry = Arc::new(NodeRegistry::new(dir.path()).unwrap());
    registry.register_analysis_node(Arc::new(SleepNode));
    let memory = Arc::new(MemoryNode::new());
    let (metrics, _rx) = MetricsCollectorNode::channel();
    let hub = InteractionHub::new(registry.clone(), memory.clone(), metrics);
    hub.add_auth_token("t");
    let token = CancellationToken::new();
    hub.analyze("sleep", "", "t", &token).await.unwrap();
    hub.analyze("sleep", "", "t", &token).await.unwrap();
    let avg = memory.average_time_ms("sleep").unwrap();
    assert!(avg >= 10);
}
