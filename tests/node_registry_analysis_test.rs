use std::sync::Arc;

use backend::analysis_node::{AnalysisNode, AnalysisResult, NodeStatus};
use backend::node_registry::NodeRegistry;
use tokio_util::sync::CancellationToken;

struct DummyNode;

impl AnalysisNode for DummyNode {
    fn id(&self) -> &str {
        "dummy"
    }
    fn analysis_type(&self) -> &str {
        "dummy"
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
    fn analyze(&self, _input: &str, _cancel: &CancellationToken) -> AnalysisResult {
        AnalysisResult::new(self.id(), "out", vec![])
    }
    fn explain(&self) -> String {
        String::new()
    }
}

#[test]
fn registry_registers_analysis_nodes() {
    let dir = tempfile::tempdir().unwrap();
    let registry = NodeRegistry::new(dir.path()).unwrap();
    registry.register_analysis_node(Arc::new(DummyNode));
    assert!(registry.get_analysis_node("dummy").is_some());
}
