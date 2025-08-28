use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::action::metrics_collector_node::{MetricsCollectorNode, MetricsRecord};
use crate::analysis_node::QualityMetrics;

/// Watches keyboard input and display output latency and reports delays
/// to the diagnostics system.
pub struct IoWatcher {
    collector: Arc<MetricsCollectorNode>,
    threshold: Duration,
}

impl IoWatcher {
    /// Create a new watcher. `threshold_ms` specifies the latency
    /// threshold in milliseconds after which a diagnostic record is sent.
    pub fn new(collector: Arc<MetricsCollectorNode>, threshold_ms: u64) -> Self {
        Self {
            collector,
            threshold: Duration::from_millis(threshold_ms),
        }
    }

    fn emit_delay(&self, kind: &str) {
        let record = MetricsRecord {
            id: format!("system.io.{}", kind),
            metrics: QualityMetrics {
                credibility: Some(0.0),
                ..Default::default()
            },
        };
        self.collector.record(record);
    }

    /// Record latency for keyboard input.
    pub fn record_keyboard_latency(&self, latency: Duration) {
        metrics::histogram!("io_keyboard_latency_ms").record(latency.as_millis() as f64);
        if latency > self.threshold {
            self.emit_delay("keyboard");
        }
    }

    /// Record latency for display output.
    pub fn record_display_latency(&self, latency: Duration) {
        metrics::histogram!("io_display_latency_ms").record(latency.as_millis() as f64);
        if latency > self.threshold {
            self.emit_delay("display");
        }
    }

    /// Continuously watches stdin/stdout and records latencies.
    pub async fn run(self) {
        let mut stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut buf = [0u8; 1];
        loop {
            let start_in = Instant::now();
            if stdin.read_exact(&mut buf).await.is_ok() {
                let latency_in = start_in.elapsed();
                self.record_keyboard_latency(latency_in);

                let start_out = Instant::now();
                if stdout.write_all(&buf).await.is_ok() {
                    let latency_out = start_out.elapsed();
                    self.record_display_latency(latency_out);
                    let _ = stdout.flush().await;
                }
            } else {
                tokio::task::yield_now().await;
            }
        }
    }
}
