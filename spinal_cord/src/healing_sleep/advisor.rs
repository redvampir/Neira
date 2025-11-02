/* neira:meta
id: NEI-20280430-120100-healing-advisor
intent: code
summary: |
  Определяет интерфейс внешнего консультанта и типы для советов, полученных во время «исцеляющего сна».
*/
use std::collections::BTreeMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::incident::IncidentSeverity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsultationPrompt {
    pub refined_question: String,
    pub context: String,
    pub expectations: String,
    pub focus: IncidentSeverity,
    pub labels: BTreeMap<String, String>,
}

impl ConsultationPrompt {
    pub fn new(
        question: String,
        context: String,
        expectations: String,
        focus: IncidentSeverity,
    ) -> Self {
        Self {
            refined_question: question,
            context,
            expectations,
            focus,
            labels: BTreeMap::new(),
        }
    }

    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsultationRecord {
    pub provider: String,
    pub advice: String,
    pub confidence: Option<f32>,
    #[serde(default)]
    pub follow_up: Vec<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

impl ConsultationRecord {
    pub fn new(provider: impl Into<String>, advice: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            advice: advice.into(),
            confidence: None,
            follow_up: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_confidence(mut self, value: f32) -> Self {
        self.confidence = Some(value.clamp(0.0, 1.0));
        self
    }
}

#[derive(Debug, Error)]
pub enum ConsultationError {
    #[error("внешний консультант недоступен: {0}")]
    Unavailable(String),
    #[error("консультант вернул ошибку: {0}")]
    Provider(String),
    #[error("не удалось подготовить запрос: {0}")]
    Prompt(String),
    #[error("транспортная ошибка: {0}")]
    Transport(String),
}

impl ConsultationError {
    pub fn transport(err: impl std::fmt::Display) -> Self {
        ConsultationError::Transport(err.to_string())
    }
}

#[async_trait]
pub trait ExternalConsultant: Send + Sync {
    fn name(&self) -> &'static str;
    async fn consult(
        &self,
        prompt: &ConsultationPrompt,
    ) -> Result<ConsultationRecord, ConsultationError>;
}

#[derive(Clone)]
pub struct NoOpConsultant;

#[async_trait]
impl ExternalConsultant for NoOpConsultant {
    fn name(&self) -> &'static str {
        "disabled"
    }

    async fn consult(
        &self,
        _: &ConsultationPrompt,
    ) -> Result<ConsultationRecord, ConsultationError> {
        Err(ConsultationError::Unavailable(
            "внешние подсказки отключены политикой".into(),
        ))
    }
}

impl From<NoOpConsultant> for Arc<dyn ExternalConsultant> {
    fn from(value: NoOpConsultant) -> Self {
        Arc::new(value)
    }
}
