/* neira:meta
id: NEI-20251227-event-bus
intent: code
summary: |-
  Простой шина событий с трейтом Event и подписчиками.
*/
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
}

impl EventBus {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            subscribers: RwLock::new(Vec::new()),
        })
    }

    pub fn subscribe(&self, sub: Arc<dyn Subscriber>) {
        self.subscribers.write().unwrap().push(sub);
    }

    pub fn publish(&self, event: &dyn Event) {
        for sub in self.subscribers.read().unwrap().iter() {
            sub.on_event(event);
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
