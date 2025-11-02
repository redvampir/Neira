/* neira:meta
id: NEI-20250211-163700-training-metrics-settings
intent: feature
summary: |
  Подключил метрики обучения к файлу конфигурации и добавил обновление параметров.
*/
use crate::autopilot::AutoPilot;
use crate::training::config::{
    config_path_to_string, TrainingConfig, TrainingConfigError, TrainingConfigUpdate,
};
use chrono::{DateTime, Utc};
use prometheus::{Gauge, IntCounter, Registry};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::error::Error as StdError;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LearningStats {
    pub success_count: u64,
    pub total_attempts: u64,
    pub current_level: f64,
    pub last_save: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct LearningMetrics {
    registry: Registry,
    success_rate: Gauge,
    attempts: IntCounter,
    difficulty_level: Gauge,
    pub stats: Arc<RwLock<LearningStats>>,
    config: Arc<RwLock<TrainingConfig>>,
    config_path: PathBuf,
}

#[derive(Debug)]
pub struct PipelineResult {
    pub value: Value,
}

pub trait Pipeline {
    fn get_success(&self) -> bool;
    fn get_difficulty(&self) -> Option<f64>;
}

impl Pipeline for PipelineResult {
    fn get_success(&self) -> bool {
        self.value
            .get("success")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    fn get_difficulty(&self) -> Option<f64> {
        self.value.get("difficulty").and_then(|v| v.as_f64())
    }
}

impl LearningMetrics {
    pub fn new() -> Self {
        let config_path = TrainingConfig::resolve_path();
        Self::with_config_path(config_path)
    }

    pub fn with_config_path(config_path: PathBuf) -> Self {
        let registry = Registry::new();
        let success_rate = Gauge::new("learning_success_rate", "Процент успешных ответов").unwrap();
        let attempts = IntCounter::new("learning_attempts", "Количество попыток").unwrap();
        let difficulty_level =
            Gauge::new("current_difficulty", "Текущий уровень сложности").unwrap();

        registry.register(Box::new(success_rate.clone())).unwrap();
        registry.register(Box::new(attempts.clone())).unwrap();
        registry
            .register(Box::new(difficulty_level.clone()))
            .unwrap();
        let config = match TrainingConfig::load_or_default(&config_path) {
            Ok(cfg) => cfg,
            Err(error) => {
                warn!(
                    path = config_path_to_string(&config_path),
                    error = %error,
                    "Не удалось загрузить конфигурацию обучения, используем значения по умолчанию"
                );
                TrainingConfig::default()
            }
        };

        Self {
            registry,
            success_rate,
            attempts,
            difficulty_level,
            stats: Arc::new(RwLock::new(LearningStats::default())),
            config: Arc::new(RwLock::new(config)),
            config_path,
        }
    }

    pub async fn record_attempt(&self, success: bool, difficulty: f64) {
        self.attempts.inc();
        let mut stats = self.stats.write().await;
        stats.update(success, difficulty);
        self.success_rate.set(stats.get_success_rate());
        self.difficulty_level.set(difficulty);

        info!(
            success = success,
            difficulty = difficulty,
            total = stats.total_attempts,
            "Recorded learning attempt"
        );
    }

    pub async fn save_progress(&self) -> Result<(), Box<dyn StdError>> {
        let stats = self.stats.read().await;
        let path = self.get_storage_path()?;

        tokio::fs::write(path, serde_json::to_string_pretty(&*stats)?).await?;

        Ok(())
    }

    pub async fn load_progress(&self) -> Result<(), Box<dyn StdError>> {
        let path = self.get_storage_path()?;
        if path.exists() {
            let data = tokio::fs::read_to_string(path).await?;
            let loaded: LearningStats = serde_json::from_str(&data)?;
            *self.stats.write().await = loaded;
        }
        Ok(())
    }

    fn get_storage_path(&self) -> Result<PathBuf, Box<dyn StdError>> {
        let base = env::var("NEIRA_DATA_DIR").unwrap_or_else(|_| "data".to_string());
        Ok(PathBuf::from(base).join("learning_progress.json"))
    }

    pub fn get_metrics(&self) -> Vec<prometheus::proto::MetricFamily> {
        self.registry.gather()
    }

    pub fn get_success_rate_metric(&self) -> f64 {
        self.success_rate.get()
    }

    pub fn get_attempts_metric(&self) -> u64 {
        self.attempts.get()
    }

    pub async fn adjust_difficulty(&self) -> f64 {
        let stats = self.stats.read().await;
        let config = self.config.read().await.clone();
        let current = stats.current_level;
        let new_level = if stats.should_increase_difficulty(&config) {
            let level = (current + 0.1).min(1.0);
            info!(from = current, to = level, "Increasing difficulty");
            level
        } else if stats.should_decrease_difficulty(&config) {
            let level = (current - 0.1).max(0.1);
            warn!(from = current, to = level, "Decreasing difficulty");
            level
        } else {
            current
        };
        new_level
    }

    pub async fn process_result<T: Pipeline>(&self, result: &T) -> Result<(), Box<dyn StdError>> {
        let success = result.get_success();
        let stats = self.stats.read().await;
        let difficulty = result.get_difficulty().unwrap_or(stats.current_level);

        drop(stats); // Освобождаем лок перед record_attempt
        self.record_attempt(success, difficulty).await;
        Ok(())
    }

    pub async fn enable_autopilot(&self) -> Arc<AutoPilot> {
        let metrics = Arc::new(self.clone());
        let pilot = AutoPilot::new(metrics);
        pilot.start().await;
        Arc::new(pilot)
    }

    pub async fn get_current_level(&self) -> f64 {
        self.stats.read().await.current_level
    }

    pub async fn get_training_config(&self) -> TrainingConfig {
        self.config.read().await.clone()
    }

    pub async fn update_training_config(
        &self,
        update: TrainingConfigUpdate,
    ) -> Result<TrainingConfig, TrainingConfigError> {
        let mut guard = self.config.write().await;
        let mut candidate = guard.clone();
        candidate.apply_update(update);
        candidate.validate()?;
        candidate.save_to_path(&self.config_path)?;
        *guard = candidate.clone();
        Ok(candidate)
    }
}

impl LearningStats {
    pub fn update(&mut self, success: bool, difficulty: f64) {
        if success {
            self.success_count += 1;
        }
        self.total_attempts += 1;
        self.current_level = difficulty;
        self.last_save = Utc::now();
    }

    pub fn get_success_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            return 0.0;
        }
        self.success_count as f64 / self.total_attempts as f64
    }

    pub fn should_increase_difficulty(&self, config: &TrainingConfig) -> bool {
        self.get_success_rate() > config.success_threshold && self.total_attempts >= config.min_attempts
    }

    pub fn should_decrease_difficulty(&self, config: &TrainingConfig) -> bool {
        self.get_success_rate() < config.failure_threshold && self.total_attempts >= config.min_attempts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_record_attempt() {
        let (_dir, _path, metrics) = metrics_with_temp_config();

        // Проверяем успешную попытку
        metrics.record_attempt(true, 0.5).await;
        let stats = metrics.stats.read().await;
        assert_eq!(stats.success_count, 1);
        assert_eq!(stats.total_attempts, 1);

        // Проверяем неуспешную попытку
        drop(stats);
        metrics.record_attempt(false, 0.6).await;
        let stats = metrics.stats.read().await;
        assert_eq!(stats.success_count, 1);
        assert_eq!(stats.total_attempts, 2);
    }

    #[tokio::test]
    async fn test_save_and_load_progress() -> Result<(), Box<dyn StdError>> {
        // Создаем временную директорию для тестов
        let temp_dir = TempDir::new()?;
        env::set_var("NEIRA_DATA_DIR", temp_dir.path());

        let (_config_guard, _path, metrics) = metrics_with_temp_config();
        metrics.record_attempt(true, 0.5).await;
        metrics.save_progress().await?;

        // Создаем новый экземпляр и проверяем загрузку
        let (_config_guard2, _path, new_metrics) = metrics_with_temp_config();
        new_metrics.load_progress().await?;

        let stats = new_metrics.stats.read().await;
        assert_eq!(stats.success_count, 1);
        assert_eq!(stats.total_attempts, 1);

        Ok(())
    }

    #[test]
    fn test_difficulty_adjustment() {
        let mut stats = LearningStats::default();
        let config = TrainingConfig::default();

        // Проверяем повышение сложности
        for _ in 0..5 {
            stats.update(true, 0.5);
        }
        assert!(stats.should_increase_difficulty(&config));
        assert!(!stats.should_decrease_difficulty(&config));

        // Проверяем понижение сложности
        let mut stats = LearningStats::default();
        for _ in 0..5 {
            stats.update(false, 0.5);
        }
        assert!(!stats.should_increase_difficulty(&config));
        assert!(stats.should_decrease_difficulty(&config));
    }

    #[test]
    fn test_metrics_export() {
        let (_guard, _path, metrics) = metrics_with_temp_config();

        // Проверяем начальные значения
        assert_eq!(metrics.get_success_rate_metric(), 0.0);
        assert_eq!(metrics.get_attempts_metric(), 0);

        // Проверяем что метрики собираются
        let gathered = metrics.get_metrics();
        assert!(!gathered.is_empty());
    }

    #[tokio::test]
    async fn test_adaptive_difficulty() {
        let (_guard, _path, metrics) = metrics_with_temp_config();

        // Проверяем повышение сложности после успешных попыток
        for _ in 0..5 {
            metrics.record_attempt(true, 0.5).await;
        }
        assert!(metrics.adjust_difficulty().await > 0.5);

        // Проверяем понижение сложности после неудач
        let (_guard2, _path, metrics) = metrics_with_temp_config();
        for _ in 0..5 {
            metrics.record_attempt(false, 0.5).await;
        }
        assert!(metrics.adjust_difficulty().await < 0.5);
    }

    #[tokio::test]
    async fn test_process_pipeline_result() {
        let (_guard, _path, metrics) = metrics_with_temp_config();

        let result = PipelineResult {
            value: serde_json::json!({
                "success": true,
                "difficulty": 0.7
            }),
        };

        metrics.process_result(&result).await.unwrap();

        let stats = metrics.stats.read().await;
        assert_eq!(stats.success_count, 1);
        assert_eq!(stats.current_level, 0.7);
    }

    #[tokio::test]
    async fn test_update_training_config_persists_changes() {
        let (temp_dir, config_path, metrics) = metrics_with_temp_config();

        let updated = metrics
            .update_training_config(TrainingConfigUpdate {
                success_threshold: Some(0.9),
                failure_threshold: Some(0.7),
                min_attempts: Some(12),
                data_dir: Some(temp_dir.path().join("custom")),
            })
            .await
            .unwrap();

        assert_eq!(updated.success_threshold, 0.9);
        assert_eq!(updated.failure_threshold, 0.7);
        assert_eq!(updated.min_attempts, 12);
        assert_eq!(updated.data_dir, temp_dir.path().join("custom"));

        let persisted = TrainingConfig::load_or_default(&config_path).unwrap();
        assert_eq!(persisted.success_threshold, 0.9);
    }

    #[tokio::test]
    async fn test_update_training_config_rolls_back_on_error() {
        let (_temp_dir, config_path, metrics) = metrics_with_temp_config();

        let original = metrics.get_training_config().await;
        let result = metrics
            .update_training_config(TrainingConfigUpdate {
                success_threshold: Some(0.5),
                failure_threshold: Some(0.6),
                ..TrainingConfigUpdate::default()
            })
            .await;

        assert!(matches!(result, Err(TrainingConfigError::Validation(_))));

        let current = metrics.get_training_config().await;
        assert_eq!(current.success_threshold, original.success_threshold);
        assert_eq!(current.failure_threshold, original.failure_threshold);
        assert_eq!(current.min_attempts, original.min_attempts);
        assert_eq!(current.data_dir, original.data_dir);
        assert!(!config_path.exists());
    }

    fn metrics_with_temp_config() -> (TempDir, PathBuf, LearningMetrics) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let metrics = LearningMetrics::with_config_path(config_path.clone());
        (temp_dir, config_path, metrics)
    }
}
