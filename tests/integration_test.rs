use neira::{server::MetricsServer, training::metrics::LearningMetrics};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::test]
async fn test_full_system() {
    // Инициализация
    let metrics = Arc::new(LearningMetrics::new());
    let _autopilot = metrics.enable_autopilot().await;

    // Имитация обучения
    for _ in 0..10 {
        metrics.record_attempt(true, 0.5).await;
    }

    // Проверка адаптации
    let new_difficulty = metrics.adjust_difficulty().await;
    assert!(new_difficulty > 0.5, "Сложность должна увеличиться");

    // Проверка метрик
    let gathered = metrics.get_metrics();
    assert!(!gathered.is_empty(), "Метрики должны быть собраны");

    // Проверка сервера
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let server = MetricsServer::new(metrics.clone(), addr);

    // Запускаем сервер в отдельном потоке
    tokio::spawn(async move {
        server.run().await.unwrap();
    });
}
