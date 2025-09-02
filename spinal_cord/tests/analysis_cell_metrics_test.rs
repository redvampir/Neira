use backend::action::diagnostics_cell::DiagnosticsCell;
use backend::action::metrics_collector_cell::MetricsCollectorCell;
use backend::analysis_cell::{AnalysisCell, AnalysisResult, CellStatus};
use backend::synapse_hub::SynapseHub;
use backend::config::Config;
use backend::memory_cell::MemoryCell;
use backend::cell_registry::CellRegistry;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

mod common;
use common::init_recorder;

struct TestAnalysisCell;

impl AnalysisCell for TestAnalysisCell {
    fn id(&self) -> &str {
        "test.analysis"
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
    fn analyze(&self, input: &str, _cancel: &CancellationToken) -> AnalysisResult {
        AnalysisResult::new(self.id(), input, vec![])
    }
    fn explain(&self) -> String {
        "test".into()
    }
}

#[tokio::test]
async fn synapse_hub_records_analysis_metric() {
    let data = init_recorder();
    let tmp = tempfile::tempdir().expect("tmpdir");
    let registry = Arc::new(CellRegistry::new(tmp.path()).expect("registry"));
    registry.register_analysis_cell(Arc::new(TestAnalysisCell));
    let memory = Arc::new(MemoryCell::new());
    let (metrics, rx) = MetricsCollectorCell::channel();
    let (diagnostics, _dev_rx, _alert_rx) = DiagnosticsCell::new(rx, 5, metrics.clone());
    let cfg = Config::default();
    let hub = SynapseHub::new(registry, memory, metrics, diagnostics, &cfg);
    hub.add_auth_token("token");
    let cancel = CancellationToken::new();
    let _ = hub
        .analyze("test.analysis", "input", "token", &cancel)
        .await
        .expect("analysis");
    let records = data.lock().unwrap();
    assert!(
        records
            .iter()
            .any(|(n, _)| n == "analysis_cell_request_duration_ms"),
        "no histogram recorded"
    );
    assert!(
        records
            .iter()
            .any(|(n, _)| n == "analysis_cell_request_duration_ms_p95"),
        "no p95 histogram recorded"
    );
    assert!(
        records
            .iter()
            .any(|(n, _)| n == "analysis_cell_request_duration_ms_p99"),
        "no p99 histogram recorded"
    );
}
