/* neira:meta
id: NEI-20240709-183230-server-localization
intent: feature
summary: |
  Переводит пользовательские сообщения и контролы в интерфейсе сервера на русский язык.
*/
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::post,
    Router,
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    services::ServeDir,
};
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
            .allow_origin(AllowOrigin::any())
            .allow_methods(AllowMethods::list([Method::GET, Method::POST]))
            .allow_headers(AllowHeaders::list([CONTENT_TYPE]));

        let app = Router::new()
            .nest("/metrics", metrics::router())
            .nest("/training", training::router())
            .route("/chat", post(chat::handle_message))
            .nest_service("/", ServeDir::new("static"))
            .with_state(self.metrics.clone())
            .layer(cors);

        let listener = TcpListener::bind(self.addr).await?;
        let bound_addr = listener.local_addr()?;

        info!("Сервер метрик стартует по адресу {}", bound_addr);

        axum::serve(listener, app).await?;
        Ok(())
    }
}
