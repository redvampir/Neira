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

use tokio::sync::{mpsc::UnboundedReceiver, Mutex};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::action::metrics_collector_cell::{MetricsCollectorCell, MetricsRecord};
use crate::analysis_cell::{AnalysisCell, QualityMetrics};
use crate::cell_registry::CellRegistry;
use crate::circulatory_system::{DataFlowController, FlowEvent, FlowMessage, TaskPayload};
use crate::event_bus::{Event, EventBus, Subscriber};
use crate::task_scheduler::{Priority, Queue, TaskScheduler};

/* neira:meta
id: NEI-20240514-brain-flowevent-import
intent: refactor
summary: Реализует Event для FlowEvent из кровотока.
*/

impl Event for FlowEvent {
    fn name(&self) -> &str {
        &self.name
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/* neira:meta
id: NEI-20240606-brain-struct
intent: refactor
summary: Оформлен Brain как структура с методами spawn/run и поддержкой регистрации нейронов.
*/

/// Мозг: получает сообщения из кровотока и активирует специализированные клетки
pub struct Brain {
    df_rx: Mutex<UnboundedReceiver<FlowMessage>>,
    registry: Arc<CellRegistry>,
    scheduler: Arc<RwLock<TaskScheduler>>,
    event_bus: Arc<EventBus>,
    metrics: Arc<MetricsCollectorCell>,
}

impl Brain {
    /// Создаёт новый экземпляр `Brain`
    pub fn new(
        df_rx: UnboundedReceiver<FlowMessage>,
        registry: Arc<CellRegistry>,
        scheduler: Arc<RwLock<TaskScheduler>>,
        event_bus: Arc<EventBus>,
        metrics: Arc<MetricsCollectorCell>,
    ) -> Self {
        Self {
            df_rx: Mutex::new(df_rx),
            registry,
            scheduler,
            event_bus,
            metrics,
        }
    }

    /// Запускает цикл обработки сообщений в отдельной задаче
    pub fn spawn(self: Arc<Self>) {
        tokio::spawn(self.run());
    }

    /// Регистрация специализированной клетки мозга («нейрона»)
    pub fn register_neuron(&self, cell: Arc<dyn AnalysisCell + Send + Sync>) {
        self.registry.register_analysis_cell(cell);
    }

    /// Основной цикл: распределяет события и задачи
    async fn run(self: Arc<Self>) {
        let mut df_rx = self.df_rx.lock().await;
        while let Some(msg) = df_rx.recv().await {
            match msg {
                FlowMessage::Event(event) => {
                    info!(event = %event.name, "получено событие");
                    self.event_bus.publish_local(&event);
                    metrics::counter!("brain_events_processed_total").increment(1);
                    self.metrics.record(MetricsRecord {
                        id: "brain.event".to_string(),
                        metrics: QualityMetrics {
                            credibility: None,
                            recency_days: None,
                            demand: Some(1),
                        },
                    });
                }
                FlowMessage::Task { id, payload } => {
                    info!(task_id = %id, "получена задача");
                    if self.registry.get_analysis_cell(&id).is_some() {
                        let TaskPayload::Text(payload) = payload;
                        if let Some((task_id, input)) =
                            self.scheduler.write().unwrap().enqueue_local(
                                Queue::Standard,
                                id.clone(),
                                payload,
                                Priority::Low,
                                None,
                                vec![id.clone()],
                            )
                        {
                            if let Some(cell) = self.registry.get_analysis_cell(&task_id) {
                                let token = CancellationToken::new();
                                cell.analyze(&input, &token);
                                metrics::counter!("brain_tasks_processed_total").increment(1);
                                self.metrics.record(MetricsRecord {
                                    id: "brain.task".to_string(),
                                    metrics: QualityMetrics {
                                        credibility: None,
                                        recency_days: None,
                                        demand: Some(1),
                                    },
                                });
                            } else {
                                warn!(task_id = %task_id, "клетка не найдена");
                            }
                        } else {
                            warn!(task_id = %id, "клетка не найдена");
                        }
                    } else {
                        warn!(task_id = %id, "клетка не найдена");
                    }
                }
            }
        }
    }
}

/* neira:meta
id: NEI-20240930-brain-subscriber
intent: feat
summary: |-
  Подписчик BrainSubscriber отправляет события в DataFlowController, игнорируя FlowEvent из кровотока.
*/
pub struct BrainSubscriber {
    flow: Arc<DataFlowController>,
}

impl BrainSubscriber {
    pub fn new(flow: Arc<DataFlowController>) -> Self {
        Self { flow }
    }
}

impl Subscriber for BrainSubscriber {
    fn on_event(&self, event: &dyn Event) {
        if !event.as_any().is::<FlowEvent>() {
            self.flow.send(FlowMessage::Event(FlowEvent {
                name: event.name().to_string(),
            }));
        }
    }
}

/* neira:meta
id: NEI-20240821-brain-metrics
intent: feat
summary: Учёт обработанных задач и событий через MetricsCollectorCell и счётчики.
*/
