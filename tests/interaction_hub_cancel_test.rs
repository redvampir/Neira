use std::sync::Arc;

use backend::analysis_node::{AnalysisNode, AnalysisResult, NodeStatus};
use backend::interaction_hub::InteractionHub;
use backend::action::metrics_collector_node::MetricsCollectorNode;
use backend::memory_node::MemoryNode;
use backend::node_registry::NodeRegistry;
use tokio_util::sync::CancellationToken;

struct CancelNode;

impl AnalysisNode for CancelNode {
    fn id(&self) -> &str {
        "cancel.node"
    }
    fn analysis_type(&self) -> &str {
        "test"
    }
    fn status(&self) -> NodeStatus {
        NodeStatus::Active
    }
    fn links(&self) -> &[String] {
        &[]
    }
    fn confidence_threshold(&self) -> f32 {
        0.0
    }
    fn analyze(&self, _input: &str, cancel: &CancellationToken) -> AnalysisResult {
        if cancel.is_cancelled() {
            let mut r = AnalysisResult::new(self.id(), "", vec![]);
            r.status = NodeStatus::Error;
            return r;
        }
        AnalysisResult::new(self.id(), "ok", vec![])
    }
    fn explain(&self) -> String {
        String::new()
    }
}

#[tokio::test]
async fn interaction_hub_saves_checkpoint_on_cancel() {
    let dir = tempfile::tempdir().unwrap();
    let registry = Arc::new(NodeRegistry::new(dir.path()).unwrap());
    registry.register_analysis_node(Arc::new(CancelNode));
    let memory = Arc::new(MemoryNode::new());
    let (metrics, _rx) = MetricsCollectorNode::channel();
    let hub = InteractionHub::new(registry.clone(), memory.clone(), metrics);
    hub.add_auth_token("t");
    let token = CancellationToken::new();
    token.cancel();
    let result = hub
        .analyze("cancel.node", "", "t", &token)
        .await
        .unwrap();
    assert_eq!(result.status, NodeStatus::Error);
    assert!(memory.load_checkpoint("cancel.node").is_some());
}
