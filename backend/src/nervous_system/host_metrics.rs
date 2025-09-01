/* neira:meta
id: NEI-20250829-175425-host-metrics
intent: docs
summary: |
  Собирает метрики хоста и пересылает их коллектору.
*/

use std::sync::Arc;

use async_trait::async_trait;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tokio::time::{sleep, Duration};

use super::SystemProbe;
use crate::action::metrics_collector_node::{MetricsCollectorNode, MetricsRecord};
use crate::analysis_node::QualityMetrics;

const CPU_HIGH_THRESHOLD: f64 = 80.0;
const MEM_HIGH_THRESHOLD: f64 = 80.0;

/// Collects host level metrics and forwards them to the metrics system.
pub struct HostMetrics {
    sys: System,
    collector: Arc<MetricsCollectorNode>,
}

impl HostMetrics {
    /// Create a new host metrics collector.
    pub fn new(collector: Arc<MetricsCollectorNode>) -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        Self { sys, collector }
    }
}

#[async_trait]
impl SystemProbe for HostMetrics {
    async fn start(&mut self) {
        loop {
            let ms = self.collector.get_interval_ms();
            sleep(Duration::from_millis(ms)).await;
            self.collect();
        }
    }

    /// Refresh metrics and publish them via `metrics::gauge!` and `MetricsCollectorNode`.
    fn collect(&mut self) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();

        let cpu = self.sys.global_cpu_info().cpu_usage() as f64;
        let total_mem = self.sys.total_memory() as f64;
        let used_mem = self.sys.used_memory() as f64;
        let mem_percent = if total_mem > 0.0 {
            used_mem / total_mem * 100.0
        } else {
            0.0
        };

        if cpu > CPU_HIGH_THRESHOLD || mem_percent > MEM_HIGH_THRESHOLD {
            self.collector.set_low();
        } else {
            self.collector.set_normal();
        }

        metrics::gauge!("host_cpu_usage_percent").set(cpu);
        metrics::gauge!("host_memory_total_bytes").set(total_mem);
        metrics::gauge!("host_memory_used_bytes").set(used_mem);

        // Forward a simplified record to the MetricsCollectorNode so that
        // downstream consumers are notified about the updated metrics.
        let record = MetricsRecord {
            id: "system.host".to_string(),
            metrics: QualityMetrics {
                credibility: Some((cpu / 100.0) as f32),
                recency_days: None,
                demand: Some((used_mem / 1024.0 / 1024.0) as u32),
            },
        };
        self.collector.record(record);
    }
}
