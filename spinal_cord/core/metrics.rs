use std::time::Duration;

pub struct MetricsCollector {
    predictor: Box<dyn MetricsPredictor>,
    history: TimeSeriesDB,
}

impl MetricsCollector {
    /// Автоматически обнаруживает доступные метрики
    pub fn with_autodiscovery() -> Self {
        Self {
            predictor: Box::new(AIPredictor::default()),
            history: TimeSeriesDB::new()
        }
    }

    /// Предсказывает будущие значения метрик
    pub async fn predict_trends(&self, window: Duration) -> Vec<Prediction> {
        self.predictor.forecast(self.history.clone(), window).await
    }

    /// Определяет здоровье системы на основе паттернов
    pub async fn get_system_health(&self) -> Result<SystemHealth, Error> {
        let patterns = self.history.detect_patterns().await?;
        SystemHealth::analyze(patterns)
    }
}
