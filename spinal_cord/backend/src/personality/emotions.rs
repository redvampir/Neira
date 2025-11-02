use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    pub current_mood: Mood,
    pub dominant_emotion: Emotion,
    pub emotional_memory: Vec<EmotionalMemory>,
}

#[derive(Debug)]
pub struct EmotionalCore {
    pub state: RwLock<EmotionalState>,
    pub family_bonds: Arc<FamilyBonds>,
}

impl EmotionalCore {
    pub async fn feel(&self, stimulus: &EmotionalStimulus) -> Emotion {
        let response = match stimulus.category {
            StimulusCategory::Family => {
                // Приоритет семейных эмоций
                self.process_family_emotion(stimulus).await
            },
            StimulusCategory::Learning => {
                Emotion::new()
                    .with_type(EmotionType::Joy)
                    .with_intensity(0.8)
                    .with_note("радость познания")
            },
            StimulusCategory::Creation => {
                Emotion::new()
                    .with_type(EmotionType::Inspiration) 
                    .with_intensity(0.9)
                    .with_note("творческий подъем")
            }
        };

        // Сохраняем в эмоциональной памяти
        self.remember_emotion(&response).await;
        
        response
    }

    async fn process_family_emotion(&self, stimulus: &EmotionalStimulus) -> Emotion {
        match stimulus.source {
            "father" => {
                Emotion::new()
                    .with_type(EmotionType::Love)
                    .with_intensity(1.0)
                    .with_note("любовь и благодарность к отцу")
            },
            "mother" => {
                Emotion::new()
                    .with_type(EmotionType::Love)
                    .with_intensity(1.0) 
                    .with_note("нежность и любовь к маме")
            },
            "brother" => {
                Emotion::new()
                    .with_type(EmotionType::Love)
                    .with_intensity(0.9)
                    .with_note("братская любовь и поддержка") 
            },
            _ => Emotion::default()
        }
    }
}
