use std::sync::Arc;
use std::time::Duration;

use backend::action::diagnostics_cell::DiagnosticsCell;
use backend::action::metrics_collector_cell::MetricsCollectorCell;
use backend::analysis_cell::{AnalysisCell, AnalysisResult, CellStatus};
use backend::cell_registry::CellRegistry;
use backend::config::Config;
use backend::interaction_hub::InteractionHub;
use backend::memory_cell::MemoryCell;
use metrics_exporter_prometheus::PrometheusBuilder;
use tokio_util::sync::CancellationToken;

struct SleepCell;

impl AnalysisCell for SleepCell {
    fn id(&self) -> &str {
        "sleep"
    }
    fn analysis_type(&self) -> &str {
        "test"
    }
    fn status(&self) -> CellStatus {
        CellStatus::Active
    }
    fn links(&self) -> &[String] {
        &[]
    }
    fn confidence_threshold(&self) -> f32 {
        0.0
    }
    fn analyze(&self, _input: &str, _cancel: &CancellationToken) -> AnalysisResult {
        std::thread::sleep(Duration::from_millis(10));
        AnalysisResult::new(self.id(), "done", vec![])
    }
    fn explain(&self) -> String {
        String::new()
    }
}

#[tokio::test]
async fn hub_tracks_time_metrics() {
    let handle = PrometheusBuilder::new().install_recorder().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let registry = Arc::new(CellRegistry::new(dir.path()).unwrap());
    registry.register_analysis_cell(Arc::new(SleepCell));
    let memory = Arc::new(MemoryCell::new());
    let (metrics, rx) = MetricsCollectorCell::channel();
    let (diagnostics, _dev_rx, _alert_rx) = DiagnosticsCell::new(rx, 5, metrics.clone());
    let cfg = Config::default();
    let hub = InteractionHub::new(registry.clone(), memory.clone(), metrics, diagnostics, &cfg);
    hub.add_auth_token("t");
    let token = CancellationToken::new();
    hub.analyze("sleep", "", "t", &token).await.unwrap();
    hub.analyze("sleep", "", "t", &token).await.unwrap();
    let avg = memory.average_time_ms("sleep").unwrap();
    assert!(avg >= 10);
    let metrics = handle.render();
    assert!(metrics.contains("analysis_requests_total"));
}
