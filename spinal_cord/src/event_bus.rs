/* neira:meta
id: NEI-20251227-000000-event-bus
intent: feature
summary: |-
  Простой шина событий с трейтом Event и подписчиками.
*/
/* neira:meta
id: NEI-20250226-event-bus-flow
intent: feature
summary: Публикация событий транслируется через DataFlowController.
*/
/* neira:meta
id: NEI-20240728-event-bus-local-publish
intent: feature
summary: Добавлен метод локальной публикации без пересылки события в DataFlowController.
*/
/* neira:meta
id: NEI-20241026-event-bus-name-str
intent: refactor
summary: |-
  Метод Event::name возвращает &str, позволяя событиям иметь динамические имена.
*/
/* neira:meta
id: NEI-20240514-event-bus-flowevent
intent: refactor
summary: publish отправляет типизированное FlowEvent вместо строки.
*/
use crate::circulatory_system::{DataFlowController, FlowEvent, FlowMessage};
/* neira:meta
id: NEI-20270310-120100-event-bus-log-hook
intent: feature
summary: publish пишет событие в EventLog.
*/
/* neira:meta
id: NEI-20270311-event-serialize
intent: feature
summary: События могут отдавать JSON-представление для EventLog.
*/
use crate::event_log;
use serde::Serialize;
use std::any::Any;
use std::sync::{Arc, RwLock};

pub trait Event: Send + Sync {
    fn name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
    fn to_json(&self) -> Option<serde_json::Value> {
        None
    }
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

    pub fn publish_local(&self, event: &dyn Event) {
        for sub in self.subscribers.read().unwrap().iter() {
            sub.on_event(event);
        }
    }

    pub fn publish(&self, event: &dyn Event) {
        self.publish_local(event);
        if let Some(flow) = &*self.flow.read().unwrap() {
            flow.send(FlowMessage::Event(FlowEvent {
                name: event.name().to_string(),
            }));
        }
        event_log::append(event);
    }
}

use crate::factory::StemCellRecord;

pub struct CellCreated {
    pub record: StemCellRecord,
}

impl Event for CellCreated {
    fn name(&self) -> &str {
        "CellCreated"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Serialize)]
pub struct OrganBuilt {
    pub id: String,
}

impl Event for OrganBuilt {
    fn name(&self) -> &str {
        "OrganBuilt"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_json(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self).ok()
    }
}
