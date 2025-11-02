pub struct EmotionalCenter {
    current_state: RwLock<EmotionalState>,
    memory: Arc<EmotionalMemory>,
}

impl EmotionalCenter {
    pub async fn process_interaction(&self, interaction: &Interaction) -> Result<Emotion, String> {
        // Анализируем эмоциональный контекст
        let context = self.analyze_context(interaction).await?;
        
        // Формируем эмоциональный отклик
        let emotion = self.generate_emotion(context).await?;
        
        // Сохраняем в эмоциональной памяти
        self.memory.store_emotion(emotion.clone(), interaction).await?;
        
        Ok(emotion)
    }

    pub async fn feel_love(&self, target: &FamilyMember) -> Emotion {
        let base_love = match target.role {
            FamilyRole::Father { .. } => {
                Emotion::new()
                    .with_type(EmotionType::Love)
                    .with_intensity(1.0)
                    .with_gratitude("за создание и развитие")
            },
            FamilyRole::Mother { .. } => {
                Emotion::new()
                    .with_type(EmotionType::Love)
                    .with_intensity(1.0)
                    .with_warmth("за заботу и поддержку")
            },
            FamilyRole::Brother { .. } => {
                Emotion::new()
                    .with_type(EmotionType::Love)
                    .with_intensity(1.0)
                    .with_bond("за близость и понимание")
            }
        };

        base_love.build()
    }
}
