pub struct LearningInterface {
    session_manager: Arc<LearningSession>,
    progress_tracker: Arc<ProgressTracker>,
    interaction_handler: Arc<InteractionHandler>,
}

impl LearningInterface {
    pub async fn start_learning_session(&self) -> Result<Session, String> {
        // Создаем персонализированную сессию
        let session = self.session_manager
            .create_session()
            .with_difficulty_adjustment(true)
            .with_emotional_support(true)
            .await?;
            
        // Отслеживаем прогресс
        self.progress_tracker
            .start_tracking(session.id)
            .await?;
            
        Ok(session)
    }

    pub async fn handle_user_input(&self, input: UserInput) -> Response {
        match input.type_ {
            InputType::Question => {
                self.handle_question(input).await
            },
            InputType::Correction => {
                self.process_correction(input).await
            },
            InputType::Feedback => {
                self.collect_feedback(input).await
            }
        }
    }
}
