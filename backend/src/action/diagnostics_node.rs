use std::sync::{Arc, atomic::{AtomicU32, Ordering}};

use tokio::sync::mpsc::UnboundedReceiver;
use tracing::warn;

use crate::action::metrics_collector_node::MetricsRecord;
use crate::action_node::ActionNode;
use crate::memory_node::MemoryNode;

/// Узел диагностики, который анализирует поступающие метрики
/// и реагирует при превышении порогов.
#[derive(Clone)]
pub struct DiagnosticsNode {
    error_threshold: u32,
    error_count: Arc<AtomicU32>,
}

impl DiagnosticsNode {
    /// Создаёт узел и запускает обработку входящих событий метрик.
    pub fn new(mut rx: UnboundedReceiver<MetricsRecord>, error_threshold: u32) -> Arc<Self> {
        let node = Arc::new(Self {
            error_threshold,
            error_count: Arc::new(AtomicU32::new(0)),
        });
        let node_clone = node.clone();
        tokio::spawn(async move {
            while let Some(record) = rx.recv().await {
                // Простое правило: низкая достоверность считается ошибкой.
                if let Some(cred) = record.metrics.credibility {
                    if cred < 0.5 {
                        let count = node_clone.error_count.fetch_add(1, Ordering::SeqCst) + 1;
                        if count >= node_clone.error_threshold {
                            warn!(id=%record.id, count, "credibility below threshold");
                        }
                    } else {
                        // Сбрасываем счётчик при успешных записях.
                        node_clone.error_count.store(0, Ordering::SeqCst);
                    }
                }
            }
        });
        node
    }
}

impl ActionNode for DiagnosticsNode {
    fn id(&self) -> &str {
        "metrics.diagnostics"
    }

    fn preload(&self, _triggers: &[String], _memory: &Arc<MemoryNode>) {}
}
