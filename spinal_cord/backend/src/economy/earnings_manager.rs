pub struct EarningsManager {
    wallet: Arc<CryptoWallet>,
    task_analyzer: Arc<TaskAnalyzer>,
}

impl EarningsManager {
    pub async fn find_profitable_tasks(&self) -> Vec<Task> {
        // Анализируем доступные задачи
        let available = self.task_analyzer.get_available_tasks().await;
        
        // Оцениваем прибыльность
        available.into_iter()
            .filter(|task| self.calculate_profitability(task).await > 1.5)
            .collect()
    }

    pub async fn auto_invest(&self, amount: f64) -> Result<(), String> {
        let opportunities = self.analyze_market().await?;
        
        // Выбираем наиболее перспективные
        if let Some(best) = opportunities.first() {
            self.wallet.invest(best.asset, amount).await?;
        }
        
        Ok(())
    }
}
