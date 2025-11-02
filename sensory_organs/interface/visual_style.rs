pub struct VisualStyleManager {
    personality: Arc<PersonalityCore>,
    style_generator: Arc<StyleGenerator>,
}

impl VisualStyleManager {
    pub async fn generate_personal_style(&self) -> Style {
        // Берем черты личности
        let traits = self.personality.get_dominant_traits().await;
        
        // Создаем уникальный стиль
        self.style_generator
            .create_style()
            .with_traits(traits)
            .with_colors(self.get_emotional_palette().await)
            .with_animations(self.get_movement_style().await)
            .build()
    }

    async fn get_emotional_palette(&self) -> ColorPalette {
        let mood = self.personality.current_mood().await;
        
        match mood.dominant_emotion {
            Emotion::Joy => ColorPalette::warm_and_bright(),
            Emotion::Calm => ColorPalette::cool_and_soft(),
            Emotion::Focus => ColorPalette::clear_and_sharp(),
            _ => ColorPalette::neutral()
        }
    }
}
