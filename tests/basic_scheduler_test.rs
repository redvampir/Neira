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
