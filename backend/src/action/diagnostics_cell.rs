/* neira:meta
id: NEI-20250829-175425-diagnostics-cell
intent: docs
summary: |
  Анализирует поток метрик, фиксирует аномалии и уведомляет разработчика.
*/

use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tracing::warn;

use crate::action::metrics_collector_cell::{MetricsCollectorCell, MetricsRecord};
use crate::action_cell::ActionCell;
use crate::memory_cell::MemoryCell;

/// Максимальный размер окна истории для анализа.
pub const MAX_HISTORY: usize = 100;

/// Запрос разработчику, отправляемый при невозможности устранить проблему автоматически.
#[derive(Debug, Clone)]
pub struct DeveloperRequest {
    pub description: String,
}

/// Событие, сигнализирующее об обнаружении аномалии в данных.
#[derive(Debug, Clone)]
pub struct Alert {
    pub message: String,
}

/// Простая проверка на аномалию по правилу трёх сигм.
///
/// Принимает последовательность значений и возвращает `Alert`, если
/// последнее значение отклоняется от среднего предыдущих значений
/// более чем на три стандартных отклонения.
pub fn detect_anomaly(values: &[f32]) -> Option<Alert> {
    if values.len() < 2 {
        return None;
    }

    let (last, rest) = values.split_last()?;
    let mean = rest.iter().sum::<f32>() / rest.len() as f32;
    let variance = rest
        .iter()
        .map(|v| {
            let diff = *v - mean;
            diff * diff
        })
        .sum::<f32>()
        / rest.len() as f32;
    let std_dev = variance.sqrt();
    if (*last - mean).abs() > 3.0 * std_dev {
        warn!(value = *last, mean, std_dev, "anomaly detected");
        Some(Alert {
            message: format!("value {} deviates from mean {} by more than 3σ", last, mean),
        })
    } else {
        None
    }
}

/// Узел диагностики, который анализирует поступающие метрики
/// и реагирует при превышении порогов.
#[derive(Clone)]
pub struct DiagnosticsCell {
    error_threshold: u32,
    error_count: Arc<AtomicU32>,
    notify: UnboundedSender<DeveloperRequest>,
    attempt_fix: Arc<dyn Fn() -> bool + Send + Sync>,
    alert: UnboundedSender<Alert>,
    collector: Arc<MetricsCollectorCell>,
}

impl DiagnosticsCell {
    /// Создаёт узел и запускает обработку входящих событий метрик.
    pub fn new(
        rx: UnboundedReceiver<MetricsRecord>,
        error_threshold: u32,
        collector: Arc<MetricsCollectorCell>,
    ) -> (
        Arc<Self>,
        UnboundedReceiver<DeveloperRequest>,
        UnboundedReceiver<Alert>,
    ) {
        Self::new_with_fix(rx, error_threshold, collector, Arc::new(|| true))
    }

    /// Создаёт узел с настраиваемой функцией попытки исправления.
    pub fn new_with_fix(
        mut rx: UnboundedReceiver<MetricsRecord>,
        error_threshold: u32,
        collector: Arc<MetricsCollectorCell>,
        attempt_fix: Arc<dyn Fn() -> bool + Send + Sync>,
    ) -> (
        Arc<Self>,
        UnboundedReceiver<DeveloperRequest>,
        UnboundedReceiver<Alert>,
    ) {
        let (notify_tx, notify_rx) = unbounded_channel();
        let (alert_tx, alert_rx) = unbounded_channel();
        let cell = Arc::new(Self {
            error_threshold,
            error_count: Arc::new(AtomicU32::new(0)),
            notify: notify_tx,
            attempt_fix,
            alert: alert_tx,
            collector,
        });
        let cell_clone = cell.clone();
        tokio::spawn(async move {
            let mut history: VecDeque<f32> = VecDeque::new();
            while let Some(record) = rx.recv().await {
                metrics::counter!("diagnostics_cell_requests_total").increment(1);
                // Простое правило: низкая достоверность считается ошибкой.
                if let Some(cred) = record.metrics.credibility {
                    history.push_back(cred);
                    if history.len() > MAX_HISTORY {
                        history.pop_front();
                    }
                    let slice = history.make_contiguous();
                    if let Some(alert) = detect_anomaly(&slice[..]) {
                        warn!(id=%record.id, message=%alert.message, "publishing alert");
                        let _ = cell_clone.alert.send(alert);
                        cell_clone.collector.set_normal();
                    }
                    if cred < 0.5 {
                        metrics::counter!("diagnostics_cell_errors_total").increment(1);
                        let count = cell_clone.error_count.fetch_add(1, Ordering::SeqCst) + 1;
                        if count >= cell_clone.error_threshold {
                            warn!(id=%record.id, count, "credibility below threshold");
                            cell_clone.collector.set_normal();
                            if !(cell_clone.attempt_fix)() {
                                let _ = cell_clone.notify.send(DeveloperRequest {
                                    description: format!(
                                        "credibility below threshold for {}",
                                        record.id
                                    ),
                                });
                            }
                        }
                    } else {
                        // Сбрасываем счётчик при успешных записях.
                        cell_clone.error_count.store(0, Ordering::SeqCst);
                        cell_clone.collector.set_low();
                    }
                }
            }
        });
        (cell, notify_rx, alert_rx)
    }
}

impl ActionCell for DiagnosticsCell {
    fn id(&self) -> &str {
        "metrics.diagnostics"
    }

    fn preload(&self, _triggers: &[String], _memory: &Arc<MemoryCell>) {}
}
