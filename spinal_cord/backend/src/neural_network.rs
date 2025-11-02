use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkMetrics {
    pub accuracy: f64,
    pub loss: f64,
    pub training_iterations: u64,
}

pub struct NeuralCore {
    weights: RwLock<Vec<f64>>,
    metrics: RwLock<NetworkMetrics>,
}

impl NeuralCore {
    pub fn new() -> Self {
        Self {
            weights: RwLock::new(vec![0.5; 10]), // Начальные веса
            metrics: RwLock::new(NetworkMetrics {
                accuracy: 0.0,
                loss: 1.0,
                training_iterations: 0,
            }),
        }
    }

    pub async fn learn(&self, input: &[f64], expected: &[f64]) -> Result<NetworkMetrics, String> {
        let mut weights = self.weights.write().await;
        let mut metrics = self.metrics.write().await;

        // Простой градиентный спуск
        for (i, &x) in input.iter().enumerate() {
            if i < weights.len() {
                weights[i] += 0.1 * (expected[i] - weights[i] * x);
            }
        }

        metrics.training_iterations += 1;
        metrics.accuracy = 0.8; // Упрощенно для примера
        metrics.loss = 0.2;

        Ok(metrics.clone())
    }
}
