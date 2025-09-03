use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};

use backend::analysis_cell::{AnalysisCell, AnalysisResult, CellStatus};
use backend::brain::brain_loop;
use backend::cell_registry::CellRegistry;
use backend::circulatory_system::{DataFlowController, FlowMessage};
use backend::event_bus::{Event, EventBus, Subscriber};
use backend::task_scheduler::TaskScheduler;
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

    tokio::spawn(brain_loop(
        rx,
        registry.clone(),
        scheduler.clone(),
        event_bus,
    ));

    flow.send(FlowMessage::Task {
        id: "dummy".into(),
        payload: "".into(),
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    assert_eq!(counter.load(Ordering::SeqCst), 0);
    let next = scheduler.write().unwrap().next();
    assert_eq!(next, Some(("dummy".into(), "".into())));
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
    let counter = Arc::new(AtomicUsize::new(0));
    event_bus.subscribe(Arc::new(DummySubscriber {
        hits: counter.clone(),
    }));

    tokio::spawn(brain_loop(rx, registry, scheduler, event_bus.clone()));

    flow.send(FlowMessage::Event("ping".into()));

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    assert_eq!(counter.load(Ordering::SeqCst), 1);
}
