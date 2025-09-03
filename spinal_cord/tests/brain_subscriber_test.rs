/* neira:meta
id: NEI-20240930-brain-subscriber-test
intent: test
summary: Проверяет, что BrainSubscriber пересылает события в DataFlowController.
*/
/* neira:meta
id: NEI-20240514-brain-subscriber-test-flowevent
intent: test
summary: Проверяет имя в типизированном FlowEvent.
*/
use std::any::Any;
use std::sync::Arc;

use backend::brain::BrainSubscriber;
use backend::circulatory_system::{DataFlowController, FlowEvent, FlowMessage};
use backend::event_bus::{Event, EventBus};

struct DummyEvent;

impl Event for DummyEvent {
    fn name(&self) -> &str {
        "DummyEvent"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[tokio::test]
async fn brain_subscriber_forwards_events() {
    let (flow, mut rx) = DataFlowController::new();
    let bus = EventBus::new();
    bus.subscribe(Arc::new(BrainSubscriber::new(flow.clone())));

    bus.publish_local(&DummyEvent);

    let msg = rx.try_recv().expect("message forwarded");
    assert!(matches!(msg, FlowMessage::Event(ev) if ev.name == "DummyEvent"));
}
