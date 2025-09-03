/* neira:meta
id: NEI-20250226-dataflow-controller
intent: code
summary: Простая шина передачи данных между органами через mpsc.
*/
/* neira:meta
id: NEI-20240514-flowmessage-serde
intent: refactor
summary: FlowMessage использует типизированные события и payload задач, сериализуемые через serde.
*/
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowEvent {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPayload {
    Text(String),
}

/// Сообщения, циркулирующие между органами
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowMessage {
    /// Событие из глобальной шины
    Event(FlowEvent),
    /// Задача для конкретного органа
    Task { id: String, payload: TaskPayload },
}

/// Контроллер потоков данных между органами
pub struct DataFlowController {
    sender: UnboundedSender<FlowMessage>,
}

impl DataFlowController {
    /// Создаёт контроллер и возвращает его вместе с приёмником событий
    pub fn new() -> (Arc<Self>, UnboundedReceiver<FlowMessage>) {
        let (tx, rx) = unbounded_channel();
        (Arc::new(Self { sender: tx }), rx)
    }

    /// Отправка сообщения в общий канал
    pub fn send(&self, msg: FlowMessage) {
        let _ = self.sender.send(msg);
    }
}
