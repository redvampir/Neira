/* neira:meta
id: NEI-20240709-175200-bind-addr
intent: feature
summary: |
  Добавил поддержку NEIRA_BIND_ADDR и перевёл системные журналы на русский язык.
env:
  - NEIRA_BIND_ADDR
risks: low
*/
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{info, warn, Level};

use neira::{server::MetricsServer, training::metrics::LearningMetrics};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Запускаем Нейру...");

    let metrics = Arc::new(LearningMetrics::new());

    let _autopilot = metrics.enable_autopilot().await;
    info!("Автопилот активирован");

    let metrics_clone = metrics.clone();
    tokio::spawn(async move {
        run_demo(metrics_clone).await;
    });

    let addr = resolve_bind_addr();
    info!(%addr, "Сервер метрик и диалога привязан к адресу");
    let server = MetricsServer::new(metrics.clone(), addr);
    server.run().await?;

    Ok(())
}

fn resolve_bind_addr() -> SocketAddr {
    let default_addr = SocketAddr::from(([127, 0, 0, 1], 9090));
    match std::env::var("NEIRA_BIND_ADDR") {
        Ok(raw) => raw.parse().unwrap_or_else(|error| {
            warn!(
                address = raw.as_str(),
                %error,
                "Некорректное значение NEIRA_BIND_ADDR, возвращаемся к {default_addr}"
            );
            default_addr
        }),
        Err(_) => default_addr,
    }
}

async fn run_demo(metrics: Arc<LearningMetrics>) {
    for attempt in 0..10 {
        let success = attempt % 3 != 0;
        metrics.record_attempt(success, 0.5).await;

        let difficulty = metrics.adjust_difficulty().await;
        info!(
            attempt,
            success,
            %difficulty,
            "Ход обучения"
        );

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
