/* neira:meta
id: NEI-20251103-dialogue-integration-tests
intent: test
summary: |
  Интеграционные тесты для EvolvingDialogue — полный цикл с памятью и ростом.
  Тестирует: recall → learn → generate → remember → reflect → grow
*/

#[cfg(test)]
mod dialogue_tests {
    use backend::dialogue::{EvolvingDialogue, UserInput};
    use backend::memory::SemanticMemory;
    use backend::embeddings::EmbeddingsClient;
    use std::sync::Arc;
    use chrono::Utc;

    /// Создать тестовый dialogue с mock embeddings
    fn create_test_dialogue() -> Arc<EvolvingDialogue> {
        let embeddings_client = Arc::new(EmbeddingsClient::new());
        let semantic_memory = Arc::new(SemanticMemory::new(embeddings_client));
        Arc::new(EvolvingDialogue::new(semantic_memory))
    }

    #[tokio::test]
    #[ignore] // Требует running embeddings service
    async fn test_first_dialogue_creates_milestone() {
        let dialogue = create_test_dialogue();
        
        let input = UserInput {
            text: "Привет, Нейра! Как дела?".to_string(),
            session_id: "test_session_1".to_string(),
            timestamp: Utc::now(),
        };

        let response = dialogue.respond_with_growth(input).await;
        assert!(response.is_ok());

        let stats = dialogue.get_growth_stats().await;
        assert_eq!(stats.total_conversations, 1);
        assert_eq!(stats.milestones_count, 1); // FirstConversation milestone
    }

    #[tokio::test]
    #[ignore]
    async fn test_recall_similar_conversations() {
        let dialogue = create_test_dialogue();
        
        // Первый диалог
        let input1 = UserInput {
            text: "Расскажи о Rust".to_string(),
            session_id: "test_session_2".to_string(),
            timestamp: Utc::now(),
        };
        dialogue.respond_with_growth(input1).await.unwrap();

        // Похожий диалог
        let input2 = UserInput {
            text: "Что такое Rust язык программирования?".to_string(),
            session_id: "test_session_2".to_string(),
            timestamp: Utc::now(),
        };
        let response2 = dialogue.respond_with_growth(input2).await.unwrap();

        // Должны быть источники из прошлого
        assert!(!response2.sources.is_empty(), "Should recall similar conversations");
        assert!(response2.confidence > 0.0);
    }

    #[tokio::test]
    #[ignore]
    async fn test_growth_tracking_after_multiple_dialogues() {
        let dialogue = create_test_dialogue();
        
        // 15 диалогов
        for i in 1..=15 {
            let input = UserInput {
                text: format!("Тестовый вопрос номер {}", i),
                session_id: "test_session_3".to_string(),
                timestamp: Utc::now(),
            };
            dialogue.respond_with_growth(input).await.unwrap();
        }

        let stats = dialogue.get_growth_stats().await;
        assert_eq!(stats.total_conversations, 15);
        assert!(stats.milestones_count >= 2); // First + ReachedConversationCount(10)
        assert!(!stats.top_skills.is_empty());
        
        // Проверяем рост навыков
        for skill in &stats.top_skills {
            assert!(skill.level > 0.0);
            assert_eq!(skill.practice_count, 15);
        }
    }

    #[test]
    fn test_growth_stats_initial_state() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dialogue = create_test_dialogue();
            let stats = dialogue.get_growth_stats().await;
            
            assert_eq!(stats.total_conversations, 0);
            assert_eq!(stats.successful_helps, 0);
            assert_eq!(stats.success_rate, 0.0);
            assert_eq!(stats.milestones_count, 0);
        });
    }

    #[test]
    fn test_dialogue_response_structure() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let dialogue = create_test_dialogue();
            
            // Этот тест НЕ требует embeddings service (но respond_with_growth требует)
            // Просто проверяем, что структура создана
            let stats = dialogue.get_growth_stats().await;
            assert!(stats.skills_count == 0); // Пока нет навыков
        });
    }
}
