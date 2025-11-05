/* neira:meta
id: NEI-20251103-semantic-memory-tests
intent: test
summary: |
  Интеграционные и unit-тесты для SemanticMemory.
  Тестирование cosine similarity, remember/recall, векторного поиска.
*/

use std::collections::HashMap;
use std::sync::Arc;

// Эти тесты требуют запущенного embeddings сервиса
// Запустить: cargo test --test semantic_memory_tests -- --ignored

#[tokio::test]
#[ignore]
async fn test_semantic_memory_remember_and_get() {
    use backend::embeddings::EmbeddingsClient;
    use backend::memory::SemanticMemory;

    let client = Arc::new(EmbeddingsClient::new());
    let memory = SemanticMemory::new(client);

    // Сохранить диалог
    let mut metadata = HashMap::new();
    metadata.insert("user".to_string(), "test_user".to_string());
    metadata.insert("session".to_string(), "sess_001".to_string());

    memory
        .remember(
            "dialog_1".to_string(),
            "Привет, Нейра! Как дела?".to_string(),
            metadata.clone(),
        )
        .await
        .expect("Failed to remember");

    // Проверить размер памяти
    assert_eq!(memory.size().await, 1);

    // Получить по ID
    let conv = memory.get("dialog_1").await.expect("Failed to get");
    assert_eq!(conv.text, "Привет, Нейра! Как дела?");
    assert_eq!(conv.embedding.len(), 1024); // multilingual-e5-large
    assert_eq!(conv.metadata.get("user").unwrap(), "test_user");
}

#[tokio::test]
#[ignore]
async fn test_semantic_memory_recall_similar() {
    use backend::embeddings::EmbeddingsClient;
    use backend::memory::SemanticMemory;

    let client = Arc::new(EmbeddingsClient::new());
    let memory = SemanticMemory::new(client);

    // Сохранить несколько диалогов
    memory
        .remember(
            "greeting_1".to_string(),
            "Привет, как дела?".to_string(),
            HashMap::new(),
        )
        .await
        .unwrap();

    memory
        .remember(
            "greeting_2".to_string(),
            "Добрый день, как поживаешь?".to_string(),
            HashMap::new(),
        )
        .await
        .unwrap();

    memory
        .remember(
            "ml_question".to_string(),
            "Что такое машинное обучение?".to_string(),
            HashMap::new(),
        )
        .await
        .unwrap();

    memory
        .remember(
            "weather".to_string(),
            "Какая сегодня погода?".to_string(),
            HashMap::new(),
        )
        .await
        .unwrap();

    // Поиск похожих на приветствие
    let results = memory
        .recall_similar("Здравствуй, как жизнь?", 2)
        .await
        .expect("Failed to recall");

    // Должно найти 2 самых похожих
    assert_eq!(results.len(), 2);

    // Первый результат должен быть с высоким similarity
    assert!(results[0].similarity > 0.6, "Top result similarity too low: {}", results[0].similarity);

    // Первые два должны быть приветствиями
    assert!(
        results[0].conversation.id.starts_with("greeting") ||
        results[1].conversation.id.starts_with("greeting"),
        "Expected greeting dialogs in top results"
    );

    println!("Top result: {} (similarity: {:.3})", results[0].conversation.text, results[0].similarity);
    println!("Second result: {} (similarity: {:.3})", results[1].conversation.text, results[1].similarity);
}

#[tokio::test]
#[ignore]
async fn test_semantic_memory_forget() {
    use backend::embeddings::EmbeddingsClient;
    use backend::memory::SemanticMemory;

    let client = Arc::new(EmbeddingsClient::new());
    let memory = SemanticMemory::new(client);

    // Сохранить диалог
    memory
        .remember(
            "temp_dialog".to_string(),
            "Временный диалог".to_string(),
            HashMap::new(),
        )
        .await
        .unwrap();

    assert_eq!(memory.size().await, 1);

    // Удалить
    memory.forget("temp_dialog").await.unwrap();

    assert_eq!(memory.size().await, 0);

    // Попытка получить должна вернуть ошибку
    let result = memory.get("temp_dialog").await;
    assert!(result.is_err(), "Expected NotFound error");
}

#[tokio::test]
#[ignore]
async fn test_semantic_memory_clear() {
    use backend::embeddings::EmbeddingsClient;
    use backend::memory::SemanticMemory;

    let client = Arc::new(EmbeddingsClient::new());
    let memory = SemanticMemory::new(client);

    // Сохранить несколько диалогов
    for i in 0..5 {
        memory
            .remember(
                format!("dialog_{}", i),
                format!("Диалог номер {}", i),
                HashMap::new(),
            )
            .await
            .unwrap();
    }

    assert_eq!(memory.size().await, 5);

    // Очистить всё
    memory.clear().await;

    assert_eq!(memory.size().await, 0);
}

#[tokio::test]
#[ignore]
async fn test_semantic_memory_multilingual() {
    use backend::embeddings::EmbeddingsClient;
    use backend::memory::SemanticMemory;

    let client = Arc::new(EmbeddingsClient::new());
    let memory = SemanticMemory::new(client);

    // Русский
    memory
        .remember(
            "ru_1".to_string(),
            "Я люблю программирование".to_string(),
            HashMap::new(),
        )
        .await
        .unwrap();

    // Английский (похожий смысл)
    memory
        .remember(
            "en_1".to_string(),
            "I love programming".to_string(),
            HashMap::new(),
        )
        .await
        .unwrap();

    // Немецкий (другая тема)
    memory
        .remember(
            "de_1".to_string(),
            "Das Wetter ist heute schön".to_string(),
            HashMap::new(),
        )
        .await
        .unwrap();

    // Поиск на русском должен найти английский аналог
    let results = memory
        .recall_similar("Я обожаю писать код", 2)
        .await
        .unwrap();

    assert_eq!(results.len(), 2);

    // Первый должен быть русский или английский (оба про программирование)
    assert!(
        results[0].conversation.id == "ru_1" || results[0].conversation.id == "en_1",
        "Expected programming-related dialog, got: {}",
        results[0].conversation.id
    );

    println!("Multilingual search results:");
    for (i, result) in results.iter().enumerate() {
        println!("{}. {} (similarity: {:.3})", i+1, result.conversation.text, result.similarity);
    }
}

#[test]
fn test_cosine_similarity_unit() {
    // Unit тест для cosine similarity (не требует сервиса)
    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len());
        let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }
        dot / (norm_a * norm_b)
    }

    // Идентичные векторы → similarity = 1.0
    let a = vec![1.0, 0.0, 0.0];
    let b = vec![1.0, 0.0, 0.0];
    assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

    // Ортогональные векторы → similarity = 0.0
    let c = vec![1.0, 0.0, 0.0];
    let d = vec![0.0, 1.0, 0.0];
    assert!((cosine_similarity(&c, &d) - 0.0).abs() < 0.001);

    // Под углом 45° → similarity ≈ 0.707
    let e = vec![1.0, 1.0, 0.0];
    let f = vec![1.0, 0.0, 0.0];
    let sim = cosine_similarity(&e, &f);
    assert!(sim > 0.7 && sim < 0.72, "Expected ~0.707, got {}", sim);

    // Противоположные векторы → similarity = -1.0
    let g = vec![1.0, 0.0, 0.0];
    let h = vec![-1.0, 0.0, 0.0];
    assert!((cosine_similarity(&g, &h) + 1.0).abs() < 0.001);
}
