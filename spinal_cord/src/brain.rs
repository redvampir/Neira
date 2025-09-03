/* neira:meta
id: NEI-20260614-brain-loop
intent: code
summary: Обрабатывает сообщения DataFlowController, распределяя события и задачи.
*/
use std::sync::{Arc, RwLock};

use tokio::sync::mpsc::UnboundedReceiver;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::cell_registry::CellRegistry;
use crate::circulatory_system::FlowMessage;
use crate::event_bus::EventBus;
use crate::task_scheduler::TaskScheduler;

/// Главный цикл мозга: потребляет сообщения из общего канала и реагирует на них
pub async fn brain_loop(
    mut df_rx: UnboundedReceiver<FlowMessage>,
    registry: Arc<CellRegistry>,
    _scheduler: Arc<RwLock<TaskScheduler>>,
    _event_bus: Arc<EventBus>,
) {
    while let Some(msg) = df_rx.recv().await {
        match msg {
            FlowMessage::Event(ev) => {
                info!(event = %ev, "получено событие");
            }
            FlowMessage::Task { id, payload } => {
                info!(task_id = %id, "получена задача");
                if let Some(cell) = registry.get_analysis_cell(&id) {
                    let cancel = CancellationToken::new();
                    let _ = cell.analyze(&payload, &cancel);
                } else {
                    warn!(task_id = %id, "клетка не найдена");
                }
            }
        }
    }
}
