use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::analysis_node::QualityMetrics;
use crate::memory_node::UsageStats;

#[derive(Eq, PartialEq)]
struct ScheduledTask {
    priority: u8,
    id: String,
    input: String,
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct TaskScheduler {
    heap: BinaryHeap<ScheduledTask>,
}

impl TaskScheduler {
    pub fn enqueue(&mut self, id: String, input: String, priority: u8) {
        self.heap.push(ScheduledTask { priority, id, input });
    }

    pub fn enqueue_with_metrics(
        &mut self,
        id: String,
        input: String,
        metrics: QualityMetrics,
        stats: UsageStats,
    ) {
        let priority = compute_priority(&metrics, &stats);
        self.enqueue(id, input, priority);
    }

    pub fn next(&mut self) -> Option<(String, String)> {
        self.heap.pop().map(|t| (t.id, t.input))
    }
}

fn compute_priority(metrics: &QualityMetrics, stats: &UsageStats) -> u8 {
    let credibility = metrics.credibility.unwrap_or(0.0);
    let recency = metrics
        .recency_days
        .map(|d| 1.0 / (1.0 + d as f32))
        .unwrap_or(0.0);
    let demand = metrics.demand.unwrap_or(0) as f32 + stats.calls as f32;
    let demand_norm = if demand > 0.0 {
        (demand.log10() / 3.0).min(1.0)
    } else {
        0.0
    };
    let score = credibility * 0.5 + recency * 0.3 + demand_norm * 0.2;
    (score * 100.0).round().clamp(0.0, 100.0) as u8
}
