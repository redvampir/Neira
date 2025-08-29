use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::action_node::ActionNode;
use crate::analysis_node::QualityMetrics;
use crate::memory_node::MemoryNode;

/// Запись метрик, пересылаемая `MetricsCollectorNode`.
#[derive(Debug, Clone)]
pub struct MetricsRecord {
    pub id: String,
    pub metrics: QualityMetrics,
}

/// Узел, который принимает записи метрик и пересылает их как сообщения через канал.
pub struct MetricsCollectorNode {
    tx: UnboundedSender<MetricsRecord>,
    fast_interval_ms: u64,
    slow_interval_ms: u64,
    current_interval_ms: AtomicU64,
}

impl MetricsCollectorNode {
    /// Создаёт узел и возвращает связанный с ним приёмник для сообщений.
    pub fn channel() -> (Arc<Self>, UnboundedReceiver<MetricsRecord>) {
        let (tx, rx) = unbounded_channel();
        let fast = std::env::var("METRICS_FAST_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1_000);
        let slow = std::env::var("METRICS_SLOW_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30_000);
        (
            Arc::new(Self {
                tx,
                fast_interval_ms: fast,
                slow_interval_ms: slow,
                current_interval_ms: AtomicU64::new(slow),
            }),
            rx,
        )
    }

    /// Отправляет запись метрик для дальнейшей обработки.
    pub fn record(&self, record: MetricsRecord) {
        if self.tx.send(record).is_err() {
            metrics::counter!("metrics_collector_node_errors_total").increment(1);
        } else {
            metrics::counter!("metrics_collector_node_requests_total").increment(1);
        }
    }

    /// Текущий интервал опроса в миллисекундах.
    pub fn get_interval_ms(&self) -> u64 {
        self.current_interval_ms.load(Ordering::SeqCst)
    }

    /// Переключает коллектор в «быстрый» режим.
    pub fn set_fast(&self) {
        self
            .current_interval_ms
            .store(self.fast_interval_ms, Ordering::SeqCst);
    }

    /// Переключает коллектор в «медленный» режим.
    pub fn set_slow(&self) {
        self
            .current_interval_ms
            .store(self.slow_interval_ms, Ordering::SeqCst);
    }
}

impl ActionNode for MetricsCollectorNode {
    fn id(&self) -> &str {
        "metrics.collector"
    }

    fn preload(&self, _triggers: &[String], _memory: &Arc<MemoryNode>) {}
}

