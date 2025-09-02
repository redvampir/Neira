/* neira:meta
id: NEI-20250829-175425-memory-cell
intent: docs
summary: |
  Хранит результаты анализа и метаданные, поддерживает предзагрузку и приоритизацию.
*/

use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::{Arc, RwLock};

use lru::LruCache;

use chrono::{DateTime, Utc};
use tokio::spawn;
use std::time::Instant;

use crate::analysis_cell::{AnalysisResult, QualityMetrics, ReasoningStep};
use crate::task_scheduler::{compute_priority, Priority};

#[derive(Debug, Clone, Default)]
pub struct UsageStats {
    pub calls: u64,
    pub last_access: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default)]
pub struct TimeMetrics {
    pub total_duration_ms: u128,
    pub count: u64,
    pub smoothed_duration_ms: f64,
    pub min_ms: Option<u128>,
    pub max_ms: Option<u128>,
    pub median_ms: Option<u128>,
    durations: Vec<u128>,
}

#[derive(Debug, Clone)]
pub struct MemoryRecord {
    pub id: String,
    pub quality_metrics: QualityMetrics,
    pub reasoning_chain: Vec<ReasoningStep>,
    pub usage: UsageStats,
    pub time: TimeMetrics,
    pub priority: Priority,
}

#[derive(Debug)]
pub struct MemoryCell {
    records: RwLock<Vec<MemoryRecord>>,
    checkpoints: RwLock<HashMap<String, AnalysisResult>>,
    preload_cache: RwLock<LruCache<String, Vec<MemoryRecord>>>,
}

impl MemoryCell {
    pub fn new() -> Self {
        Self {
            records: RwLock::new(Vec::new()),
            checkpoints: RwLock::new(HashMap::new()),
            preload_cache: RwLock::new(LruCache::new(NonZeroUsize::new(128).unwrap())),
        }
    }

    pub fn base_path(&self) -> String {
        std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| ".".into())
    }

    pub fn set_cache_capacity(&self, capacity: usize) {
        *self.preload_cache.write().unwrap() =
            LruCache::new(NonZeroUsize::new(capacity.max(1)).unwrap());
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
                priority: Priority::Low,
            });
        }
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
        metrics::counter!("memory_cell_requests_total").increment(1);
        let res = self.checkpoints.read().unwrap().get(id).cloned();
        if res.is_none() {
            metrics::counter!("memory_cell_errors_total").increment(1);
        }
        res
    }

    pub fn preload_by_trigger(&self, triggers: &[String]) -> Vec<MemoryRecord> {
        let start = Instant::now();
        let mut key = triggers.to_vec();
        key.sort();
        let cache_key = key.join("|");
        if let Some(records) = self
            .preload_cache
            .write()
            .unwrap()
            .get(&cache_key)
            .cloned()
        {
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            metrics::histogram!("memory_cell_preload_duration_ms").record(elapsed);
            metrics::histogram!("memory_cell_preload_duration_ms_p95").record(elapsed);
            metrics::histogram!("memory_cell_preload_duration_ms_p99").record(elapsed);
            return records;
        }

        let mut records_lock = self.records.write().unwrap();
        let mut matched = Vec::new();
        for rec in records_lock.iter_mut() {
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
        self.preload_cache
            .write()
            .unwrap()
            .put(cache_key, matched.clone());
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        metrics::histogram!("memory_cell_preload_duration_ms").record(elapsed);
        metrics::histogram!("memory_cell_preload_duration_ms_p95").record(elapsed);
        metrics::histogram!("memory_cell_preload_duration_ms_p99").record(elapsed);
        matched
    }

    pub fn update_time(&self, id: &str, duration_ms: u128) {
        let mut records = self.records.write().unwrap();
        if let Some(rec) = records.iter_mut().find(|r| r.id == id) {
            rec.time.total_duration_ms += duration_ms;
            rec.time.count += 1;
            let alpha = 0.3;
            rec.time.smoothed_duration_ms = if rec.time.count == 1 {
                duration_ms as f64
            } else {
                alpha * duration_ms as f64 + (1.0 - alpha) * rec.time.smoothed_duration_ms
            };
            rec.time.min_ms = Some(rec.time.min_ms.map_or(duration_ms, |m| m.min(duration_ms)));
            rec.time.max_ms = Some(rec.time.max_ms.map_or(duration_ms, |m| m.max(duration_ms)));
            rec.time.durations.push(duration_ms);
            let mut ds = rec.time.durations.clone();
            ds.sort();
            rec.time.median_ms = ds.get(ds.len() / 2).cloned();
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
                    Some(r.time.smoothed_duration_ms.round() as u128)
                } else {
                    None
                }
            })
    }

    pub fn time_distribution(&self, id: &str) -> Option<(u128, u128, u128)> {
        self.records
            .read()
            .unwrap()
            .iter()
            .find(|r| r.id == id)
            .and_then(|r| match (r.time.min_ms, r.time.median_ms, r.time.max_ms) {
                (Some(min), Some(med), Some(max)) => Some((min, med, max)),
                _ => None,
            })
    }

    pub fn get_priority(&self, id: &str) -> Priority {
        self.records
            .read()
            .unwrap()
            .iter()
            .find(|r| r.id == id)
            .map(|r| r.priority)
            .unwrap_or(Priority::Low)
    }

    fn recalc_priority(&self, id: &str) {
        let mut records = self.records.write().unwrap();
        if let Some(rec) = records.iter_mut().find(|r| r.id == id) {
            rec.priority = compute_priority(&rec.quality_metrics, &rec.usage);
        }
    }

    pub fn recalc_priority_async(self: Arc<Self>, id: String) {
        spawn(async move {
            self.recalc_priority(&id);
        });
    }
}

impl Default for MemoryCell {
    fn default() -> Self {
        Self::new()
    }
}
