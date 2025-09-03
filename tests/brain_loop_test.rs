use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};

/* neira:meta
id: NEI-20240725-brain-loop-test
intent: test
summary: Имитация боевой схемы: планировщик и шина используют общий DataFlowController; проверяет отсутствие циклов.
*/
/* neira:meta
id: NEI-20240728-brain-loop-event-test
intent: test
summary: Подключённый планировщик и шина не образуют циклов при обработке событий.
*/
/* neira:meta
id: NEI-20240514-brain-loop-test-typed
intent: test
summary: Использует FlowEvent и TaskPayload в сообщениях.
*/
use backend::action::metrics_collector_cell::MetricsCollectorCell;
use backend::analysis_cell::{AnalysisCell, AnalysisResult, CellStatus};
use backend::brain::Brain;
use backend::cell_registry::CellRegistry;
use backend::circulatory_system::{DataFlowController, FlowEvent, FlowMessage, TaskPayload};
use backend::digestive_pipeline::ParsedInput;
use backend::event_bus::{Event, EventBus, Subscriber};
use backend::task_scheduler::{Priority, Queue, TaskScheduler};
use tokio_util::sync::CancellationToken;

/* neira:meta
id: NEI-20260530-brainloop-digest
intent: test
summary: DummyCell обновлён для ParsedInput.
*/
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
    fn analyze_parsed(&self, _input: &ParsedInput, _cancel: &CancellationToken) -> AnalysisResult {
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
    scheduler.write().unwrap().set_flow_controller(flow.clone());
    let event_bus = EventBus::new();
    event_bus.attach_flow_controller(flow.clone());

    let (metrics, _rx_metrics) = MetricsCollectorCell::channel();
    let brain = Arc::new(Brain::new(
        rx,
        flow.clone(),
        registry.clone(),
        scheduler.clone(),
        event_bus,
        metrics,
    ));
    brain.clone().spawn();

    flow.send(FlowMessage::Task {
        id: "dummy".into(),
        payload: TaskPayload::Text("".into()),
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    assert_eq!(counter.load(Ordering::SeqCst), 1);
    assert!(scheduler.write().unwrap().next().is_none());
    assert_eq!(flow.sent_count(), flow.received_count());
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
    scheduler.write().unwrap().set_flow_controller(flow.clone());
    let event_bus = EventBus::new();
    event_bus.attach_flow_controller(flow.clone());
    let counter = Arc::new(AtomicUsize::new(0));
    event_bus.subscribe(Arc::new(DummySubscriber {
        hits: counter.clone(),
    }));

    let (metrics, _rx_metrics) = MetricsCollectorCell::channel();
    let brain = Arc::new(Brain::new(
        rx,
        flow.clone(),
        registry,
        scheduler.clone(),
        event_bus.clone(),
        metrics,
    ));
    brain.clone().spawn();

    flow.send(FlowMessage::Event(FlowEvent {
        name: "ping".into(),
    }));

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    assert_eq!(counter.load(Ordering::SeqCst), 1);
    assert!(scheduler.write().unwrap().next().is_none());
    assert_eq!(flow.sent_count(), flow.received_count());
}

/* neira:meta
id: NEI-20240821-brain-metrics-test
intent: test
summary: Brain отправляет запись в MetricsCollectorCell при обработке события.
*/
#[tokio::test]
async fn brain_loop_records_metrics() {
    let dir = tempfile::tempdir().unwrap();
    let registry = Arc::new(CellRegistry::new(dir.path()).unwrap());
    let (flow, rx) = DataFlowController::new();
    let scheduler = Arc::new(RwLock::new(TaskScheduler::default()));
    scheduler.write().unwrap().set_flow_controller(flow.clone());
    let event_bus = EventBus::new();
    event_bus.attach_flow_controller(flow.clone());
    let (metrics, mut metrics_rx) = MetricsCollectorCell::channel();

    let brain = Arc::new(Brain::new(
        rx,
        flow.clone(),
        registry,
        scheduler.clone(),
        event_bus,
        metrics,
    ));
    brain.clone().spawn();

    flow.send(FlowMessage::Event(FlowEvent {
        name: "ping".into(),
    }));
    flow.send(FlowMessage::Event(FlowEvent {
        name: "pong".into(),
    }));

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let first = metrics_rx.try_recv().unwrap();
    let second = metrics_rx.try_recv().unwrap();
    assert_eq!(first.id, "brain.event");
    assert_eq!(second.id, "brain.event");
}

/* neira:meta
id: NEI-20241003-brain-loop-test-update
intent: chore
summary: Тесты обновлены под счётчики кровотока и FlowReceiver без промежуточных каналов.
*/
