use backend::analysis_node::AnalysisResult;
use backend::memory_node::MemoryNode;

#[test]
fn memory_node_stores_metrics_and_chain() {
    let mut result = AnalysisResult::new("id", "out", vec![]);
    result.add_step("first");
    let memory = MemoryNode::new();
    memory.push_metrics(&result);
    let records = memory.records();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].quality_metrics.demand, Some(1));
    assert_eq!(records[0].reasoning_chain[0].content, "first");
}
