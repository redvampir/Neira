use async_trait::async_trait;
use tracing::info;

/// Узел для простого чата.
#[async_trait]
pub trait ChatNode: Send + Sync {
    /// Идентификатор узла.
    fn id(&self) -> &str;
    /// Обрабатывает текстовый запрос и возвращает ответ.
    async fn chat(&self, input: &str) -> String;
}

/// Простейшая реализация узла чата, возвращающая входной текст.
pub struct EchoChatNode;

#[async_trait]
impl ChatNode for EchoChatNode {
    fn id(&self) -> &str {
        "echo.chat"
    }

    async fn chat(&self, input: &str) -> String {
        info!("chat request: {}", input);
        let response = input.to_string();
        info!("chat response: {}", response);
        response
    }
}

impl Default for EchoChatNode {
    fn default() -> Self {
        Self
    }
}
