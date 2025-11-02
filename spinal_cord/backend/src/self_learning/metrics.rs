use prometheus::{register_histogram, register_int_counter, Histogram, IntCounter};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

pub struct LearningMetrics {
    total_tasks: AtomicU64,
    successful_tasks: IntCounter,
    failed_tasks: IntCounter,
    learning_duration: Histogram,
}

impl LearningMetrics {
    pub fn new() -> Self {
        Self {
            total_tasks: AtomicU64::new(0),
            successful_tasks: register_int_counter!(
                "neira_learning_successful_tasks",
                "Количество успешных задач обучения"
            )
            .unwrap(),
            failed_tasks: register_int_counter!(
                "neira_learning_failed_tasks",
                "Количество неудачных задач обучения"
            )
            .unwrap(),
            learning_duration: register_histogram!(
                "neira_learning_duration_seconds",
                "Время выполнения задач обучения"
            )
            .unwrap(),
        }
    }

    pub async fn record_success(&self, duration: Duration) {
        self.total_tasks.fetch_add(1, Ordering::SeqCst);
        self.successful_tasks.inc();
        self.learning_duration.observe(duration.as_secs_f64());
    }

    pub async fn record_failure(&self) {
        self.total_tasks.fetch_add(1, Ordering::SeqCst);
        self.failed_tasks.inc();
    }
}
