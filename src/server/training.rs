/* neira:meta
id: NEI-20240709-190100-training-endpoint
intent: feature
summary: |
  Добавил endpoint /training/attempt для фиксации попыток и обновления сложности.
*/
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::training::config::{TrainingConfig, TrainingConfigUpdate};
use crate::training::metrics::LearningMetrics;

pub fn router() -> Router<Arc<LearningMetrics>> {
    Router::new()
        .route("/attempt", axum::routing::post(record_attempt))
        .route("/config", get(get_config).put(update_config))
}

#[derive(Debug, Deserialize)]
pub struct AttemptRequest {
    pub success: bool,
    pub difficulty: f64,
}

#[derive(Debug, Serialize)]
pub struct AttemptResponse {
    pub new_difficulty: f64,
}

#[derive(Debug, Serialize)]
pub struct TrainingConfigResponse {
    pub success_threshold: f64,
    pub failure_threshold: f64,
    pub min_attempts: u64,
    pub data_dir: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
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

pub async fn get_config(State(metrics): State<Arc<LearningMetrics>>) -> Json<TrainingConfigResponse> {
    let config = metrics.get_training_config().await;
    Json(TrainingConfigResponse::from(config))
}

pub async fn update_config(
    State(metrics): State<Arc<LearningMetrics>>,
    Json(payload): Json<TrainingConfigUpdate>,
) -> Result<Json<TrainingConfigResponse>, (StatusCode, Json<ErrorResponse>)> {
    match metrics.update_training_config(payload).await {
        Ok(config) => Ok(Json(TrainingConfigResponse::from(config))),
        Err(error) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: error.to_string(),
            }),
        )),
    }
}

impl From<TrainingConfig> for TrainingConfigResponse {
    fn from(value: TrainingConfig) -> Self {
        Self {
            success_threshold: value.success_threshold,
            failure_threshold: value.failure_threshold,
            min_attempts: value.min_attempts,
            data_dir: value.data_dir.to_string_lossy().into_owned(),
        }
    }
}
