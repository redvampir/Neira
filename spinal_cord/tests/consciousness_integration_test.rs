/* neira:meta
id: NEI-20260131-consciousness-integration-test
intent: test
summary: |
  Интеграционный тест для Фазы 3 (Consciousness).
  Проверяет полный 9-шаговый цикл EvolvingDialogue с MetaCognition:
  recall → learn → understand → generate → assess → remember → reflect → grow → meta-reflect.
*/

use backend::dialogue::{EvolvingDialogue, UserInput};
use backend::memory::SemanticMemory;
use backend::embeddings::EmbeddingsClient;
use std::sync::Arc;

#[tokio::test]
#[ignore] // Требует работающего embeddings сервиса
async fn test_full_9_step_cycle_with_metacognition() {
    // Инициализация компонентов
    let embeddings_client = EmbeddingsClient::new("http://localhost:8765".to_string());
    let semantic_memory = Arc::new(
        SemanticMemory::new(embeddings_client, None)
            .await
            .expect("Failed to create SemanticMemory")
    );
    
    let dialogue = EvolvingDialogue::new(semantic_memory);
    
    // Тестовый ввод
    let input = UserInput {
        text: "Привет, Нейра! Как ты себя чувствуешь?".to_string(),
        session_id: "test_consciousness_session".to_string(),
        chat_id: "test_chat".to_string(),
        user_id: Some("test_user".to_string()),
        timestamp: chrono::Utc::now(),
    };
    
    // Выполнить полный цикл (включая 9-й шаг META-REFLECT)
    let response = dialogue
        .respond_with_growth(input)
        .await
        .expect("Failed to generate response");
    
    // Проверки
    assert!(!response.text.is_empty(), "Response should not be empty");
    assert!(response.confidence > 0.0, "Confidence should be positive");
    
    // Проверяем, что метакогниция записала мысли
    let metacognition = &dialogue.metacognition;
    let stats = metacognition.get_stats().await;
    
    assert_eq!(stats.total_thoughts, 1, "Should record 1 thought trace");
    
    // Статистика bias может быть 0 (если нет искажений) или больше
    println!("MetaCognition stats: {:?}", stats);
}

#[tokio::test]
async fn test_metacognition_bias_detection_in_dialogue() {
    // Mock embeddings для теста (в реальности нужен сервис)
    let embeddings_client = EmbeddingsClient::new("http://localhost:8765".to_string());
    let semantic_memory = Arc::new(
        SemanticMemory::new(embeddings_client, None)
            .await
            .unwrap_or_else(|_| {
                panic!("Embeddings service not available. Start mock service or skip test.")
            })
    );
    
    let dialogue = EvolvingDialogue::new(semantic_memory);
    
    // Создаём несколько диалогов для накопления данных
    for i in 1..=3 {
        let input = UserInput {
            text: format!("Тестовое сообщение {}", i),
            session_id: format!("session_{}", i),
            chat_id: "test_chat".to_string(),
            user_id: Some("test_user".to_string()),
            timestamp: chrono::Utc::now(),
        };
        
        let _response = dialogue.respond_with_growth(input).await;
    }
    
    // Проверяем накопление thought traces
    let stats = dialogue.metacognition.get_stats().await;
    println!("After 3 conversations - MetaCognition stats: {:?}", stats);
    
    // Должно быть записано минимум 3 thought trace
    // (может быть больше, если были ошибки и retry)
    assert!(
        stats.total_thoughts >= 3,
        "Should have at least 3 thought traces, got {}",
        stats.total_thoughts
    );
}

#[test]
fn test_metacognition_integration_exists() {
    // Простая проверка, что модуль consciousness экспортируется
    use backend::consciousness::MetaCognitionEngine;
    
    let _engine = MetaCognitionEngine::new();
    // Если компилируется — тест пройден
}
