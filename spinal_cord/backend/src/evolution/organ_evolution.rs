pub struct EvolutionManager {
    stats_collector: Arc<StatsCollector>,
    organ_factory: Arc<OrganFactory>,
}

impl EvolutionManager {
    pub async fn evolve_organ(&self, organ_id: &str) -> Result<(), String> {
        // Анализируем статистику использования
        let stats = self.stats_collector.get_organ_stats(organ_id).await?;
        
        if stats.is_overloaded() {
            // Разделяем перегруженный орган
            self.split_organ(organ_id).await?;
        } else if stats.is_underutilized() {
            // Ищем кандидата для слияния
            if let Some(partner) = self.find_merge_candidate(organ_id).await {
                self.merge_organs(organ_id, &partner).await?;
            }
        }
        
        Ok(())
    }

    async fn split_organ(&self, organ_id: &str) -> Result<(), String> {
        info!("Начинаю разделение органа: {}", organ_id);
        
        // Создаем два новых специализированных органа
        let (organ1, organ2) = self.organ_factory
            .create_specialized_pair(organ_id)
            .await?;
            
        // Перенаправляем нагрузку
        self.redistribute_load(organ_id, &organ1.id, &organ2.id).await?;
        
        Ok(())
    }
}
