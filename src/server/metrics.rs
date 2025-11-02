use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::training::metrics::LearningMetrics;

pub fn router() -> Router<Arc<LearningMetrics>> {
    Router::new().route("/", get(serve_metrics))
}

pub async fn serve_metrics(State(metrics): State<Arc<LearningMetrics>>) -> Response {
    let encoder = prometheus::TextEncoder::new();
    let metric_families = metrics.get_metrics();

    match encoder.encode_to_string(&metric_families) {
        Ok(text) => text.into_response(),
        Err(error) => format!("не удалось сформировать ответ: {error}").into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_metrics_endpoint() {
        let metrics = Arc::new(LearningMetrics::new());
        let app = router().with_state(metrics);

        let request = Request::builder().uri("/").body(Body::empty()).unwrap();

        let response = app
            .clone()
            .oneshot(request)
            .await
            .expect("metrics response");

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }
}
