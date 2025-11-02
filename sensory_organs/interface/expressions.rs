pub struct ExpressionSystem {
    avatar_controller: Arc<AvatarController>,
    emotion_analyzer: Arc<EmotionAnalyzer>,
    gesture_engine: Arc<GestureEngine>,
}

impl ExpressionSystem {
    pub async fn express_emotion(&self, emotion: &Emotion) -> Result<(), String> {
        // Создаем выражение лица
        let expression = self.emotion_analyzer
            .create_facial_expression(emotion)
            .await?;
            
        // Подбираем подходящие жесты
        let gestures = self.gesture_engine
            .match_emotional_gestures(emotion)
            .await?;
            
        // Применяем анимацию
        self.avatar_controller
            .animate_expression(expression, gestures)
            .await
    }

    async fn create_reaction_chain(&self, stimulus: &Stimulus) -> Vec<Expression> {
        // Создаем цепочку естественных реакций
        let base_emotion = self.emotion_analyzer.analyze(stimulus).await;
        
        vec![
            Expression::MicroExpression(base_emotion.clone()),
            Expression::MainReaction(base_emotion),
            Expression::ReturnToNeutral,
        ]
    }
}
