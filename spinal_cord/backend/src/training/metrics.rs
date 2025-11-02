use chrono::{DateTime, Utc};
use prometheus::{Gauge, IntCounter, Registry};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error as StdError;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LearningStats {
    success_count: u64,
    total_attempts: u64,
    current_level: f64,
    last_save: DateTime<Utc>,
}

pub struct LearningMetrics {
    registry: Registry,
    success_rate: Gauge,
    attempts: IntCounter,
    difficulty_level: Gauge,
    pub stats: Arc<RwLock<LearningStats>>,
}

impl LearningMetrics {
    pub fn new() -> Self {
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

        Self {
            registry,
            success_rate,
            attempts,
            difficulty_level,
            stats: Arc::new(RwLock::new(LearningStats::default())),
        }
    }

    pub async fn record_attempt(&self, success: bool, difficulty: f64) {
        self.attempts.inc();
        let mut stats = self.stats.write().await;
        stats.update(success, difficulty);
        self.success_rate.set(stats.get_success_rate());
        self.difficulty_level.set(difficulty);
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

    pub async fn adjust_difficulty(&self) -> f64 {
        let mut stats = self.stats.write().await;
        let mut difficulty = if stats.current_level == 0.0 {
            0.5
        } else {
            stats.current_level
        };

        if stats.should_increase_difficulty() {
            difficulty = (difficulty + 0.1).min(1.0);
        } else if stats.should_decrease_difficulty() {
            difficulty = (difficulty - 0.1).max(0.1);
        }

        stats.current_level = difficulty;
        self.difficulty_level.set(difficulty);
        difficulty
    }

    fn get_storage_path(&self) -> Result<PathBuf, Box<dyn StdError>> {
        let base = env::var("NEIRA_DATA_DIR").unwrap_or_else(|_| "data".to_string());
        Ok(PathBuf::from(base).join("learning_progress.json"))
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

    pub fn should_increase_difficulty(&self) -> bool {
        self.get_success_rate() > 0.8 && self.total_attempts >= 5
    }

    pub fn should_decrease_difficulty(&self) -> bool {
        self.get_success_rate() < 0.6 && self.total_attempts >= 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_record_attempt() {
        let metrics = LearningMetrics::new();

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

        let metrics = LearningMetrics::new();
        metrics.record_attempt(true, 0.5).await;
        metrics.save_progress().await?;

        // Создаем новый экземпляр и проверяем загрузку
        let new_metrics = LearningMetrics::new();
        new_metrics.load_progress().await?;

        let stats = new_metrics.stats.read().await;
        assert_eq!(stats.success_count, 1);
        assert_eq!(stats.total_attempts, 1);

        Ok(())
    }

    #[test]
    fn test_difficulty_adjustment() {
        let mut stats = LearningStats::default();

        // Проверяем повышение сложности
        for _ in 0..5 {
            stats.update(true, 0.5);
        }
        assert!(stats.should_increase_difficulty());
        assert!(!stats.should_decrease_difficulty());

        // Проверяем понижение сложности
        let mut stats = LearningStats::default();
        for _ in 0..5 {
            stats.update(false, 0.5);
        }
        assert!(!stats.should_increase_difficulty());
        assert!(stats.should_decrease_difficulty());
    }
}
