use std::sync::Arc;

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
#[derive(Clone)]
pub struct MetricsCollectorNode {
    tx: UnboundedSender<MetricsRecord>,
}

impl MetricsCollectorNode {
    /// Создаёт узел и возвращает связанный с ним приёмник для сообщений.
    pub fn channel() -> (Arc<Self>, UnboundedReceiver<MetricsRecord>) {
        let (tx, rx) = unbounded_channel();
        (Arc::new(Self { tx }), rx)
    }

    /// Отправляет запись метрик для дальнейшей обработки.
    pub fn record(&self, record: MetricsRecord) {
        let _ = self.tx.send(record);
    }
}

impl ActionNode for MetricsCollectorNode {
    fn id(&self) -> &str {
        "metrics.collector"
    }

    fn preload(&self, _triggers: &[String], _memory: &Arc<MemoryNode>) {}
}

