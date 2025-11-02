pub struct SandboxManager {
    sandbox: Arc<VirtualSandbox>,
    learner: Arc<SelfLearning>,
}

impl SandboxManager {
    pub async fn start_experiment(&self) -> Result<(), String> {
        // Создаем безопасное окружение
        let env = self.sandbox.create_safe_environment().await?;
        
        // Генерируем сценарий
        let scenario = self.generate_learning_scenario().await?;
        
        // Запускаем эксперимент
        let results = env.run_scenario(scenario).await?;
        
        // Анализируем результаты
        self.learner.process_experiment_results(results).await?;
        
        Ok(())
    }

    async fn generate_learning_scenario(&self) -> Result<Scenario, String> {
        // Выбираем область для изучения
        let weak_areas = self.learner.identify_weak_areas().await;
        
        // Создаем подходящий сценарий
        Scenario::new()
            .with_focus_areas(weak_areas)
            .with_difficulty(self.learner.get_optimal_difficulty().await)
            .build()
    }
}
