/* neira:meta
id: NEI-20240709-183230-server-localization
intent: feature
summary: |
  Переводит пользовательские сообщения и контролы в интерфейсе сервера на русский язык.
*/
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::post,
    Router, Server,
};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;

use crate::training::metrics::LearningMetrics;

pub mod analysis;
pub mod chat;
pub mod metrics;
pub mod training;

#[derive(Debug)]
pub struct MetricsServer {
    metrics: Arc<LearningMetrics>,
    addr: SocketAddr,
}

impl MetricsServer {
    pub fn new(metrics: Arc<LearningMetrics>, addr: SocketAddr) -> Self {
        Self { metrics, addr }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cors = CorsLayer::new()
            .allow_origin("*".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([CONTENT_TYPE]);

        let app = Router::new()
            .nest("/metrics", metrics::router())
            .nest("/training", training::router())
            .route("/chat", post(chat::handle_message))
            .nest_service("/", ServeDir::new("static"))
            .with_state(self.metrics.clone())
            .layer(cors);

        info!("Сервер метрик стартует по адресу {}", self.addr);

        Server::bind(&self.addr)
            .serve(app.into_make_service())
            .await?;
        Ok(())
    }
}
