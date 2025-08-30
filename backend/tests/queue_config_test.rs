use backend::analysis_node::AnalysisResult;
use backend::memory_node::MemoryNode;
use backend::queue_config::QueueConfig;
use backend::task_scheduler::Queue;

#[test]
fn queue_config_recalculates() {
    std::env::set_var("ANALYSIS_QUEUE_RECALC_MIN", "2");
    let memory = MemoryNode::new();

    let mut r = AnalysisResult::new("a", "", vec![]);
    memory.push_metrics(&r);
    memory.update_time("a", 50);
    r = AnalysisResult::new("b", "", vec![]);
    memory.push_metrics(&r);
    memory.update_time("b", 500);

    let mut cfg = QueueConfig::new(&memory);
    assert_eq!(cfg.thresholds(), (50, 500));

    r = AnalysisResult::new("c", "", vec![]);
    memory.push_metrics(&r);
    memory.update_time("c", 2000);
    memory.update_time("c", 2000);

    let q = cfg.classify(1500, &memory);
    assert_eq!(q, Queue::Long);
    assert_eq!(cfg.thresholds(), (500, 2000));
    std::env::remove_var("ANALYSIS_QUEUE_RECALC_MIN");
}

#[test]
fn queue_config_env_override() {
    std::env::set_var("ANALYSIS_QUEUE_FAST_MS", "150");
    std::env::set_var("ANALYSIS_QUEUE_LONG_MS", "1500");
    let memory = MemoryNode::new();

    let mut r = AnalysisResult::new("a", "", vec![]);
    memory.push_metrics(&r);
    memory.update_time("a", 50);

    let cfg = QueueConfig::new(&memory);
    assert_eq!(cfg.thresholds(), (150, 1500));
    std::env::remove_var("ANALYSIS_QUEUE_FAST_MS");
    std::env::remove_var("ANALYSIS_QUEUE_LONG_MS");
}
