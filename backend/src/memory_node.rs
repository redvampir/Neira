use std::sync::RwLock;

use crate::analysis_node::{AnalysisResult, QualityMetrics, ReasoningStep};

#[derive(Debug, Clone)]
pub struct MemoryRecord {
    pub id: String,
    pub quality_metrics: QualityMetrics,
    pub reasoning_chain: Vec<ReasoningStep>,
}

#[derive(Debug, Default)]
pub struct MemoryNode {
    records: RwLock<Vec<MemoryRecord>>,
}

impl MemoryNode {
    pub fn new() -> Self {
        Self { records: RwLock::new(Vec::new()) }
    }

    pub fn push_metrics(&self, result: &AnalysisResult) {
        let record = MemoryRecord {
            id: result.id.clone(),
            quality_metrics: result.quality_metrics.clone(),
            reasoning_chain: result.reasoning_chain.clone(),
        };
        self.records.write().unwrap().push(record);
    }

    pub fn records(&self) -> Vec<MemoryRecord> {
        self.records.read().unwrap().clone()
    }
}
