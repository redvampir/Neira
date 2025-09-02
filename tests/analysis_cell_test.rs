struct AnalysisCell;

impl AnalysisCell {
    fn analyze(input: &str) -> bool {
        input.contains("trigger")
    }
}

#[test]
fn detects_trigger() {
    assert!(AnalysisCell::analyze("trigger word"));
    assert!(!AnalysisCell::analyze("no match"));
}
