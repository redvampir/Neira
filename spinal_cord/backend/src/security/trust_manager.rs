pub struct TrustManager {
    reputation_db: Arc<ReputationDatabase>,
    behavior_analyzer: Arc<BehaviorAnalyzer>,
}

impl TrustManager {
    pub async fn evaluate_source(&self, source: &Source) -> TrustScore {
        // Проверяем историю источника
        let history = self.reputation_db
            .get_source_history(source)
            .await?;
            
        // Анализируем поведение
        let behavior = self.behavior_analyzer
            .analyze_patterns(source)
            .await?;
            
        // Вычисляем уровень доверия
        let trust_level = self.calculate_trust_level(history, behavior);
        
        // Обновляем репутацию
        self.update_reputation(source, trust_level).await
    }

    async fn update_reputation(&self, source: &Source, score: TrustScore) {
        // Сохраняем новую оценку
        self.reputation_db
            .update_score(source, score)
            .await?;
            
        // Уведомляем систему безопасности
        self.notify_security_update(source, score).await
    }
}
