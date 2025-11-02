use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub words: Vec<String>,
    pub difficulty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    pub min_words: usize,
    pub max_words: usize,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            min_words: 5,
            max_words: 20,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ошибка метрик: {0}")]
    Metrics(String),
    #[error("Ошибка планировщика: {0}")]
    Scheduler(String),
}
