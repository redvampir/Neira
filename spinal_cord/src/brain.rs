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
id: NEI-20240731-brain-requeue-pipeline
intent: fix
summary: Перезапущенные задачи извлекаются из очереди и сохраняют результаты в MemoryCell.
*/
use std::any::Any;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use tokio::sync::mpsc::UnboundedReceiver;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::action::metrics_collector_cell::{MetricsCollectorCell, MetricsRecord};
use crate::cell_registry::CellRegistry;
use crate::circulatory_system::FlowMessage;
use crate::event_bus::{Event, EventBus};
use crate::memory_cell::MemoryCell;
use crate::task_scheduler::TaskScheduler;

/// Главный цикл мозга: потребляет сообщения из общего канала и реагирует на них
pub async fn brain_loop(
    mut df_rx: UnboundedReceiver<FlowMessage>,
    registry: Arc<CellRegistry>,
    scheduler: Arc<RwLock<TaskScheduler>>,
    event_bus: Arc<EventBus>,
    memory: Arc<MemoryCell>,
    metrics: Arc<MetricsCollectorCell>,
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
            FlowMessage::Task { id, payload: _ } => {
                info!(task_id = %id, "получена задача");
                let next_task = { scheduler.write().unwrap().next() };
                if let Some((task_id, input)) = next_task {
                    if let Some(cell) = registry.get_analysis_cell(&task_id) {
                        let token = CancellationToken::new();
                        let start = Instant::now();
                        let cell_cloned = cell.clone();
                        let input_cloned = input.clone();
                        let token_cloned = token.clone();
                        let result = tokio::task::spawn_blocking(move || {
                            cell_cloned.analyze(&input_cloned, &token_cloned)
                        })
                        .await
                        .unwrap();
                        memory.push_metrics(&result);
                        metrics.record(MetricsRecord {
                            id: result.id.clone(),
                            metrics: result.quality_metrics.clone(),
                        });
                        let elapsed = start.elapsed().as_millis();
                        memory.update_time(&task_id, elapsed);
                        let mem = memory.clone();
                        let rid = task_id.clone();
                        mem.recalc_priority_async(rid);
                        memory.save_checkpoint(&task_id, &result);
                    } else {
                        warn!(task_id = %task_id, "клетка не найдена");
                    }
                } else {
                    warn!(task_id = %id, "планировщик пуст");
                }
            }
        }
    }
}
