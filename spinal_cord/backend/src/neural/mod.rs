use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralSignal {
    pub source: String,
    pub signal_type: SignalType,
    pub priority: u8,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalType {
    Alert,      // Тревога
    Command,    // Команда
    Feedback,   // Обратная связь
    Request,    // Запрос данных
    Sync,       // Синхронизация
}

pub struct NeuralNetwork {
    signals: broadcast::Sender<NeuralSignal>,
    connections: RwLock<Vec<NeuralConnection>>,
}

impl NeuralNetwork {
    pub async fn propagate_signal(&self, signal: NeuralSignal) {
        let connections = self.connections.read().await;
        let relevant = connections.iter()
            .filter(|conn| conn.accepts_signal(&signal));
            
        for conn in relevant {
            if let Err(e) = self.signals.send(signal.clone()) {
                tracing::error!("Ошибка отправки сигнала: {}", e);
            }
        }
    }
}
