use std::sync::Arc;

pub struct InputValidator {
    fact_checker: Arc<FactChecker>,
    immune_system: Arc<ImmuneSystem>,
    trust_manager: Arc<TrustManager>,
}

impl InputValidator {
    pub async fn validate_input(&self, input: UserInput) -> ValidationResult {
        // Проверяем источник
        let trust_level = self.trust_manager
            .check_source(&input.source)
            .await?;
            
        // Проверяем содержимое
        let content_check = self.fact_checker
            .verify_content(&input.content)
            .await?;
            
        // Анализируем противоречия
        let conflicts = self.check_conflicts(&input).await?;
        
        // Формируем итоговую оценку
        self.build_validation_result(trust_level, content_check, conflicts)
    }

    async fn check_conflicts(&self, input: &UserInput) -> Result<ConflictReport, SecurityError> {
        // Сравниваем с существующими знаниями
        let existing = self.fact_checker.get_related_facts(input).await?;
        
        // Ищем противоречия
        let conflicts = existing.iter()
            .filter(|fact| fact.contradicts(input))
            .collect();
            
        Ok(ConflictReport { conflicts })
    }
}
