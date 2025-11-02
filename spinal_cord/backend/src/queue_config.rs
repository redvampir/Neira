/* neira:meta
id: NEI-20250922-adaptive-queue-config
intent: code
summary: |
  Адаптивные пороги очередей анализа вычисляются из исторических метрик
  и могут переопределяться переменными окружения.
*/

use crate::memory_cell::MemoryCell;
use crate::task_scheduler::Queue;

/// Runtime configuration for analysis task queues.
#[derive(Debug)]
pub struct QueueConfig {
    fast_ms: u128,
    long_ms: u128,
    min_samples: u64,
    last_total: u64,
    fast_override: Option<u128>,
    long_override: Option<u128>,
}

impl QueueConfig {
    /// Build config using historical metrics from `MemoryCell`.
    pub fn new(memory: &MemoryCell) -> Self {
        let fast_override = std::env::var("ANALYSIS_QUEUE_FAST_MS")
            .ok()
            .and_then(|v| v.parse().ok());
        let long_override = std::env::var("ANALYSIS_QUEUE_LONG_MS")
            .ok()
            .and_then(|v| v.parse().ok());
        let min_samples = std::env::var("ANALYSIS_QUEUE_RECALC_MIN")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        let (fast_ms, long_ms, total) = Self::compute_thresholds(memory);
        Self {
            fast_ms: fast_override.unwrap_or(fast_ms),
            long_ms: long_override.unwrap_or(long_ms),
            min_samples,
            last_total: total,
            fast_override,
            long_override,
        }
    }

    /// Return current thresholds.
    pub fn thresholds(&self) -> (u128, u128) {
        (self.fast_ms, self.long_ms)
    }

    /// Classify average latency into a queue and recompute thresholds if needed.
    pub fn classify(&mut self, avg_time: u128, memory: &MemoryCell) -> Queue {
        self.maybe_recompute(memory);
        if avg_time < self.fast_ms {
            Queue::Fast
        } else if avg_time < self.long_ms {
            Queue::Standard
        } else {
            Queue::Long
        }
    }

    fn maybe_recompute(&mut self, memory: &MemoryCell) {
        let total = Self::total_requests(memory);
        if total >= self.last_total + self.min_samples {
            let (fast_ms, long_ms, total_now) = Self::compute_thresholds(memory);
            self.fast_ms = self.fast_override.unwrap_or(fast_ms);
            self.long_ms = self.long_override.unwrap_or(long_ms);
            self.last_total = total_now;
        }
    }

    fn total_requests(memory: &MemoryCell) -> u64 {
        memory.records().into_iter().map(|r| r.time.count).sum()
    }

    fn compute_thresholds(memory: &MemoryCell) -> (u128, u128, u64) {
        let records = memory.records();
        let mut avgs = Vec::new();
        let mut total = 0u64;
        for r in records {
            if r.time.count > 0 {
                avgs.push(r.time.smoothed_duration_ms.round() as u128);
                total += r.time.count;
            }
        }
        if avgs.len() >= 3 {
            avgs.sort_unstable();
            let fast_idx = avgs.len() / 3;
            let long_idx = avgs.len() * 2 / 3;
            (avgs[fast_idx], avgs[long_idx], total)
        } else {
            (100, 1000, total)
        }
    }
}
