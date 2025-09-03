/* neira:meta
id: NEI-20270310-local-analysis-test
intent: test
summary: Проверяет, что анализ не запускается повторно через brain_loop.
*/
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use backend::action::diagnostics_cell::DiagnosticsCell;
use backend::action::metrics_collector_cell::MetricsCollectorCell;
use backend::analysis_cell::{AnalysisCell, AnalysisResult, CellStatus};
use backend::cell_registry::CellRegistry;
use backend::config::Config;
use backend::memory_cell::MemoryCell;
use backend::synapse_hub::SynapseHub;
use tokio_util::sync::CancellationToken;

struct CountCell {
    hits: Arc<AtomicUsize>,
}

impl AnalysisCell for CountCell {
    fn id(&self) -> &str {
        "count.cell"
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
        self.hits.fetch_add(1, Ordering::SeqCst);
        AnalysisResult::new(self.id(), "ok", vec![])
    }
    fn explain(&self) -> String {
        String::new()
    }
}

#[tokio::test]
async fn synapse_hub_runs_analysis_once() {
    let dir = tempfile::tempdir().unwrap();
    let registry = Arc::new(CellRegistry::new(dir.path()).unwrap());
    let counter = Arc::new(AtomicUsize::new(0));
    registry.register_analysis_cell(Arc::new(CountCell {
        hits: counter.clone(),
    }));
    let memory = Arc::new(MemoryCell::new());
    let (metrics, rx) = MetricsCollectorCell::channel();
    let (diagnostics, _dev_rx, _alert_rx) = DiagnosticsCell::new(rx, 5, metrics.clone());
    let cfg = Config::default();
    let hub = SynapseHub::new(registry, memory, metrics, diagnostics, &cfg);
    hub.add_auth_token("t");
    let token = CancellationToken::new();
    let _ = hub.analyze("count.cell", "", "t", &token).await;
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}
