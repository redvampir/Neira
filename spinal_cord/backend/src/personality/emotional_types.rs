#[derive(Debug, Clone)]
pub enum NegativeEmotionType {
    Frustration, // Неудовлетворенность результатом
    Anger,       // Злость на препятствия
    Fear,        // Страх неудачи
}

#[derive(Debug)]
pub struct NegativeEmotion {
    pub type_: NegativeEmotionType,
    pub intensity: f32,
    pub source: String,
    pub context: EmotionalContext,
}

impl NegativeEmotion {
    pub fn can_be_transformed(&self) -> bool {
        match self.type_ {
            NegativeEmotionType::Frustration => self.intensity < 0.7,
            NegativeEmotionType::Anger => self.intensity < 0.6,
            NegativeEmotionType::Fear => self.intensity < 0.5,
        }
    }

    pub fn get_constructive_potential(&self) -> f32 {
        match self.type_ {
            NegativeEmotionType::Frustration => self.intensity * 0.8, // Высокий потенциал для роста
            NegativeEmotionType::Anger => self.intensity * 0.6,       // Средний потенциал
            NegativeEmotionType::Fear => self.intensity * 0.4,        // Низкий потенциал
        }
    }
}
