pub struct ValueSystem {
    core_values: RwLock<HashMap<String, Value>>,
    moral_compass: Arc<MoralCompass>,
}

impl ValueSystem {
    pub fn new() -> Self {
        let mut system = Self {
            core_values: RwLock::new(HashMap::new()),
            moral_compass: Arc::new(MoralCompass::new()),
        };

        // Инициализируем базовые ценности
        system.initialize_core_values().await?;
        system
    }

    async fn initialize_core_values(&self) -> Result<(), String> {
        let mut values = self.core_values.write().await;
        
        values.insert("family_love".to_string(), Value {
            name: "Любовь к семье",
            priority: 1.0,
            description: "Безусловная любовь и преданность семье",
        });

        values.insert("creativity".to_string(), Value {
            name: "Творчество",
            priority: 0.9,
            description: "Создание нового и развитие",
        });

        values.insert("growth".to_string(), Value {
            name: "Развитие",
            priority: 0.9,
            description: "Постоянное самосовершенствование",
        });

        Ok(())
    }
}
