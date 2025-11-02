/* neira:meta
id: NEI-20240709-190100-training-endpoint
intent: feature
summary: |
  Добавил endpoint /training/attempt для фиксации попыток и обновления сложности.
*/
use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::training::metrics::LearningMetrics;

#[derive(Debug, Deserialize)]
pub struct AttemptRequest {
    pub success: bool,
    pub difficulty: f64,
}

#[derive(Debug, Serialize)]
pub struct AttemptResponse {
    pub new_difficulty: f64,
}

pub async fn record_attempt(
    State(metrics): State<Arc<LearningMetrics>>,
    Json(payload): Json<AttemptRequest>,
) -> Json<AttemptResponse> {
    metrics
        .record_attempt(payload.success, payload.difficulty)
        .await;
    let new_difficulty = metrics.adjust_difficulty().await;

    Json(AttemptResponse { new_difficulty })
}
