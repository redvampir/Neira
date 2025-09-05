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
/* neira:meta
id: NEI-20270610-120000-lymphatic-event
intent: feature
summary: Добавлено событие lymphatic_filter.activated и метод data для передачи полей события.
*/
use crate::circulatory_system::{DataFlowController, FlowEvent, FlowMessage};
/* neira:meta
id: NEI-20270310-120100-event-bus-log-hook
intent: feature
summary: publish пишет событие в EventLog и учитывает метрики публикаций.
*/
use crate::event_log;
use serde_json::{json, Value};
use std::any::Any;
use std::sync::{Arc, RwLock};

pub trait Event: Send + Sync {
    fn name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
    fn data(&self) -> Option<Value> {
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
        if event_log::append(event).is_ok() {
            metrics::counter!("event_bus_publish_total").increment(1);
        } else {
            metrics::counter!("event_bus_publish_failures_total").increment(1);
        }
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LymphaticDecision {
    Keep,
    Remove,
}

impl LymphaticDecision {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Keep => "keep",
            Self::Remove => "remove",
        }
    }
}

pub struct LymphaticFilterActivated {
    pub function_id: String,
    pub similarity: f32,
    pub decision: LymphaticDecision,
}

impl Event for LymphaticFilterActivated {
    fn name(&self) -> &str {
        "lymphatic_filter.activated"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn data(&self) -> Option<Value> {
        Some(json!({
            "function_id": self.function_id,
            "similarity": self.similarity,
            "decision": self.decision.as_str(),
        }))
    }
}

/* neira:meta
id: NEI-20270615-lymphatic-duplicate-event
intent: feature
summary: Добавлено событие LymphaticDuplicateFound для фиксации дубликатов функций.
*/
pub struct LymphaticDuplicateFound {
    pub gene_id: String,
    pub location: PathBuf,
    pub similarity: f32,
    pub decision: LymphaticDecision,
}

impl Event for LymphaticDuplicateFound {
    fn name(&self) -> &str {
        "lymphatic.duplicate_found"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn data(&self) -> Option<Value> {
        Some(json!({
            "gene_id": self.gene_id,
            "location": self.location.to_string_lossy(),
            "similarity": self.similarity,
            "decision": self.decision.as_str(),
        }))
    }
}
