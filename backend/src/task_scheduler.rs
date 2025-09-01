/* neira:meta
id: NEI-20250829-175425-task-scheduler
intent: docs
summary: |
  Планировщик задач с очередями по длительности и приоритетам.
*/

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;

use crate::analysis_node::QualityMetrics;
use crate::memory_node::UsageStats;

/// Очередь выполнения в зависимости от ожидаемой длительности задачи
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Queue {
    Fast,
    Standard,
    Long,
}

/// Приоритет задачи. Более высокие значения обрабатываются раньше
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        use Priority::*;
        let a = match self {
            High => 3u8,
            Medium => 2u8,
            Low => 1u8,
        };
        let b = match other {
            High => 3u8,
            Medium => 2u8,
            Low => 1u8,
        };
        a.cmp(&b)
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Параметры конфигурации планировщика
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    pub global_time_budget: u64,
    pub cancel_token_poll_ms: u64,
    pub checkpoint_interval_ms: u64,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            global_time_budget: 8 * 60 * 60 * 1000, // 8 часов
            cancel_token_poll_ms: 50,
            checkpoint_interval_ms: 60_000,
        }
    }
}

#[derive(Eq, PartialEq)]
struct ScheduledTask {
    priority: Priority,
    id: String,
    input: String,
    timeout_ms: Option<u64>,
    retry_count: u32,
    nodes: Vec<String>,
    created_at: Instant,
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

/// Планировщик задач с разделением по длительности и приоритетам
pub struct TaskScheduler {
    fast: BinaryHeap<ScheduledTask>,
    standard: BinaryHeap<ScheduledTask>,
    long: BinaryHeap<ScheduledTask>,
    pub config: SchedulerConfig,
}

impl Default for TaskScheduler {
    fn default() -> Self {
        Self {
            fast: BinaryHeap::new(),
            standard: BinaryHeap::new(),
            long: BinaryHeap::new(),
            config: SchedulerConfig::default(),
        }
    }
}

impl TaskScheduler {
    /// Добавление задачи в очередь с явным приоритетом и метаданными
    pub fn enqueue(
        &mut self,
        queue: Queue,
        id: String,
        input: String,
        priority: Priority,
        timeout_ms: Option<u64>,
        nodes: Vec<String>,
    ) {
        let task = ScheduledTask {
            priority,
            id,
            input,
            timeout_ms,
            retry_count: 0,
            nodes,
            created_at: Instant::now(),
        };
        match queue {
            Queue::Fast => self.fast.push(task),
            Queue::Standard => self.standard.push(task),
            Queue::Long => self.long.push(task),
        }
    }

    /// Добавление задачи с вычислением приоритета на основе метрик
    pub fn enqueue_with_metrics(
        &mut self,
        queue: Queue,
        id: String,
        input: String,
        metrics: QualityMetrics,
        stats: UsageStats,
        timeout_ms: Option<u64>,
        nodes: Vec<String>,
    ) {
        let priority = compute_priority(&metrics, &stats);
        self.enqueue(queue, id, input, priority, timeout_ms, nodes);
    }

    /// Получение следующей задачи, учитывая порядок очередей fast > standard > long
    pub fn next(&mut self) -> Option<(String, String)> {
        if let Some(t) = self.fast.pop() {
            return Some((t.id, t.input));
        }
        if let Some(t) = self.standard.pop() {
            return Some((t.id, t.input));
        }
        self.long.pop().map(|t| (t.id, t.input))
    }

    /// Возвращает длины очередей (fast, standard, long) для оценки backpressure
    pub(crate) fn queue_lengths(&self) -> (usize, usize, usize) {
        (self.fast.len(), self.standard.len(), self.long.len())
    }
}

/// Расчёт приоритета на основе метрик качества и статистики использования
pub fn compute_priority(metrics: &QualityMetrics, stats: &UsageStats) -> Priority {
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
    if score > 0.66 {
        Priority::High
    } else if score > 0.33 {
        Priority::Medium
    } else {
        Priority::Low
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Low
    }
}
