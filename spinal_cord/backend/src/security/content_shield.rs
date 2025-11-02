pub struct ContentShield {
    pattern_detector: Arc<PatternDetector>,
    threat_analyzer: Arc<ThreatAnalyzer>,
    learning_protector: Arc<LearningProtector>,
}

impl ContentShield {
    pub async fn analyze_content(&self, content: &Content) -> SecurityVerdict {
        // Проверяем на известные паттерны атак
        let patterns = self.pattern_detector
            .detect_harmful_patterns(content)
            .await?;
            
        // Анализируем потенциальные угрозы
        let threats = self.threat_analyzer
            .analyze_risks(content)
            .await?;
            
        // Проверяем влияние на обучение
        let learning_impact = self.learning_protector
            .assess_impact(content)
            .await?;
            
        self.make_verdict(patterns, threats, learning_impact)
    }

    async fn make_verdict(
        &self,
        patterns: Vec<Pattern>,
        threats: Vec<Threat>,
        impact: LearningImpact
    ) -> SecurityVerdict {
        match (patterns.is_empty(), threats.is_empty(), impact.is_safe()) {
            (true, true, true) => SecurityVerdict::Safe,
            (false, _, _) => SecurityVerdict::DangerousPatterns(patterns),
            (_, false, _) => SecurityVerdict::PotentialThreats(threats),
            (_, _, false) => SecurityVerdict::UnsafeLearning(impact),
        }
    }
}
