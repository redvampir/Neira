use super::advisor::{
    ConsultationError, ConsultationPrompt, ConsultationRecord, ExternalConsultant,
};
use super::config::HealingSleepConfig;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct OpenAiConsultant {
    client: Client,
    endpoint: String,
    model: String,
    api_key: String,
    temperature: f32,
    system_prompt: String,
}

impl OpenAiConsultant {
    pub fn from_config(config: &HealingSleepConfig) -> Result<Self, ConsultationError> {
        let settings = &config.consultant;
        let api_key = settings
            .api_key
            .clone()
            .or_else(|| std::env::var("HEALING_SLEEP_OPENAI_KEY").ok())
            .or_else(|| std::env::var("OPENAI_API_KEY").ok())
            .ok_or_else(|| {
                ConsultationError::Unavailable(
                    "не найден API-ключ (HEALING_SLEEP_OPENAI_KEY или OPENAI_API_KEY)".into(),
                )
            })?;

        let endpoint = settings
            .endpoint
            .clone()
            .or_else(|| std::env::var("HEALING_SLEEP_OPENAI_ENDPOINT").ok())
            .or_else(|| std::env::var("OPENAI_BASE_URL").ok())
            .unwrap_or_else(|| "https://api.openai.com/v1/chat/completions".into());

        let model = settings
            .model
            .clone()
            .or_else(|| std::env::var("HEALING_SLEEP_OPENAI_MODEL").ok())
            .or_else(|| std::env::var("OPENAI_MODEL").ok())
            .unwrap_or_else(|| "gpt-4o-mini".into());

        let timeout = settings.timeout_seconds.unwrap_or(30).max(5);
        let temperature = settings.temperature.unwrap_or(0.2).clamp(0.0, 2.0);
        let system_prompt = settings
            .system_prompt
            .clone()
            .or_else(|| std::env::var("HEALING_SLEEP_OPENAI_SYSTEM_PROMPT").ok())
            .unwrap_or_else(|| {
                "Ты опытная инженерка сопровождения. Получишь описание инцидента, проанализируй и предложи конкретные шаги по исправлению.".into()
            });

        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .map_err(|err| ConsultationError::Provider(format!("client build failed: {err}")))?;

        Ok(Self {
            client,
            endpoint,
            model,
            api_key,
            temperature,
            system_prompt,
        })
    }
}

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Serialize)]
struct ChatMessage {
    role: &'static str,
    content: String,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatChoiceMessage,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct ChatChoiceMessage {
    #[serde(default)]
    content: Option<String>,
}

#[async_trait]
impl ExternalConsultant for OpenAiConsultant {
    fn name(&self) -> &'static str {
        "openai"
    }

    async fn consult(
        &self,
        prompt: &ConsultationPrompt,
    ) -> Result<ConsultationRecord, ConsultationError> {
        let user_message = build_user_message(prompt);
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            temperature: self.temperature,
            messages: vec![
                ChatMessage {
                    role: "system",
                    content: self.system_prompt.clone(),
                },
                ChatMessage {
                    role: "user",
                    content: user_message,
                },
            ],
        };

        let response = self
            .client
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(ConsultationError::transport)?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| "<empty>".into());
            return Err(ConsultationError::Provider(format!(
                "HTTP {status}: {body}"
            )));
        }

        let payload: ChatCompletionResponse = response
            .json()
            .await
            .map_err(ConsultationError::transport)?;
        let choice = payload
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| ConsultationError::Provider("пустой ответ от модели".into()))?;
        let advice = choice
            .message
            .content
            .unwrap_or_else(|| "Модель не вернула ответа".into());

        let mut record = ConsultationRecord::new(self.name(), advice);
        record.metadata.insert("model".into(), self.model.clone());
        if let Some(reason) = choice.finish_reason {
            record.metadata.insert("finish_reason".into(), reason);
        }
        if !prompt.labels.is_empty() {
            record.metadata.insert(
                "labels".into(),
                prompt
                    .labels
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
        Ok(record)
    }
}

fn build_user_message(prompt: &ConsultationPrompt) -> String {
    let mut builder = String::new();
    builder.push_str(&prompt.refined_question);
    builder.push_str("\n\nКонтекст:\n");
    builder.push_str(&prompt.context);
    builder.push_str("\n\nОжидания:\n");
    builder.push_str(&prompt.expectations);
    if !prompt.labels.is_empty() {
        builder.push_str("\n\nМетки:\n");
        for (key, value) in &prompt.labels {
            builder.push_str("- ");
            builder.push_str(key);
            builder.push_str(": ");
            builder.push_str(value);
            builder.push('\n');
        }
    }
    builder
}
