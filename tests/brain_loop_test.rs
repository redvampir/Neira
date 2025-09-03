use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};

/* neira:meta
id: NEI-20240725-brain-loop-test
intent: test
summary: Проверяет локальную обработку задач и пустоту канала DataFlowController.
*/
/* neira:meta
id: NEI-20240728-brain-loop-event-test
intent: test
summary: Убеждается, что события не возвращаются в DataFlowController при локальной публикации.
*/
use backend::analysis_cell::{AnalysisCell, AnalysisResult, CellStatus};
use backend::brain::brain_loop;
use backend::cell_registry::CellRegistry;
use backend::circulatory_system::{DataFlowController, FlowMessage};
use backend::event_bus::{Event, EventBus, Subscriber};
use backend::task_scheduler::{Priority, Queue, TaskScheduler};
use tokio::sync::mpsc::unbounded_channel;
use tokio::time::{timeout, Duration};
use tokio_util::sync::CancellationToken;

struct DummyCell {
    hits: Arc<AtomicUsize>,
}

impl AnalysisCell for DummyCell {
    fn id(&self) -> &str {
        "dummy"
    }
    fn analysis_type(&self) -> &str {
        "test"
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
        self.hits.fetch_add(1, Ordering::SeqCst);
        AnalysisResult::new(self.id(), "ok", vec![])
    }
    fn explain(&self) -> String {
        String::new()
    }
}

#[tokio::test]
async fn brain_loop_schedules_tasks() {
    let dir = tempfile::tempdir().unwrap();
    let registry = Arc::new(CellRegistry::new(dir.path()).unwrap());
    let counter = Arc::new(AtomicUsize::new(0));
    registry.register_analysis_cell(Arc::new(DummyCell {
        hits: counter.clone(),
    }));

    let (flow, rx) = DataFlowController::new();
    let scheduler = Arc::new(RwLock::new(TaskScheduler::default()));
    let event_bus = EventBus::new();

    let (tx_forward, rx_forward) = unbounded_channel();
    let (monitor_tx, mut monitor_rx) = unbounded_channel();
    tokio::spawn(async move {
        let mut rx = rx;
        while let Some(msg) = rx.recv().await {
            let _ = monitor_tx.send(msg.clone());
            let _ = tx_forward.send(msg);
        }
    });

    tokio::spawn(brain_loop(
        rx_forward,
        registry.clone(),
        scheduler.clone(),
        event_bus,
    ));

    flow.send(FlowMessage::Task {
        id: "dummy".into(),
        payload: "".into(),
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    monitor_rx.try_recv().unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    assert_eq!(counter.load(Ordering::SeqCst), 1);
    assert!(scheduler.write().unwrap().next().is_none());
    assert!(monitor_rx.try_recv().is_err());
}

/* neira:meta
id: NEI-20240810-manual-analysis-test
intent: test
summary: Извлекает задачу из планировщика и вручную запускает анализ.
*/
#[tokio::test]
async fn brain_loop_manual_analysis_runs_cell() {
    let dir = tempfile::tempdir().unwrap();
    let registry = Arc::new(CellRegistry::new(dir.path()).unwrap());
    let counter = Arc::new(AtomicUsize::new(0));
    registry.register_analysis_cell(Arc::new(DummyCell {
        hits: counter.clone(),
    }));

    let mut scheduler = TaskScheduler::default();
    scheduler.enqueue(
        Queue::Fast,
        "dummy".into(),
        "".into(),
        Priority::Low,
        None,
        vec![],
    );

    let (id, input) = scheduler.next().expect("task scheduled");
    let cell = registry.get_analysis_cell(&id).unwrap();
    let token = CancellationToken::new();
    cell.analyze(&input, &token);

    assert_eq!(counter.load(Ordering::SeqCst), 1);
    assert!(scheduler.next().is_none());
}

struct DummySubscriber {
    hits: Arc<AtomicUsize>,
}

impl Subscriber for DummySubscriber {
    fn on_event(&self, _event: &dyn Event) {
        self.hits.fetch_add(1, Ordering::SeqCst);
    }
}

#[tokio::test]
async fn brain_loop_publishes_events() {
    let dir = tempfile::tempdir().unwrap();
    let registry = Arc::new(CellRegistry::new(dir.path()).unwrap());
    let (flow, rx) = DataFlowController::new();
    let scheduler = Arc::new(RwLock::new(TaskScheduler::default()));
    let event_bus = EventBus::new();
    event_bus.attach_flow_controller(flow.clone());
    let counter = Arc::new(AtomicUsize::new(0));
    event_bus.subscribe(Arc::new(DummySubscriber {
        hits: counter.clone(),
    }));

    let (tx_forward, rx_forward) = unbounded_channel();
    let (monitor_tx, mut monitor_rx) = unbounded_channel();
    tokio::spawn(async move {
        let mut rx = rx;
        while let Some(msg) = rx.recv().await {
            let _ = monitor_tx.send(msg.clone());
            let _ = tx_forward.send(msg);
        }
    });

    tokio::spawn(brain_loop(
        rx_forward,
        registry,
        scheduler,
        event_bus.clone(),
    ));

    flow.send(FlowMessage::Event("ping".into()));

    timeout(Duration::from_millis(100), monitor_rx.recv())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(counter.load(Ordering::SeqCst), 1);
    assert!(timeout(Duration::from_millis(50), monitor_rx.recv())
        .await
        .is_err());
}
