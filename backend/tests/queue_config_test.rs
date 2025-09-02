use backend::analysis_cell::AnalysisResult;
use backend::memory_cell::MemoryCell;
use backend::queue_config::QueueConfig;
use backend::task_scheduler::Queue;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn queue_config_recalculates() {
    let _guard = ENV_LOCK.lock().unwrap();
    std::env::remove_var("ANALYSIS_QUEUE_FAST_MS");
    std::env::remove_var("ANALYSIS_QUEUE_LONG_MS");
    std::env::set_var("ANALYSIS_QUEUE_RECALC_MIN", "2");
    let memory = MemoryCell::new();

    let r = AnalysisResult::new("a", "", vec![]);
    memory.push_metrics(&r);
    memory.update_time("a", 50);
    let r = AnalysisResult::new("b", "", vec![]);
    memory.push_metrics(&r);
    memory.update_time("b", 500);

    let mut cfg = QueueConfig::new(&memory);
    assert_eq!(cfg.thresholds(), (100, 1000));

    let r = AnalysisResult::new("c", "", vec![]);
    memory.push_metrics(&r);
    memory.update_time("c", 2000);
    memory.update_time("c", 2000);

    let q = cfg.classify(1500, &memory);
    assert_eq!(q, Queue::Standard);
    assert_eq!(cfg.thresholds(), (500, 2000));
    std::env::remove_var("ANALYSIS_QUEUE_RECALC_MIN");
}

#[test]
fn queue_config_env_override() {
    let _guard = ENV_LOCK.lock().unwrap();
    std::env::set_var("ANALYSIS_QUEUE_FAST_MS", "150");
    std::env::set_var("ANALYSIS_QUEUE_LONG_MS", "1500");
    let memory = MemoryCell::new();

    let r = AnalysisResult::new("a", "", vec![]);
    memory.push_metrics(&r);
    memory.update_time("a", 50);

    let cfg = QueueConfig::new(&memory);
    assert_eq!(cfg.thresholds(), (150, 1500));
    std::env::remove_var("ANALYSIS_QUEUE_FAST_MS");
    std::env::remove_var("ANALYSIS_QUEUE_LONG_MS");
}
