use crate::context::context_storage::{ChatMessage, ContextStorage, Role};
use async_trait::async_trait;
use chrono::Utc;
use std::time::Instant;
use tracing::info;

/// Узел для простого чата.
#[async_trait]
pub trait ChatCell: Send + Sync {
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
pub struct EchoChatCell;

#[async_trait]
impl ChatCell for EchoChatCell {
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
        metrics::counter!("chat_cell_requests_total").increment(1);
        let start = Instant::now();
        let sid_log = session_id.as_deref().unwrap_or("<none>");
        info!(chat_id=%chat_id, session_id=%sid_log, "chat request: {}", input);

        // Если задан session_id, подгружаем контекст диалога
        if let Some(ref sid) = session_id {
            if storage.load_session(chat_id, sid).is_err() {
                metrics::counter!("chat_cell_errors_total").increment(1);
            }
        }

        // User message is saved upstream in SynapseHub to avoid duplicates

        // Echo logic
        let response = input.to_string();

        // Save assistant response
        if let Some(ref sid) = session_id {
            if storage
                .save_message(
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
                )
                .is_err()
            {
                metrics::counter!("chat_cell_errors_total").increment(1);
            }
        }

        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        metrics::histogram!("chat_cell_request_duration_ms").record(elapsed_ms);
        metrics::histogram!("chat_cell_request_duration_ms_p95").record(elapsed_ms);
        metrics::histogram!("chat_cell_request_duration_ms_p99").record(elapsed_ms);
        info!(
            chat_id=%chat_id,
            session_id=%sid_log,
            duration_ms=elapsed_ms,
            "chat response: {}",
            response
        );
        response
    }
}

impl Default for EchoChatCell {
    fn default() -> Self {
        Self
    }
}
/* neira:meta
id: NEI-20250829-setup-meta-chatcell
intent: docs
scope: backend/chat-cell
summary: |
  EchoChatCell: простая отражающая клетка. Входящее user‑сообщение сохраняется в SynapseHub,
  чтобы избежать дублей; здесь сохраняется ответ ассистента.
links:
  - docs/api/spinal_cord.md
metrics:
  - chat_cell_requests_total
  - chat_cell_errors_total
  - chat_cell_request_duration_ms
risks: low
safe_mode:
  affects_write: true
  requires_admin: false
i18n:
  reviewer_note: |
    Держи этот узел минимальным; он хорош как дефолт и для проверок SSE/поиска.
*/
