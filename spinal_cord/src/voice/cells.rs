/* neira:meta
id: NEI-20280105-voice-cells
intent: code
summary: |
  Клетки голосового контура: нормализация текста, фонемизация и адаптер
  речи с действием проигрывания.
*/

use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use tokio_util::sync::CancellationToken;

use crate::action_cell::ActionCell;
use crate::action_engine::{ActionCommand, ActionEngine, ActionError};
use crate::analysis_cell::{AnalysisCell, AnalysisResult, CellStatus};
use crate::digestive_pipeline::ParsedInput;
use crate::hearing;
use crate::memory_cell::MemoryCell;

use super::organ::{VoiceOrgan, VoiceSynthesis};
use super::phonemes::phonemize;
use super::VoiceError;

pub const TEXT_NORMALIZE_ID: &str = "analysis.text_normalize.v1";
pub const TEXT_TO_PHONEMES_ID: &str = "analysis.text_to_phonemes.v1";
pub const SPEAK_ADAPTER_ID: &str = "analysis.speak_adapter.v1";
pub const SPEAK_ACTION_ID: &str = "action.speak_adapter.v1";

fn extract_text(input: &ParsedInput) -> Option<String> {
    match input {
        ParsedInput::Json(v) => v
            .get("text")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
            .or_else(|| v.as_str().map(|s| s.to_string())),
        ParsedInput::Text(t) => Some(t.clone()),
    }
}

fn analysis_error(id: &str, message: &str) -> AnalysisResult {
    let mut res = AnalysisResult::new(id, message, vec![message.to_string()]);
    res.status = CellStatus::Error;
    res.explanation = Some(message.to_string());
    res
}

pub struct VoiceTextNormalizeCell;

impl VoiceTextNormalizeCell {
    pub fn new() -> Self {
        Self
    }
}

impl Default for VoiceTextNormalizeCell {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisCell for VoiceTextNormalizeCell {
    fn id(&self) -> &str {
        TEXT_NORMALIZE_ID
    }

    fn analysis_type(&self) -> &str {
        "text_normalize"
    }

    fn status(&self) -> CellStatus {
        CellStatus::Active
    }

    fn links(&self) -> &[String] {
        &[]
    }

    fn confidence_threshold(&self) -> f32 {
        0.0
    }

    fn analyze_parsed(&self, input: &ParsedInput, _: &CancellationToken) -> AnalysisResult {
        let Some(text) = extract_text(input) else {
            return analysis_error(TEXT_NORMALIZE_ID, "не удалось извлечь текст");
        };
        let trimmed = text.trim();
        let normalized = trimmed
            .split_whitespace()
            .filter(|chunk| !chunk.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        let lower = normalized.to_lowercase();
        let output = json!({
            "text": text,
            "normalized": lower,
        });
        let mut result = AnalysisResult::new(
            TEXT_NORMALIZE_ID,
            output.to_string(),
            vec!["удалены лишние пробелы".to_string()],
        );
        result.metadata.schema = "voice.text_normalize/1.0".into();
        result
    }

    fn explain(&self) -> String {
        "Приводит фразы к единому виду перед синтезом".into()
    }
}

pub struct VoiceTextToPhonemesCell;

impl VoiceTextToPhonemesCell {
    pub fn new() -> Self {
        Self
    }
}

impl Default for VoiceTextToPhonemesCell {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisCell for VoiceTextToPhonemesCell {
    fn id(&self) -> &str {
        TEXT_TO_PHONEMES_ID
    }

    fn analysis_type(&self) -> &str {
        "text_to_phonemes"
    }

    fn status(&self) -> CellStatus {
        CellStatus::Active
    }

    fn links(&self) -> &[String] {
        &[]
    }

    fn confidence_threshold(&self) -> f32 {
        0.0
    }

    fn analyze_parsed(&self, input: &ParsedInput, _: &CancellationToken) -> AnalysisResult {
        let Some(base_text) = extract_text(input) else {
            return analysis_error(TEXT_TO_PHONEMES_ID, "не удалось извлечь текст");
        };
        let normalized = match input {
            ParsedInput::Json(v) => v
                .get("normalized")
                .and_then(|t| t.as_str())
                .unwrap_or(base_text.as_str())
                .to_string(),
            ParsedInput::Text(_) => base_text.clone(),
        };
        let phonemes = phonemize(&normalized);
        let output = json!({
            "text": base_text,
            "normalized": normalized,
            "phonemes": phonemes,
        });
        let mut result = AnalysisResult::new(
            TEXT_TO_PHONEMES_ID,
            output.to_string(),
            vec!["подготовлены фонемы".to_string()],
        );
        result.metadata.schema = "voice.text_to_phonemes/1.0".into();
        result
    }

