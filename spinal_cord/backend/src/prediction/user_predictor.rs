use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub struct UserPredictor {
    patterns: RwLock<HashMap<String, Vec<UserActivity>>>,
    memory: Arc<MemoryCell>,
}

impl UserPredictor {
    pub async fn analyze_patterns(&self) -> Vec<PredictedNeed> {
        let time = Utc::now();
        let day_of_week = time.weekday();
        let hour = time.hour();
        
        // Анализируем типичную активность для текущего времени
        let predicted = self.patterns.read().await
            .iter()
            .filter(|(_, activities)| {
                activities.iter().any(|a| {
                    a.day == day_of_week && 
                    (a.hour as i32 - hour as i32).abs() < 2
                })
            })
            .map(|(need, _)| need.clone())
            .collect();

        // Начинаем предварительную подготовку
        self.prepare_resources(predicted).await
    }

    async fn prepare_resources(&self, needs: Vec<String>) -> Vec<PredictedNeed> {
        for need in &needs {
            // Подгружаем данные в память
            self.memory.preload_data(need).await;
            // Запускаем предварительные вычисления
            self.start_preliminary_processing(need).await;
        }
        needs.into_iter()
            .map(|n| PredictedNeed::new(n))
            .collect()
    }
}
