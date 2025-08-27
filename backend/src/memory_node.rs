use std::collections::HashMap;
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
    checkpoints: RwLock<HashMap<String, AnalysisResult>>,
}

impl MemoryNode {
    pub fn new() -> Self {
        Self {
            records: RwLock::new(Vec::new()),
            checkpoints: RwLock::new(HashMap::new()),
        }
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

    pub fn save_checkpoint(&self, id: &str, result: &AnalysisResult) {
        self.checkpoints
            .write()
            .unwrap()
            .insert(id.to_string(), result.clone());
    }

    pub fn load_checkpoint(&self, id: &str) -> Option<AnalysisResult> {
        self.checkpoints.read().unwrap().get(id).cloned()
    }

    pub fn preload_by_trigger(&self, triggers: &[String]) -> Vec<MemoryRecord> {
        let records = self.records.read().unwrap();
        records
            .iter()
            .filter(|rec| {
                triggers.iter().any(|t| {
                    rec.id.contains(t)
                        || rec
                            .reasoning_chain
                            .iter()
                            .any(|step| step.content.contains(t))
                })
            })
            .cloned()
            .collect()
    }
}
