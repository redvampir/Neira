use backend::analysis_node::QualityMetrics;
use backend::memory_node::UsageStats;
use backend::task_scheduler::{Priority, Queue, TaskScheduler};

#[test]
fn scheduler_orders_by_priority() {
    let mut scheduler = TaskScheduler::default();
    scheduler.enqueue(
        Queue::Standard,
        "a".into(),
        "one".into(),
        Priority::Low,
        None,
        vec![],
    );
    scheduler.enqueue(
        Queue::Standard,
        "b".into(),
        "two".into(),
        Priority::High,
        None,
        vec![],
    );
    scheduler.enqueue(
        Queue::Standard,
        "c".into(),
        "three".into(),
        Priority::Medium,
        None,
        vec![],
    );
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
    scheduler.enqueue_with_metrics(
        Queue::Standard,
        "high".into(),
        "a".into(),
        m1,
        UsageStats::default(),
        None,
        vec![],
    );
    scheduler.enqueue_with_metrics(
        Queue::Standard,
        "low".into(),
        "b".into(),
        m2,
        UsageStats::default(),
        None,
        vec![],
    );
    let (id, _) = scheduler.next().unwrap();
    assert_eq!(id, "high");
}
