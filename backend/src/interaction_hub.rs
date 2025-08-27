use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::analysis_node::{AnalysisResult, NodeStatus};
use crate::memory_node::MemoryNode;
use crate::node_registry::NodeRegistry;

pub struct InteractionHub {
    pub registry: Arc<NodeRegistry>,
    pub memory: Arc<MemoryNode>,
}

impl InteractionHub {
    pub fn new(registry: Arc<NodeRegistry>, memory: Arc<MemoryNode>) -> Self {
        Self { registry, memory }
    }

    pub fn analyze(
        &self,
        id: &str,
        input: &str,
        cancel_token: &CancellationToken,
    ) -> Option<AnalysisResult> {
        let triggers: Vec<String> =
            input.split_whitespace().map(|s| s.to_lowercase()).collect();
        let _ = self.memory.preload_by_trigger(&triggers);
        let node = self.registry.get_analysis_node(id)?;
        let result = node.analyze(input, cancel_token);
        if result.status == NodeStatus::Error {
            self.memory.save_checkpoint(id, &result);
        } else {
            self.memory.push_metrics(&result);
        }
        Some(result)
    }
}
