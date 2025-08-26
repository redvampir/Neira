struct AnalysisNode;

impl AnalysisNode {
    fn analyze(input: &str) -> bool {
        input.contains("trigger")
    }
}

#[test]
fn detects_trigger() {
    assert!(AnalysisNode::analyze("trigger word"));
    assert!(!AnalysisNode::analyze("no match"));
}
