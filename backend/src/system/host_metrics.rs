use std::sync::Arc;

use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind};

use crate::action::metrics_collector_node::{MetricsCollectorNode, MetricsRecord};
use crate::analysis_node::QualityMetrics;

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

    /// Refresh metrics and publish them via `metrics::gauge!` and `MetricsCollectorNode`.
    pub fn poll(&mut self) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();

        let cpu = self.sys.global_cpu_info().cpu_usage() as f64;
        let total_mem = self.sys.total_memory() as f64;
        let used_mem = self.sys.used_memory() as f64;

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
