/* neira:meta
id: NEI-20251103-embeddings-client
intent: feature
summary: |
  Rust клиент для Python embeddings микросервиса (multilingual-e5-large).
  HTTP запросы через reqwest к localhost:8765.
*/

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "http://127.0.0.1:8765";
const DEFAULT_TIMEOUT_SECS: u64 = 10;

#[derive(Debug, thiserror::Error)]
pub enum EmbeddingsError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    
    #[error("Service unavailable: {0}")]
    Unavailable(String),
    
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

pub type Result<T> = std::result::Result<T, EmbeddingsError>;

/// Префикс для эмбеддингов (важно для качества multilingual-e5)
#[derive(Debug, Clone, Copy)]
pub enum EmbeddingPrefix {
    /// Для поисковых запросов (текущий диалог)
    Query,
    /// Для сохраняемых текстов (память)
    Passage,
}

impl EmbeddingPrefix {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Query => "query",
            Self::Passage => "passage",
        }
    }
}

#[derive(Serialize)]
struct EncodeRequest {
    text: String,
    prefix: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct EncodeResponse {
    embedding: Vec<f32>,
    dimension: usize,
    inference_time_ms: f64,
}

#[derive(Serialize)]
struct BatchEncodeRequest {
    texts: Vec<String>,
    prefix: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct BatchEncodeResponse {
    embeddings: Vec<Vec<f32>>,
    dimension: usize,
    count: usize,
    total_inference_time_ms: f64,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub(crate) struct HealthResponse {
    status: String,
    model: String,
    device: String,
    cuda_available: bool,
    gpu_name: Option<String>,
    embedding_dim: usize,
}

/// Клиент для Python embeddings сервиса
pub struct EmbeddingsClient {
    client: Client,
    base_url: String,
}

impl EmbeddingsClient {
    /// Создать клиент с дефолтными настройками
    pub fn new() -> Self {
        Self::with_base_url(DEFAULT_BASE_URL.to_string())
    }
    
    /// Создать клиент с кастомным URL
    pub fn with_base_url(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()
            .expect("failed to build HTTP client");
        
        Self { client, base_url }
    }
    
    /// Создать из переменных окружения
    pub fn from_env() -> Self {
        let base_url = std::env::var("EMBEDDINGS_SERVICE_URL")
            .unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
        
        Self::with_base_url(base_url)
    }
    
    /// Проверка здоровья сервиса
    #[allow(dead_code)]
    pub(crate) async fn health_check(&self) -> Result<HealthResponse> {
        let url = format!("{}/health", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| EmbeddingsError::Unavailable(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(EmbeddingsError::Unavailable(
                format!("health check failed: {}", response.status())
            ));
        }
        
        response.json().await.map_err(Into::into)
    }
    
    /// Закодировать один текст в вектор
    pub async fn encode(&self, text: &str, prefix: EmbeddingPrefix) -> Result<Vec<f32>> {
        let url = format!("{}/encode", self.base_url);
        
        let request = EncodeRequest {
            text: text.to_string(),
            prefix: prefix.as_str().to_string(),
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(EmbeddingsError::InvalidResponse(
                format!("encode failed: {}", response.status())
            ));
        }
        
        let result: EncodeResponse = response.json().await?;
        
        // Логирование метрик
        metrics::histogram!("embeddings_inference_time_ms")
            .record(result.inference_time_ms);
        metrics::counter!("embeddings_requests_total").increment(1);
        
        Ok(result.embedding)
    }
    
    /// Закодировать несколько текстов (batch - эффективнее)
    pub async fn encode_batch(&self, texts: &[String], prefix: EmbeddingPrefix) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        
        let url = format!("{}/encode/batch", self.base_url);
        
        let request = BatchEncodeRequest {
            texts: texts.to_vec(),
            prefix: prefix.as_str().to_string(),
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(EmbeddingsError::InvalidResponse(
                format!("batch encode failed: {}", response.status())
            ));
        }
        
        let result: BatchEncodeResponse = response.json().await?;
        
        // Метрики
        metrics::histogram!("embeddings_batch_inference_time_ms")
            .record(result.total_inference_time_ms);
        metrics::counter!("embeddings_batch_requests_total").increment(1);
        metrics::histogram!("embeddings_batch_size").record(result.count as f64);
        
        Ok(result.embeddings)
    }
}

impl Default for EmbeddingsClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    #[ignore] // требует запущенного сервиса
    async fn test_health_check() {
        let client = EmbeddingsClient::new();
        let health = client.health_check().await.unwrap();
        
        assert_eq!(health.status, "healthy");
        assert_eq!(health.model, "intfloat/multilingual-e5-large");
        assert_eq!(health.embedding_dim, 1024);
    }
    
    #[tokio::test]
    #[ignore]
    async fn test_encode() {
        let client = EmbeddingsClient::new();
        let embedding = client.encode("Привет, Нейра!", EmbeddingPrefix::Query).await.unwrap();
        
        assert_eq!(embedding.len(), 1024);
    }
    
    #[tokio::test]
    #[ignore]
    async fn test_encode_batch() {
        let client = EmbeddingsClient::new();
        let texts = vec![
            "Первый текст".to_string(),
            "Второй текст".to_string(),
            "Третий текст".to_string(),
        ];
        
        let embeddings = client.encode_batch(&texts, EmbeddingPrefix::Passage).await.unwrap();
        
        assert_eq!(embeddings.len(), 3);
        assert_eq!(embeddings[0].len(), 1024);
    }
}
