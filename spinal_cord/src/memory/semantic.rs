/* neira:meta
id: NEI-20251103-semantic-memory
intent: feature
summary: |
  SemanticMemory — in-memory векторная БД для семантического поиска диалогов.
  Использует cosine similarity для поиска похожих разговоров.
*/

use crate::embeddings::{EmbeddingsClient, EmbeddingPrefix};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

const EMBEDDING_DIM: usize = 1024; // multilingual-e5-large

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("Embeddings error: {0}")]
    Embeddings(#[from] crate::embeddings::EmbeddingsError),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Invalid embedding dimension: expected {expected}, got {actual}")]
    InvalidDimension { expected: usize, actual: usize },
}

pub type Result<T> = std::result::Result<T, MemoryError>;

/// Хранимая запись диалога с эмбеддингом
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationEmbedding {
    pub id: String,
    pub text: String,
    pub embedding: Vec<f32>,
    pub timestamp_ms: i64,
    pub metadata: HashMap<String, String>,
}

impl ConversationEmbedding {
    pub fn new(id: String, text: String, embedding: Vec<f32>) -> Self {
        Self {
            id,
            text,
            embedding,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Результат поиска с метрикой похожести
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub conversation: ConversationEmbedding,
    pub similarity: f32, // cosine similarity [0, 1]
}

/// In-memory векторная БД для семантического поиска
pub struct SemanticMemory {
    embeddings_client: Arc<EmbeddingsClient>,
    // Хранилище: id → (embedding, conversation)
    storage: Arc<RwLock<HashMap<String, ConversationEmbedding>>>,
}

impl SemanticMemory {
    /// Создать новую семантическую память
    pub fn new(embeddings_client: Arc<EmbeddingsClient>) -> Self {
        Self {
            embeddings_client,
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Сохранить диалог в память (автоматически генерирует embedding)
    pub async fn remember(&self, id: String, text: String, metadata: HashMap<String, String>) -> Result<()> {
        // Получить embedding для текста
        let embedding = self.embeddings_client
            .encode(&text, EmbeddingPrefix::Passage)
            .await?;
        
        if embedding.len() != EMBEDDING_DIM {
            return Err(MemoryError::InvalidDimension {
                expected: EMBEDDING_DIM,
                actual: embedding.len(),
            });
        }
        
        // Создать запись
        let mut conv = ConversationEmbedding::new(id.clone(), text, embedding);
        conv.metadata = metadata;
        
        // Сохранить
        let mut storage = self.storage.write().await;
        storage.insert(id.clone(), conv);
        
        // Метрики
        metrics::counter!("semantic_memory_store_total").increment(1);
        metrics::gauge!("semantic_memory_size").set(storage.len() as f64);
        
        Ok(())
    }
    
    /// Вспомнить похожие диалоги (top-k по cosine similarity)
    pub async fn recall_similar(&self, query: &str, top_k: usize) -> Result<Vec<SearchResult>> {
        // Получить embedding для запроса
        let query_embedding = self.embeddings_client
            .encode(query, EmbeddingPrefix::Query)
            .await?;
        
        if query_embedding.len() != EMBEDDING_DIM {
            return Err(MemoryError::InvalidDimension {
                expected: EMBEDDING_DIM,
                actual: query_embedding.len(),
            });
        }
        
        // Поиск похожих
        let storage = self.storage.read().await;
        let mut results: Vec<SearchResult> = storage
            .values()
            .map(|conv| {
                let similarity = cosine_similarity(&query_embedding, &conv.embedding);
                SearchResult {
                    conversation: conv.clone(),
                    similarity,
                }
            })
            .collect();
        
        // Сортировка по убыванию similarity
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        
        // Взять топ-k
        results.truncate(top_k);
        
        // Метрики
        metrics::counter!("semantic_memory_recall_total").increment(1);
        metrics::histogram!("semantic_memory_recall_results").record(results.len() as f64);
        if !results.is_empty() {
            metrics::histogram!("semantic_memory_top_similarity").record(results[0].similarity as f64);
        }
        
        Ok(results)
    }
    
    /// Получить конкретный диалог по ID
    pub async fn get(&self, id: &str) -> Result<ConversationEmbedding> {
        let storage = self.storage.read().await;
        storage
            .get(id)
            .cloned()
            .ok_or_else(|| MemoryError::NotFound(id.to_string()))
    }
    
    /// Удалить диалог из памяти
    pub async fn forget(&self, id: &str) -> Result<()> {
        let mut storage = self.storage.write().await;
        storage.remove(id);
        
        metrics::counter!("semantic_memory_forget_total").increment(1);
        metrics::gauge!("semantic_memory_size").set(storage.len() as f64);
        
        Ok(())
    }
    
    /// Размер памяти (количество сохранённых диалогов)
    pub async fn size(&self) -> usize {
        self.storage.read().await.len()
    }
    
    /// Очистить всю память
    pub async fn clear(&self) {
        let mut storage = self.storage.write().await;
        storage.clear();
        
        metrics::gauge!("semantic_memory_size").set(0.0);
    }
}

/// Cosine similarity между двумя векторами
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Vectors must have same dimension");
    
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    
    dot_product / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);
        
        let c = vec![1.0, 0.0, 0.0];
        let d = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&c, &d) - 0.0).abs() < 0.001);
        
        let e = vec![1.0, 1.0, 0.0];
        let f = vec![1.0, 0.0, 0.0];
        let sim = cosine_similarity(&e, &f);
        assert!(sim > 0.7 && sim < 0.8); // ~0.707
    }
    
    #[tokio::test]
    #[ignore] // требует запущенного embeddings сервиса
    async fn test_remember_and_recall() {
        let client = Arc::new(EmbeddingsClient::new());
        let memory = SemanticMemory::new(client);
        
        // Сохранить несколько диалогов
        memory.remember(
            "1".to_string(),
            "Привет, как дела?".to_string(),
            HashMap::new()
        ).await.unwrap();
        
        memory.remember(
            "2".to_string(),
            "Что такое машинное обучение?".to_string(),
            HashMap::new()
        ).await.unwrap();
        
        memory.remember(
            "3".to_string(),
            "Добрый день, как поживаешь?".to_string(),
            HashMap::new()
        ).await.unwrap();
        
        // Поиск похожих
        let results = memory.recall_similar("Здравствуй", 2).await.unwrap();
        
        assert_eq!(results.len(), 2);
        // Должны быть диалоги 1 и 3 (приветствия)
        assert!(results[0].similarity > 0.5);
    }
}
