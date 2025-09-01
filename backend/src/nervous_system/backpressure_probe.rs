/* neira:meta
id: NEI-20250314-backpressure-probe
intent: docs
summary: |-
  Монитор очередей планировщика, публикующий backpressure и выполняющий троттлинг.
*/

use std::sync::Arc;
use tokio::time::{sleep, Duration};

use crate::interaction_hub::InteractionHub;

/// Проба нагрузки на очереди: вычисляет длины и публикует backpressure,
/// а также применяет троттлинг при превышении порогов.
pub struct BackpressureProbe {
    hub: Arc<InteractionHub>,
}

impl BackpressureProbe {
    /// Создание новой пробы на основе InteractionHub.
    pub fn new(hub: Arc<InteractionHub>) -> Self {
        Self { hub }
    }

    /// Возвращает длины очередей планировщика (fast, standard, long).
    pub fn queue_lengths(&self) -> (usize, usize, usize) {
        let sched = self.hub.scheduler.read().unwrap();
        sched.queue_lengths()
    }

    /// Суммарная длина очередей.
    pub fn backpressure_sum(&self) -> u64 {
        let (a, b, c) = self.queue_lengths();
        (a + b + c) as u64
    }

    /// Публикация значения backpressure через gauge.
    pub fn publish(&self) {
        metrics::gauge!("backpressure").set(self.backpressure_sum() as f64);
    }

    /// Троттлинг запросов в зависимости от нагрузки в очередях.
    pub async fn throttle(&self) {
        let bp = self.backpressure_sum();
        self.publish();
        let bp_high = std::env::var("BACKPRESSURE_HIGH_WATERMARK")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(100);
        let bp_sleep = std::env::var("BACKPRESSURE_THROTTLE_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);
        if bp_sleep > 0 && bp > bp_high {
            metrics::counter!("throttle_events_total").increment(1);
            sleep(Duration::from_millis(bp_sleep)).await;
        }
        if std::env::var("AUTO_BACKOFF_ENABLED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
            && bp > bp_high
        {
            let max_backoff = std::env::var("BP_MAX_BACKOFF_MS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(200);
            let over = (bp - bp_high) as f64 / (bp_high.max(1) as f64);
            let extra = ((bp_sleep as f64) * over).min(max_backoff as f64) as u64;
            if extra > 0 {
                sleep(Duration::from_millis(extra)).await;
            }
        }
    }
}
