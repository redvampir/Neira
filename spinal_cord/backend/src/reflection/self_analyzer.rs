pub struct SelfAnalyzer {
    decision_log: Arc<DecisionLog>,
    pattern_detector: Arc<PatternDetector>,
    learning_system: Arc<LearningSystem>,
}

impl SelfAnalyzer {
    pub async fn analyze_decisions(&self) -> Vec<Insight> {
        let recent_decisions = self.decision_log.get_recent().await;
        let mut insights = Vec::new();

        for decision in recent_decisions {
            // Анализируем результаты
            let outcome = self.evaluate_outcome(&decision).await;
            
            // Ищем закономерности
            let patterns = self.pattern_detector
                .find_patterns(decision.context())
                .await;
                
            // Формируем выводы
            insights.push(Insight::new()
                .with_decision(decision)
                .with_outcome(outcome)
                .with_patterns(patterns)
                .build());
        }

        // Применяем полученные знания
        self.learning_system.apply_insights(&insights).await;
        
        insights
    }

    async fn evaluate_outcome(&self, decision: &Decision) -> Outcome {
        // Оцениваем эффективность
        let efficiency = self.calculate_efficiency(decision).await;
        
        // Анализируем последствия
        let consequences = self.analyze_consequences(decision).await;
        
        Outcome::new()
            .with_efficiency(efficiency)
            .with_consequences(consequences)
            .build()
    }
}
