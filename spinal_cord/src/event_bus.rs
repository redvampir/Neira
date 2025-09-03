/* neira:meta
id: NEI-20251227-event-bus
intent: code
summary: |-
  Простой шина событий с трейтом Event и подписчиками.
*/
/* neira:meta
id: NEI-20250226-event-bus-flow
intent: feature
summary: Публикация событий транслируется через DataFlowController.
*/
use crate::circulatory_system::{DataFlowController, FlowMessage};
use std::any::Any;
use std::sync::{Arc, RwLock};

pub trait Event: Send + Sync {
    fn name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
}

pub trait Subscriber: Send + Sync {
    fn on_event(&self, event: &dyn Event);
}

#[derive(Default)]
pub struct EventBus {
    subscribers: RwLock<Vec<Arc<dyn Subscriber>>>,
    flow: RwLock<Option<Arc<DataFlowController>>>,
}

impl EventBus {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            subscribers: RwLock::new(Vec::new()),
            flow: RwLock::new(None),
        })
    }

    /// Подключение глобального контроллера потоков
    pub fn attach_flow_controller(&self, flow: Arc<DataFlowController>) {
        *self.flow.write().unwrap() = Some(flow);
    }

    pub fn subscribe(&self, sub: Arc<dyn Subscriber>) {
        self.subscribers.write().unwrap().push(sub);
    }

    pub fn publish(&self, event: &dyn Event) {
        for sub in self.subscribers.read().unwrap().iter() {
            sub.on_event(event);
        }
        if let Some(flow) = &*self.flow.read().unwrap() {
            flow.send(FlowMessage::Event(event.name().to_string()));
        }
    }
}

use crate::factory::StemCellRecord;

pub struct CellCreated {
    pub record: StemCellRecord,
}

impl Event for CellCreated {
    fn name(&self) -> &'static str {
        "CellCreated"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct OrganBuilt {
    pub id: String,
}

impl Event for OrganBuilt {
    fn name(&self) -> &'static str {
        "OrganBuilt"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
