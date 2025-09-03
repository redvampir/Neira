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
/* neira:meta
id: NEI-20240725-brain-local-dispatch
intent: bugfix
summary: Задачи ставятся локально и сразу отправляются в клетку анализа без повторной переотправки.
*/
/* neira:meta
id: NEI-20240728-brain-loop-local-event
intent: bugfix
summary: События из DataFlowController публикуются локально без повторной отправки.
*/
use std::any::Any;
use std::sync::{Arc, RwLock};

use tokio::sync::mpsc::UnboundedReceiver;
use tokio_util::sync::CancellationToken;
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
                event_bus.publish_local(&event);
            }
            FlowMessage::Task { id, payload } => {
                info!(task_id = %id, "получена задача");
                if registry.get_analysis_cell(&id).is_some() {
                    if let Some((task_id, input)) = scheduler
                        .write()
                        .unwrap()
                        .enqueue_local(
                            Queue::Standard,
                            id.clone(),
                            payload,
                            Priority::Low,
                            None,
                            vec![id.clone()],
                        )
                    {
                        if let Some(cell) = registry.get_analysis_cell(&task_id) {
                            let token = CancellationToken::new();
                            cell.analyze(&input, &token);
                        } else {
                            warn!(task_id = %task_id, "клетка не найдена");
                        }
                    }
                } else {
                    warn!(task_id = %id, "клетка не найдена");
                }
            }
        }
    }
}
