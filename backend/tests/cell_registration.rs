/* neira:meta
id: NEI-20240607-cell-registration-test
intent: test
summary: \
  Проверяет, что при создании записи клетки нервная и иммунная системы получают уведомления.
*/

use backend::cell_template::{CellTemplate, Metadata};
use backend::event_bus::{CellCreated, Event, EventBus, Subscriber};
use backend::factory::StemCellFactory;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

struct MockSubscriber {
    flag: Arc<AtomicBool>,
}

impl Subscriber for MockSubscriber {
    fn on_event(&self, event: &dyn Event) {
        if event.as_any().downcast_ref::<CellCreated>().is_some() {
            self.flag.store(true, Ordering::SeqCst);
        }
    }
}

#[test]
fn cell_registration_notifies_subsystems() {
    let bus = EventBus::new();
    let nervous_flag = Arc::new(AtomicBool::new(false));
    let immune_flag = Arc::new(AtomicBool::new(false));
    bus.subscribe(Arc::new(MockSubscriber {
        flag: nervous_flag.clone(),
    }));
    bus.subscribe(Arc::new(MockSubscriber {
        flag: immune_flag.clone(),
    }));

    let factory = StemCellFactory::new();
    let tpl = CellTemplate {
        id: "c1".to_string(),
        version: "0.1.0".to_string(),
        analysis_type: "text".to_string(),
        links: vec![],
        confidence_threshold: None,
        draft_content: None,
        metadata: Metadata {
            schema: "1.0.0".to_string(),
            extra: HashMap::new(),
        },
    };
    let record = factory
        .create_record("backend", &tpl)
        .expect("record created");
    bus.publish(&CellCreated { record });

    assert!(nervous_flag.load(Ordering::SeqCst));
    assert!(immune_flag.load(Ordering::SeqCst));
}