    fn explain(&self) -> String {
        "Готовит фонемный ряд для синтеза речи".into()
    }
}

#[derive(Deserialize)]
struct SpeakPayload {
    #[serde(default)]
    request_id: Option<String>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    normalized: Option<String>,
    #[serde(default)]
    phonemes: Option<String>,
}

pub struct VoiceSpeakAdapterCell {
    organ: Arc<VoiceOrgan>,
}

impl VoiceSpeakAdapterCell {
    pub fn new(organ: Arc<VoiceOrgan>) -> Self {
        Self { organ }
    }

    fn synthesize(&self, payload: SpeakPayload) -> Result<VoiceSynthesis, VoiceError> {
        let text = payload
            .normalized
            .or(payload.text)
            .ok_or_else(|| VoiceError::with_context("отсутствует текст для синтеза"))?;
        let phonemes = payload.phonemes.unwrap_or_else(|| phonemize(&text));
        self.organ
            .synthesize(payload.request_id.as_deref(), &text, Some(&phonemes))
    }
}

impl AnalysisCell for VoiceSpeakAdapterCell {
    fn id(&self) -> &str {
        SPEAK_ADAPTER_ID
    }

    fn analysis_type(&self) -> &str {
        "speak_adapter"
    }

    fn status(&self) -> CellStatus {
        CellStatus::Active
    }

    fn links(&self) -> &[String] {
        &[]
    }

    fn confidence_threshold(&self) -> f32 {
        0.0
    }

    fn analyze_parsed(
        &self,
        input: &ParsedInput,
        cancel_token: &CancellationToken,
    ) -> AnalysisResult {
        if cancel_token.is_cancelled() {
            return analysis_error(SPEAK_ADAPTER_ID, "запрос синтеза отменён");
        }
        let payload: SpeakPayload = match input {
            ParsedInput::Json(v) => {
                serde_json::from_value(v.clone()).unwrap_or_else(|_| SpeakPayload {
                    request_id: None,
                    text: v.as_str().map(|s| s.to_string()),
                    normalized: None,
                    phonemes: None,
                })
            }
            ParsedInput::Text(t) => SpeakPayload {
                request_id: None,
                text: Some(t.clone()),
                normalized: None,
                phonemes: None,
            },
        };
        match self.synthesize(payload) {
            Ok(synth) => {
                hearing::info(&format!("voice synthesis готов; id={}", synth.request_id));
                let output = json!({
                    "request_id": synth.request_id,
                    "file": synth.file_path,
                    "audio_base64": synth.audio_base64,
                    "text": synth.text,
                    "phonemes": synth.phonemes,
                });
                let mut result = AnalysisResult::new(
                    SPEAK_ADAPTER_ID,
                    output.to_string(),
                    vec!["аудиофайл сохранён".to_string()],
                );
                result.metadata.schema = "voice.speak_adapter/1.0".into();
                result
            }
            Err(err) => analysis_error(SPEAK_ADAPTER_ID, &format!("ошибка синтеза: {err}")),
        }
    }

    fn explain(&self) -> String {
        "Запускает TTS и сохраняет аудио".into()
    }
}

pub struct VoiceSpeakActionCell {
    organ: Arc<VoiceOrgan>,
}

impl VoiceSpeakActionCell {
    pub fn new(organ: Arc<VoiceOrgan>) -> Self {
        Self { organ }
    }
}

#[async_trait]
impl ActionCell for VoiceSpeakActionCell {
    fn id(&self) -> &str {
        SPEAK_ACTION_ID
    }

    fn preload(&self, _triggers: &[String], _memory: &Arc<MemoryCell>) {}

    fn command(&self) -> Option<ActionCommand> {
        None
    }

    async fn execute(&self, engine: &ActionEngine) -> Result<Option<String>, ActionError> {
        let Some(latest) = self.organ.latest_output() else {
            return Ok(Some("нет готового аудио для воспроизведения".to_string()));
        };
        if let Some(cfg) = self.organ.play_command_config() {
            let path = latest.file_path.to_string_lossy().to_string();
            let args = cfg
                .args
                .iter()
                .map(|arg| arg.replace("{file}", &path))
                .collect::<Vec<_>>();
            let response = engine
                .execute(ActionCommand::RunCommand {
                    program: cfg.program.clone(),
                    args,
                })
                .await?;
            Ok(Some(response))
        } else {
            Ok(Some(latest.file_path.to_string_lossy().to_string()))
        }
    }
}
