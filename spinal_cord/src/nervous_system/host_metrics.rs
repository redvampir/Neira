/* neira:meta
id: NEI-20250829-175425-host-metrics
intent: docs
summary: |
  Собирает метрики хоста и пересылает их коллектору.
*/
/* neira:meta
id: NEI-20250902-host-metrics-new-cells
intent: feature
summary: |
  Добавлен сбор количества новых клеток.
*/
/* neira:meta
id: NEI-20240607-hostmetrics-stop
intent: feature
summary: Добавлен CancellationToken и метод stop для остановки сборщика.
*/

use std::sync::Arc;

use async_trait::async_trait;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

use super::SystemProbe;
use crate::action::metrics_collector_cell::{MetricsCollectorCell, MetricsRecord};
use crate::analysis_cell::QualityMetrics;
use crate::factory::StemCellFactory;

const CPU_HIGH_THRESHOLD: f64 = 80.0;
const MEM_HIGH_THRESHOLD: f64 = 80.0;
const NEW_CELLS_HIGH_THRESHOLD: u64 = 5;

/// Сводные метрики хоста, включая сведения о новых клетках.
#[derive(Debug, Default)]
pub struct Metrics {
    pub cpu: f64,
    pub mem_percent: f64,
    pub new_cells: u64,
}

/// Простейшая проверка превышения порогов.
pub fn detect_anomaly(metrics: &Metrics) -> bool {
    metrics.cpu > CPU_HIGH_THRESHOLD
        || metrics.mem_percent > MEM_HIGH_THRESHOLD
        || metrics.new_cells > NEW_CELLS_HIGH_THRESHOLD
}

/// Collects host level metrics and forwards them to the metrics system.
pub struct HostMetrics {
    sys: System,
    collector: Arc<MetricsCollectorCell>,
    factory: Arc<StemCellFactory>,
    last_total_cells: usize,
    shutdown: CancellationToken,
}

impl HostMetrics {
    /// Create a new host metrics collector.
    pub fn new(
        collector: Arc<MetricsCollectorCell>,
        factory: Arc<StemCellFactory>,
        shutdown: CancellationToken,
    ) -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        Self {
            sys,
            collector,
            factory,
            last_total_cells: 0,
            shutdown,
        }
    }
}

#[async_trait]
impl SystemProbe for HostMetrics {
    async fn start(&mut self) {
        loop {
            let ms = self.collector.get_interval_ms();
            tokio::select! {
                _ = sleep(Duration::from_millis(ms)) => self.collect(),
                _ = self.shutdown.cancelled() => break,
            }
        }
    }

    /// Refresh metrics and publish them via `metrics::gauge!` and `MetricsCollectorCell`.
    fn collect(&mut self) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();

        let cpu = self.sys.global_cpu_usage() as f64;
        let total_mem = self.sys.total_memory() as f64;
        let used_mem = self.sys.used_memory() as f64;
        let mem_percent = if total_mem > 0.0 {
            used_mem / total_mem * 100.0
        } else {
            0.0
        };

        // данные о клетках
        let (total_cells, active_cells) = self.factory.counts();
        metrics::gauge!("factory_cells_total").set(total_cells as f64);
        metrics::gauge!("factory_cells_active").set(active_cells as f64);
        let new_cells = total_cells.saturating_sub(self.last_total_cells);
        if new_cells > 0 {
            metrics::counter!("factory_new_cells_total").increment(new_cells as u64);
        }
        self.last_total_cells = total_cells;

        let m = Metrics {
            cpu,
            mem_percent,
            new_cells: new_cells as u64,
        };
        if detect_anomaly(&m) {
            self.collector.set_low();
        } else {
            self.collector.set_normal();
        }

        metrics::gauge!("host_cpu_usage_percent").set(cpu);
        metrics::gauge!("host_memory_total_bytes").set(total_mem);
        metrics::gauge!("host_memory_used_bytes").set(used_mem);

        // Forward a simplified record to the MetricsCollectorCell so that
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

    fn stop(&mut self) {
        self.shutdown.cancel();
    }
}

/* neira:meta
id: NEI-20240513-hostmetrics-lint
intent: chore
summary: Использован saturating_sub для корректного подсчёта новых клеток.
*/
