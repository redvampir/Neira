/* neira:meta
id: NEI-20241002-brain-flow-test
intent: test
summary: Проверяет, что Brain обрабатывает FlowMessage::Task и FlowMessage::Event.
*/
/* neira:meta
id: NEI-20240514-brain-flow-test-typed
intent: test
summary: Использует FlowEvent и TaskPayload.
*/
use std::sync::{Arc, RwLock};

use backend::action::metrics_collector_cell::MetricsCollectorCell;
use backend::analysis_cell::{AnalysisCell, AnalysisResult, CellStatus};
use backend::brain::Brain;
use backend::cell_registry::CellRegistry;
use backend::circulatory_system::{DataFlowController, FlowEvent, FlowMessage, TaskPayload};
use backend::digestive_pipeline::ParsedInput;
use backend::event_bus::{Event, EventBus, Subscriber};
use backend::task_scheduler::TaskScheduler;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::time::{timeout, Duration};
use tokio_util::sync::CancellationToken;

struct TestCell {
    tx: UnboundedSender<String>,
}

/* neira:meta
id: NEI-20260531-brain-flow-parsed
intent: test
summary: TestCell обновлён для analyze_parsed.
*/
impl AnalysisCell for TestCell {
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
    fn analyze_parsed(&self, input: &ParsedInput, _: &CancellationToken) -> AnalysisResult {
        if let ParsedInput::Text(text) = input {
            let _ = self.tx.send(text.clone());
        }
        AnalysisResult::new(self.id(), "ok", vec![])
    }
    fn analyze(&self, input: &str, _cancel: &CancellationToken) -> AnalysisResult {
        let _ = self.tx.send(input.to_string());
        AnalysisResult::new(self.id(), "ok", vec![])
    }
    fn explain(&self) -> String {
        String::new()
    }
}

struct TestSubscriber {
    tx: UnboundedSender<String>,
}

impl Subscriber for TestSubscriber {
    fn on_event(&self, event: &dyn Event) {
        let _ = self.tx.send(event.name().to_string());
    }
}

#[tokio::test]
async fn brain_flow_test_receives_messages() {
    let dir = tempfile::tempdir().unwrap();
    let registry = Arc::new(CellRegistry::new(dir.path()).unwrap());
    let (task_tx, mut task_rx) = unbounded_channel();
    registry.register_analysis_cell(Arc::new(TestCell { tx: task_tx }));

    let scheduler = Arc::new(RwLock::new(TaskScheduler::default()));
    let (flow, rx) = DataFlowController::new();

    let event_bus = EventBus::new();
    let (event_tx, mut event_rx) = unbounded_channel();
    event_bus.subscribe(Arc::new(TestSubscriber { tx: event_tx }));

    let (metrics, _rx_metrics) = MetricsCollectorCell::channel();

    let brain = Arc::new(Brain::new(
        rx,
        flow.clone(),
        registry,
        scheduler,
        event_bus,
        metrics,
    ));
    brain.clone().spawn();

    flow.send(FlowMessage::Task {
        id: "dummy".into(),
        payload: TaskPayload::Text("payload".into()),
    });
    let payload = timeout(Duration::from_secs(1), task_rx.recv())
        .await
        .expect("task processed")
        .expect("task payload");
    assert_eq!(payload, "payload");

    flow.send(FlowMessage::Event(FlowEvent {
        name: "test".into(),
    }));
    let ev = timeout(Duration::from_secs(1), event_rx.recv())
        .await
        .expect("event processed")
        .expect("event name");
    assert_eq!(ev, "test");
}

/* neira:meta
id: NEI-20241003-brain-flow-test-update
intent: chore
summary: Обновлён тест под новый FlowReceiver и публикацию метрик кровотока.
*/
