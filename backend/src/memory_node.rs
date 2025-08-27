use std::collections::HashMap;
use std::sync::RwLock;

use chrono::{DateTime, Utc};
use crate::analysis_node::{AnalysisResult, QualityMetrics, ReasoningStep};

#[derive(Debug, Clone, Default)]
pub struct UsageStats {
    pub calls: u64,
    pub last_access: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default)]
pub struct TimeMetrics {
    pub total_duration_ms: u128,
    pub count: u64,
}

#[derive(Debug, Clone)]
pub struct MemoryRecord {
    pub id: String,
    pub quality_metrics: QualityMetrics,
    pub reasoning_chain: Vec<ReasoningStep>,
    pub usage: UsageStats,
    pub time: TimeMetrics,
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
        let mut records = self.records.write().unwrap();
        if let Some(rec) = records.iter_mut().find(|r| r.id == result.id) {
            rec.quality_metrics = result.quality_metrics.clone();
            rec.reasoning_chain = result.reasoning_chain.clone();
        } else {
            records.push(MemoryRecord {
                id: result.id.clone(),
                quality_metrics: result.quality_metrics.clone(),
                reasoning_chain: result.reasoning_chain.clone(),
                usage: UsageStats::default(),
                time: TimeMetrics::default(),
            });
        }
    }

    pub fn records(&self) -> Vec<MemoryRecord> {
        self.records.read().unwrap().clone()
    }

    pub fn save_checkpoint(&self, id: &str, result: &AnalysisResult) {
        self
            .checkpoints
            .write()
            .unwrap()
            .insert(id.to_string(), result.clone());
    }

    pub fn load_checkpoint(&self, id: &str) -> Option<AnalysisResult> {
        self.checkpoints.read().unwrap().get(id).cloned()
    }

    pub fn preload_by_trigger(&self, triggers: &[String]) -> Vec<MemoryRecord> {
        let mut records = self.records.write().unwrap();
        let mut matched = Vec::new();
        for rec in records.iter_mut() {
            let hit = triggers.iter().any(|t| {
                rec.id.contains(t)
                    || rec
                        .reasoning_chain
                        .iter()
                        .any(|step| step.content.contains(t))
            });
            if hit {
                rec.usage.calls += 1;
                rec.usage.last_access = Some(Utc::now());
                matched.push(rec.clone());
            }
        }
        matched
    }

    pub fn update_time(&self, id: &str, duration_ms: u128) {
        let mut records = self.records.write().unwrap();
        if let Some(rec) = records.iter_mut().find(|r| r.id == id) {
            rec.time.total_duration_ms += duration_ms;
            rec.time.count += 1;
        }
    }

    pub fn get_usage(&self, id: &str) -> UsageStats {
        self.records
            .read()
            .unwrap()
            .iter()
            .find(|r| r.id == id)
            .map(|r| r.usage.clone())
            .unwrap_or_default()
    }

    pub fn get_quality(&self, id: &str) -> QualityMetrics {
        self.records
            .read()
            .unwrap()
            .iter()
            .find(|r| r.id == id)
            .map(|r| r.quality_metrics.clone())
            .unwrap_or_default()
    }

    pub fn average_time_ms(&self, id: &str) -> Option<u128> {
        self.records
            .read()
            .unwrap()
            .iter()
            .find(|r| r.id == id)
            .and_then(|r| {
                if r.time.count > 0 {
                    Some(r.time.total_duration_ms / r.time.count as u128)
                } else {
                    None
                }
            })
    }
}
