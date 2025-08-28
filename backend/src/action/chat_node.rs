use async_trait::async_trait;
use tracing::info;
use crate::context::context_storage::{ChatMessage, ContextStorage, Role};
use chrono::Utc;

/// Узел для простого чата.
#[async_trait]
pub trait ChatNode: Send + Sync {
    /// Идентификатор узла.
    fn id(&self) -> &str;
    /// Обрабатывает текстовый запрос и возвращает ответ.
    async fn chat(
        &self,
        chat_id: &str,
        session_id: Option<String>,
        input: &str,
        storage: &dyn ContextStorage,
    ) -> String;
}

/// Простейшая реализация узла чата, возвращающая входной текст.
pub struct EchoChatNode;

#[async_trait]
impl ChatNode for EchoChatNode {
    fn id(&self) -> &str {
        "echo.chat"
    }

    async fn chat(
        &self,
        chat_id: &str,
        session_id: Option<String>,
        input: &str,
        storage: &dyn ContextStorage,
    ) -> String {
        let sid_log = session_id.as_deref().unwrap_or("<none>");
        info!(chat_id=%chat_id, session_id=%sid_log, "chat request: {}", input);

        // Если задан session_id, подгружаем контекст диалога
        if let Some(ref sid) = session_id {
            let _ = storage.load_session(chat_id, sid);
        }

        // Save user message
        if let Some(ref sid) = session_id {
            let _ = storage.save_message(
                chat_id,
                sid,
                &ChatMessage {
                    role: Role::User,
                    content: input.to_string(),
                    timestamp_ms: Utc::now().timestamp_millis(),
                    source: Some("user".into()),
                    message_id: None,
                    thread_id: None,
                    parent_id: None,
                },
            );
        }

        // Echo logic
        let response = input.to_string();

        // Save assistant response
        if let Some(ref sid) = session_id {
            let _ = storage.save_message(
                chat_id,
                sid,
                &ChatMessage {
                    role: Role::Assistant,
                    content: response.clone(),
                    timestamp_ms: Utc::now().timestamp_millis(),
                    source: Some("assistant".into()),
                    message_id: None,
                    thread_id: None,
                    parent_id: None,
                },
            );
        }

        info!(chat_id=%chat_id, session_id=%sid_log, "chat response: {}", response);
        response
    }
}

impl Default for EchoChatNode {
    fn default() -> Self {
        Self
    }
}
