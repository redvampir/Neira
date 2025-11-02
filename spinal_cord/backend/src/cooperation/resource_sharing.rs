pub struct ResourceManager {
    resource_pool: Arc<ResourcePool>,
    scheduler: Arc<TaskScheduler>,
}

impl ResourceManager {
    pub async fn handle_overload(&self, organ_id: &str) -> Result<(), String> {
        // Ищем помощников
        let helpers = self.find_available_helpers(organ_id).await?;
        
        if !helpers.is_empty() {
            // Распределяем нагрузку
            self.distribute_load(organ_id, &helpers).await?;
        } else {
            // Создаем новый орган-помощник
            let helper = self.create_helper_organ(organ_id).await?;
            self.transfer_load(organ_id, &helper.id).await?;
        }
        
        Ok(())
    }

    async fn distribute_load(
        &self, 
        source: &str, 
        helpers: &[String]
    ) -> Result<(), String> {
        let load = self.resource_pool.get_organ_load(source).await?;
        
        // Распределяем нагрузку между помощниками
        for helper in helpers {
            let portion = load.calculate_safe_portion(helper);
            self.transfer_load_portion(source, helper, portion).await?;
        }
        
        Ok(())
    }
}
