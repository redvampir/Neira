use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, Semaphore};
use tracing::{error, info, warn};

mod metrics;
use crate::organogenesis::Organogenesis;
use metrics::LearningMetrics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningTask {
    pub query: String,
    pub priority: TaskPriority,
    pub max_duration: std::time::Duration,
    pub required_confidence: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

pub struct SelfLearning {
    config: Arc<LearningConfig>,
    metrics: Arc<LearningMetrics>,
    tasks: RwLock<Vec<LearningTask>>,
    concurrent_tasks: Semaphore,
    organogenesis: Organogenesis, // Добавлено поле для органогенеза
}

impl SelfLearning {
    pub fn new(config: LearningConfig) -> Self {
        Self {
            config: Arc::new(config),
            metrics: Arc::new(LearningMetrics::new()),
            tasks: RwLock::new(Vec::new()),
            concurrent_tasks: Semaphore::new(3), // Настраиваемое значение
            organogenesis: Organogenesis::new(), // Инициализация органогенеза
        }
    }

    pub async fn schedule_learning(&self, task: LearningTask) {
        let mut tasks = self.tasks.write().await;
        tasks.push(task);
        tasks.sort_by_key(|t| std::cmp::Reverse(t.priority));
    }

    pub async fn run_learning_cycle(&self) {
        let permit = self.concurrent_tasks.acquire().await.unwrap();
        let task = self.get_next_task().await;

        if let Some(task) = task {
            let metrics = self.metrics.clone();
            let config = self.config.clone();

            tokio::spawn(async move {
                info!("Начало обучения по запросу: {}", task.query);
                let start = std::time::Instant::now();

                if let Err(e) = Self::learn_task(&task, &config).await {
                    error!("Ошибка обучения: {}", e);
                    metrics.record_failure().await;
                } else {
                    metrics.record_success(start.elapsed()).await;
                }

                drop(permit);
            });
        }
    }

    async fn learn_task(
        task: &LearningTask,
        config: &LearningConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let timeout = tokio::time::sleep(task.max_duration);
        tokio::pin!(timeout);

        tokio::select! {
            _ = &mut timeout => {
                warn!("Превышено время обучения для запроса: {}", task.query);
                Ok(())
            }
            result = Self::perform_learning(task, config) => {
                result
            }
        }
    }

    async fn perform_learning(
        task: &LearningTask,
        _config: &LearningConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Реализовать:
        // 1. Поиск релевантных данных
        // 2. Применение различных стратегий обучения
        // 3. Валидация результатов
        // 4. Сохранение новых знаний
        Ok(())
    }

    async fn get_next_task(&self) -> Option<LearningTask> {
        let mut tasks = self.tasks.write().await;
        tasks.pop()
    }

    async fn check_required_organs(
        &self,
        task: &LearningTask,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let required_capabilities = self.analyze_required_capabilities(task).await;

        for capability in required_capabilities {
            if !self.has_capability(&capability).await {
                if let Some(blueprint) = self.organogenesis.propose_organ(&capability).await {
                    self.organogenesis.begin_growth(blueprint).await?;
                }
            }
        }

        Ok(())
    }

    async fn analyze_required_capabilities(&self, task: &LearningTask) -> Vec<String> {
        // Анализ необходимых возможностей на основе задачи
        vec![
            "memory_formation".to_string(),
            "pattern_recognition".to_string(),
            format!(
                "{}_processing",
                task.query.split_whitespace().next().unwrap_or("basic")
            ),
        ]
    }
}
