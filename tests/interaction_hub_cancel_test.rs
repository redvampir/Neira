use std::sync::Arc;

use backend::action::diagnostics_cell::DiagnosticsCell;
use backend::action::metrics_collector_cell::MetricsCollectorCell;
use backend::analysis_cell::{AnalysisCell, AnalysisResult, NodeStatus};
use backend::cell_registry::CellRegistry;
use backend::config::Config;
use backend::interaction_hub::InteractionHub;
use backend::memory_cell::MemoryCell;
use metrics_exporter_prometheus::PrometheusBuilder;
use tokio_util::sync::CancellationToken;

struct CancelCell;

impl AnalysisCell for CancelCell {
    fn id(&self) -> &str {
        "cancel.node"
    }
    fn analysis_type(&self) -> &str {
        "test"
    }
    fn status(&self) -> NodeStatus {
        NodeStatus::Active
    }
    fn links(&self) -> &[String] {
        &[]
    }
    fn confidence_threshold(&self) -> f32 {
        0.0
    }
    fn analyze(&self, _input: &str, cancel: &CancellationToken) -> AnalysisResult {
        if cancel.is_cancelled() {
            let mut r = AnalysisResult::new(self.id(), "", vec![]);
            r.status = NodeStatus::Error;
            return r;
        }
        AnalysisResult::new(self.id(), "ok", vec![])
    }
    fn explain(&self) -> String {
        String::new()
    }
}

#[tokio::test]
async fn interaction_hub_saves_checkpoint_on_cancel() {
    let handle = PrometheusBuilder::new().install_recorder().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let registry = Arc::new(CellRegistry::new(dir.path()).unwrap());
    registry.register_analysis_cell(Arc::new(CancelCell));
    let memory = Arc::new(MemoryCell::new());
    let (metrics, rx) = MetricsCollectorCell::channel();
    let (diagnostics, _dev_rx, _alert_rx) = DiagnosticsCell::new(rx, 5, metrics.clone());
    let cfg = Config::default();
    let hub = InteractionHub::new(registry.clone(), memory.clone(), metrics, diagnostics, &cfg);
    hub.add_auth_token("t");
    let token = CancellationToken::new();
    token.cancel();
    let result = hub.analyze("cancel.node", "", "t", &token).await.unwrap();
    assert_eq!(result.status, NodeStatus::Error);
    assert!(memory.load_checkpoint("cancel.node").is_some());
    let metrics = handle.render();
    assert!(metrics.contains("analysis_requests_total"));
    assert!(metrics.contains("analysis_errors_total"));
}
