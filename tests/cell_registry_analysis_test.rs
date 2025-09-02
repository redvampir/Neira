use std::sync::Arc;

use backend::analysis_cell::{AnalysisCell, AnalysisResult, CellStatus};
use backend::cell_registry::CellRegistry;
use tokio_util::sync::CancellationToken;

struct DummyCell;

impl AnalysisCell for DummyCell {
    fn id(&self) -> &str {
        "dummy"
    }
    fn analysis_type(&self) -> &str {
        "dummy"
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
        AnalysisResult::new(self.id(), "out", vec![])
    }
    fn explain(&self) -> String {
        String::new()
    }
}

#[test]
fn registry_registers_analysis_cells() {
    let dir = tempfile::tempdir().unwrap();
    let registry = CellRegistry::new(dir.path()).unwrap();
    registry.register_analysis_cell(Arc::new(DummyCell));
    assert!(registry.get_analysis_cell("dummy").is_some());
}
