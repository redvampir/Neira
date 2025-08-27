/// Узел для простого чата.
pub trait ChatNode: Send + Sync {
    /// Идентификатор узла.
    fn id(&self) -> &str;
    /// Обрабатывает текстовый запрос и возвращает ответ.
    fn chat(&self, input: &str) -> String;
}

/// Простейшая реализация узла чата, возвращающая входной текст.
pub struct EchoChatNode;

impl ChatNode for EchoChatNode {
    fn id(&self) -> &str {
        "echo.chat"
    }

    fn chat(&self, input: &str) -> String {
        input.to_string()
    }
}

impl Default for EchoChatNode {
    fn default() -> Self {
        Self
    }
}
