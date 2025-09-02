/* neira:meta
id: NEI-20250829-175425-metrics-collector
intent: docs
scope: backend/action
summary: |
  Сборщик метрик с динамическим интервалом опроса.
env:
  - METRICS_NORMAL_INTERVAL_MS
  - METRICS_LOW_INTERVAL_MS
*/

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::action_cell::ActionCell;
use crate::analysis_cell::QualityMetrics;
use crate::memory_cell::MemoryCell;

/// Запись метрик, пересылаемая `MetricsCollectorCell`.
#[derive(Debug, Clone)]
pub struct MetricsRecord {
    pub id: String,
    pub metrics: QualityMetrics,
}

/// Узел, который принимает записи метрик и пересылает их как сообщения через канал.
pub struct MetricsCollectorCell {
    tx: UnboundedSender<MetricsRecord>,
    normal_interval_ms: u64,
    low_interval_ms: u64,
    current_interval_ms: AtomicU64,
}

impl MetricsCollectorCell {
    /// Создаёт узел и возвращает связанный с ним приёмник для сообщений.
    pub fn channel() -> (Arc<Self>, UnboundedReceiver<MetricsRecord>) {
        let (tx, rx) = unbounded_channel();
        let normal = std::env::var("METRICS_NORMAL_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1_000);
        let low = std::env::var("METRICS_LOW_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30_000);
        (
            Arc::new(Self {
                tx,
                normal_interval_ms: normal,
                low_interval_ms: low,
                current_interval_ms: AtomicU64::new(normal),
            }),
            rx,
        )
    }

    /// Отправляет запись метрик для дальнейшей обработки.
    pub fn record(&self, record: MetricsRecord) {
        if self.tx.send(record).is_err() {
            metrics::counter!("metrics_collector_cell_errors_total").increment(1);
        } else {
            metrics::counter!("metrics_collector_cell_requests_total").increment(1);
        }
    }

    /// Текущий интервал опроса в миллисекундах.
    pub fn get_interval_ms(&self) -> u64 {
        self.current_interval_ms.load(Ordering::SeqCst)
    }

    /// Переключает коллектор в режим «normal».
    pub fn set_normal(&self) {
        self
            .current_interval_ms
            .store(self.normal_interval_ms, Ordering::SeqCst);
    }

    /// Переключает коллектор в режим «low».
    pub fn set_low(&self) {
        self
            .current_interval_ms
            .store(self.low_interval_ms, Ordering::SeqCst);
    }
}

impl ActionCell for MetricsCollectorCell {
    fn id(&self) -> &str {
        "metrics.collector"
    }

    fn preload(&self, _triggers: &[String], _memory: &Arc<MemoryCell>) {}
}

