use backend::analysis_node::AnalysisResult;
use backend::memory_node::MemoryNode;

#[test]
fn memory_node_stores_metrics_and_chain() {
    let mut result = AnalysisResult::new("id", "out", vec!["rust".into()]);
    result.add_step("first");

    let memory = MemoryNode::new();
    memory.push_metrics(&result);

    // проверяем чекпоинты
    memory.save_checkpoint("id", &result);
    assert!(memory.load_checkpoint("id").is_some());

    // предзагрузка по триггерам
    let preloaded = memory.preload_by_trigger(&vec!["rust".into()]);
    assert_eq!(preloaded.len(), 1);

    // проверяем записи
    let records = memory.records();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].quality_metrics.demand, Some(2));
    assert_eq!(records[0].reasoning_chain[0].content, "rust");
}
