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
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc::{
    error::TryRecvError, unbounded_channel, UnboundedReceiver, UnboundedSender,
};

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

/// Обёртка над `UnboundedReceiver`, учитывающая полученные сообщения.
pub struct FlowReceiver {
    rx: UnboundedReceiver<FlowMessage>,
    received: Arc<AtomicU64>,
}

impl FlowReceiver {
    pub fn new(rx: UnboundedReceiver<FlowMessage>, counter: Arc<AtomicU64>) -> Self {
        Self {
            rx,
            received: counter,
        }
    }

    /// Получение сообщения с инкрементом счётчика.
    pub async fn recv(&mut self) -> Option<FlowMessage> {
        let msg = self.rx.recv().await;
        if msg.is_some() {
            self.received.fetch_add(1, Ordering::Relaxed);
        }
        msg
    }

    /// Неблокирующее получение сообщения.
    pub fn try_recv(&mut self) -> Result<FlowMessage, TryRecvError> {
        match self.rx.try_recv() {
            Ok(msg) => {
                self.received.fetch_add(1, Ordering::Relaxed);
                Ok(msg)
            }
            Err(e) => Err(e),
        }
    }
}

/// Контроллер потоков данных между органами
pub struct DataFlowController {
    sender: UnboundedSender<FlowMessage>,
    sent: Arc<AtomicU64>,
    received: Arc<AtomicU64>,
}

impl DataFlowController {
    /// Создаёт контроллер и возвращает его вместе с приёмником событий
    pub fn new() -> (Arc<Self>, FlowReceiver) {
        let (tx, rx) = unbounded_channel();
        let sent = Arc::new(AtomicU64::new(0));
        let received = Arc::new(AtomicU64::new(0));
        (
            Arc::new(Self {
                sender: tx,
                sent: sent.clone(),
                received: received.clone(),
            }),
            FlowReceiver::new(rx, received),
        )
    }

    /// Отправка сообщения в общий канал
    pub fn send(&self, msg: FlowMessage) {
        if self.sender.send(msg).is_ok() {
            self.sent.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Количество отправленных сообщений.
    pub fn sent_count(&self) -> u64 {
        self.sent.load(Ordering::Relaxed)
    }

    /// Количество полученных сообщений.
    pub fn received_count(&self) -> u64 {
        self.received.load(Ordering::Relaxed)
    }
}

/* neira:meta
id: NEI-20241003-flow-counters
intent: feat
summary: Учёт отправленных и полученных сообщений через AtomicU64 и обёртку FlowReceiver.
*/
