/* neira:meta
id: NEI-20250226-dataflow-controller
intent: code
summary: Простая шина передачи данных между органами через mpsc.
*/
use std::sync::Arc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

/// Сообщения, циркулирующие между органами
#[derive(Debug, Clone)]
pub enum FlowMessage {
    /// Событие из глобальной шины
    Event(String),
    /// Задача для конкретного органа
    Task { id: String, payload: String },
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
