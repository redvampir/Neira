use backend::analysis_node::QualityMetrics;
use backend::memory_node::UsageStats;
use backend::task_scheduler::TaskScheduler;

#[test]
fn scheduler_orders_by_priority() {
    let mut scheduler = TaskScheduler::default();
    scheduler.enqueue("a".into(), "one".into(), 1);
    scheduler.enqueue("b".into(), "two".into(), 3);
    scheduler.enqueue("c".into(), "three".into(), 2);
    let (id, _) = scheduler.next().unwrap();
    assert_eq!(id, "b");
}

#[test]
fn scheduler_uses_metrics() {
    let mut scheduler = TaskScheduler::default();
    let m1 = QualityMetrics {
        credibility: Some(0.9),
        recency_days: Some(1),
        demand: Some(10),
    };
    let m2 = QualityMetrics {
        credibility: Some(0.1),
        recency_days: Some(100),
        demand: Some(1),
    };
    scheduler.enqueue_with_metrics("high".into(), "a".into(), m1, UsageStats::default());
    scheduler.enqueue_with_metrics("low".into(), "b".into(), m2, UsageStats::default());
    let (id, _) = scheduler.next().unwrap();
    assert_eq!(id, "high");
}
