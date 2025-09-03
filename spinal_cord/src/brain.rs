/* neira:meta
id: NEI-20260614-brain-loop
intent: code
summary: Обрабатывает сообщения DataFlowController, распределяя события и задачи.
*/
/* neira:meta
id: NEI-20240709-brain-scheduler-eventbus
intent: refactor
summary: Задачи проходят через TaskScheduler, события публикуются в EventBus.
*/
use std::any::Any;
use std::sync::{Arc, RwLock};

use tokio::sync::mpsc::UnboundedReceiver;
use tracing::{info, warn};

use crate::cell_registry::CellRegistry;
use crate::circulatory_system::FlowMessage;
use crate::event_bus::{Event, EventBus};
use crate::task_scheduler::{Priority, Queue, TaskScheduler};

/// Главный цикл мозга: потребляет сообщения из общего канала и реагирует на них
pub async fn brain_loop(
    mut df_rx: UnboundedReceiver<FlowMessage>,
    registry: Arc<CellRegistry>,
    scheduler: Arc<RwLock<TaskScheduler>>,
    event_bus: Arc<EventBus>,
) {
    while let Some(msg) = df_rx.recv().await {
        match msg {
            FlowMessage::Event(ev) => {
                info!(event = %ev, "получено событие");
                #[allow(dead_code)]
                struct BusEvent(String);
                impl Event for BusEvent {
                    fn name(&self) -> &'static str {
                        "FlowEvent"
                    }
                    fn as_any(&self) -> &dyn Any {
                        self
                    }
                }
                let event = BusEvent(ev);
                event_bus.publish(&event);
            }
            FlowMessage::Task { id, payload } => {
                info!(task_id = %id, "получена задача");
                if registry.get_analysis_cell(&id).is_some() {
                    scheduler.write().unwrap().enqueue(
                        Queue::Standard,
                        id.clone(),
                        payload,
                        Priority::Low,
                        None,
                        vec![id],
                    );
                } else {
                    warn!(task_id = %id, "клетка не найдена");
                }
            }
        }
    }
}
