/* neira:meta
id: NEI-20280105-voice-organ
intent: code
summary: |
  Орган речи: конфигурация из окружения, регистрация клеток в фабрике
  и синхронный API TTS/STT с хранением последнего результата.
*/

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock, Weak};

use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use serde::Serialize;

use crate::cell_registry::CellRegistry;
use crate::cell_template::{ActionCellTemplate, CellTemplate, Metadata};
use crate::hearing;
use crate::synapse_hub::SynapseHub;

use super::backend::{CodecVoiceBackend, CommandVoiceBackend, VoiceBackend, VoiceBackendMode};
use super::cells::{
    VoiceSpeakActionCell, VoiceSpeakAdapterCell, VoiceTextNormalizeCell, VoiceTextToPhonemesCell,
    SPEAK_ACTION_ID, SPEAK_ADAPTER_ID, TEXT_NORMALIZE_ID, TEXT_TO_PHONEMES_ID,
};
use super::phonemes::phonemize;
use super::VoiceError;

#[derive(Clone, Debug, Serialize)]
pub struct VoiceSynthesis {
    pub request_id: String,
    pub file_path: PathBuf,
    pub audio_base64: String,
    pub text: String,
    pub phonemes: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct VoiceTranscription {
    pub request_id: String,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct CommandConfig {
    pub program: String,
    pub args: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct VoiceConfig {
    pub backend_mode: VoiceBackendMode,
    pub output_dir: PathBuf,
    pub sample_rate: u32,
    pub play_command: Option<CommandConfig>,
    pub tts_command: Option<CommandConfig>,
    pub stt_command: Option<CommandConfig>,
}

impl VoiceConfig {
    pub fn from_env() -> Self {
        let backend_mode = std::env::var("VOICE_BACKEND")
            .map(|v| v.to_lowercase())
            .map(|v| {
                if v == "command" {
                    VoiceBackendMode::Command
                } else {
                    VoiceBackendMode::Codec
                }
            })
            .unwrap_or(VoiceBackendMode::Codec);
        let output_dir = std::env::var("VOICE_OUTPUT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("voice_output"));
        let sample_rate = std::env::var("VOICE_SAMPLE_RATE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(16_000);
        let play_command = parse_command("VOICE_PLAY_CMD", "VOICE_PLAY_ARGS");
        let tts_command = parse_command("VOICE_TTS_CMD", "VOICE_TTS_ARGS");
        let stt_command = parse_command("VOICE_STT_CMD", "VOICE_STT_ARGS");
        Self {
            backend_mode,
            output_dir,
            sample_rate,
            play_command,
            tts_command,
            stt_command,
        }
    }

    pub fn resolve_backend(&self) -> (Arc<dyn VoiceBackend>, VoiceBackendMode) {
        match self.backend_mode {
            VoiceBackendMode::Command => {
                if let (Some(tts), Some(stt)) = (&self.tts_command, &self.stt_command) {
                    let backend: Arc<dyn VoiceBackend> = Arc::new(CommandVoiceBackend::new(
                        tts.program.clone(),
                        tts.args.clone(),
                        stt.program.clone(),
                        stt.args.clone(),
                    ));
                    (backend, VoiceBackendMode::Command)
                } else {
                    hearing::warn(
                        "VOICE_BACKEND=command, но команды TTS/STT не заданы; используется кодек",
                    );
                    let backend: Arc<dyn VoiceBackend> =
                        Arc::new(CodecVoiceBackend::new(self.sample_rate));
                    (backend, VoiceBackendMode::Codec)
                }
            }
            VoiceBackendMode::Codec => {
                let backend: Arc<dyn VoiceBackend> =
                    Arc::new(CodecVoiceBackend::new(self.sample_rate));
                (backend, VoiceBackendMode::Codec)
            }
        }
    }
}

fn parse_command(cmd_key: &str, args_key: &str) -> Option<CommandConfig> {
    let program = std::env::var(cmd_key).ok()?;
    let args = std::env::var(args_key)
        .ok()
        .map(parse_list)
        .unwrap_or_default();
    Some(CommandConfig { program, args })
}

fn parse_list(raw: String) -> Vec<String> {
    raw.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub struct VoiceOrgan {
    hub: Weak<SynapseHub>,
    registry: Arc<CellRegistry>,
    backend: Arc<dyn VoiceBackend>,
    backend_mode: VoiceBackendMode,
    output_dir: PathBuf,
    play_command: Option<CommandConfig>,
    latest: RwLock<Option<VoiceSynthesis>>,
    counter: AtomicU64,
}

impl VoiceOrgan {
    pub fn new(
        hub: Option<Arc<SynapseHub>>,
        registry: Arc<CellRegistry>,
        config: VoiceConfig,
    ) -> Result<Arc<Self>, VoiceError> {
        let (backend, mode) = config.resolve_backend();
        if !config.output_dir.exists() {
            fs::create_dir_all(&config.output_dir)?;
        }
        let organ = Arc::new(Self {
            hub: hub.as_ref().map(Arc::downgrade).unwrap_or_else(Weak::new),
            registry,
            backend,
            backend_mode: mode,
            output_dir: config.output_dir.clone(),
            play_command: config.play_command.clone(),
            latest: RwLock::new(None),
            counter: AtomicU64::new(1),
        });
        hearing::info(&format!(
            "voice organ инициализирован; режим={:?} каталог={}",
            organ.backend_mode,
            organ.output_dir.display()
        ));
        Ok(organ)
    }

    pub fn bootstrap(self: &Arc<Self>) -> Result<(), VoiceError> {
        self.ensure_templates()?;
        self.register_cells();
        Ok(())
    }

    fn ensure_templates(&self) -> Result<(), VoiceError> {
        let hub = self.hub.upgrade();
        let factory_enabled = hub
            .as_ref()
            .map(|h| h.factory_is_adapter_enabled())
            .unwrap_or(false);
        for tpl in analysis_templates() {
            if self.registry.get(&tpl.id).is_none() {
                self.registry
                    .register_template(tpl.clone())
                    .map_err(VoiceError::Validation)?;
                metrics::counter!("voice_templates_registered_total", "kind" => "analysis")
                    .increment(1);
                if let Some(h) = hub.as_ref() {
                    if factory_enabled {
                        if let Err(err) = h.factory_create("adapter", &tpl) {
                            hearing::warn(&format!("factory_create отказал: {err}"));
                        }
                    }
                }
            }
        }
        for tpl in action_templates() {
            if self.registry.get_action_template(&tpl.id).is_none() {
                self.registry
                    .register_action_template(tpl.clone())
                    .map_err(VoiceError::Validation)?;
                metrics::counter!("voice_templates_registered_total", "kind" => "action")
                    .increment(1);
            }
        }
        Ok(())
    }

    fn register_cells(self: &Arc<Self>) {
        if self.registry.get_analysis_cell(TEXT_NORMALIZE_ID).is_none() {
            self.registry
                .register_analysis_cell(Arc::new(VoiceTextNormalizeCell::new()));
        }
        if self
            .registry
            .get_analysis_cell(TEXT_TO_PHONEMES_ID)
            .is_none()
        {
            self.registry
                .register_analysis_cell(Arc::new(VoiceTextToPhonemesCell::new()));
        }
        if self.registry.get_analysis_cell(SPEAK_ADAPTER_ID).is_none() {
            self.registry
                .register_analysis_cell(Arc::new(VoiceSpeakAdapterCell::new(Arc::clone(self))));
        }
        let already_registered = self
            .registry
            .action_cells()
            .iter()
            .any(|cell| cell.id() == SPEAK_ACTION_ID);
        if !already_registered {
            self.registry
                .register_action_cell(Arc::new(VoiceSpeakActionCell::new(Arc::clone(self))));
        }
    }

    pub fn synthesize(
        &self,
        request_id: Option<&str>,
        text: &str,
        phonemes: Option<&str>,
    ) -> Result<VoiceSynthesis, VoiceError> {
        if text.trim().is_empty() {
            return Err(VoiceError::with_context("пустой текст для синтеза"));
        }
        let id = request_id
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("voice-{}", self.counter.fetch_add(1, Ordering::Relaxed)));
        metrics::counter!("voice_tts_requests_total").increment(1);
        let audio = match self.backend.synthesize(text) {
            Ok(data) => data,
            Err(err) => {
                metrics::counter!("voice_tts_errors_total").increment(1);
                return Err(err);
            }
        };
        let path = self.output_dir.join(format!("{id}.wav"));
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, &audio)?;
        let audio_base64 = BASE64_STANDARD.encode(&audio);
        let phoneme_str = phonemes
            .map(|s| s.to_string())
            .unwrap_or_else(|| phonemize(text));
        let synthesis = VoiceSynthesis {
            request_id: id.clone(),
            file_path: path.clone(),
            audio_base64,
            text: text.to_string(),
            phonemes: phoneme_str,
        };
        *self.latest.write().unwrap() = Some(synthesis.clone());
        hearing::info(&format!(
            "voice синтез завершён; request_id={} файл={}",
            id,
            path.display()
        ));
        Ok(synthesis)
    }

    pub fn transcribe_bytes(&self, audio: &[u8]) -> Result<VoiceTranscription, VoiceError> {
        metrics::counter!("voice_stt_requests_total").increment(1);
        let text = match self.backend.transcribe(audio) {
            Ok(txt) => txt,
            Err(err) => {
                metrics::counter!("voice_stt_errors_total").increment(1);
                return Err(err);
            }
        };
        let id = format!("voice-stt-{}", self.counter.fetch_add(1, Ordering::Relaxed));
        Ok(VoiceTranscription {
            request_id: id,
            text,
        })
    }

    pub fn transcribe_file(&self, path: &Path) -> Result<VoiceTranscription, VoiceError> {
        let data = fs::read(path)?;
        self.transcribe_bytes(&data)
    }

    pub fn transcribe_base64(&self, audio_base64: &str) -> Result<VoiceTranscription, VoiceError> {
        let data = BASE64_STANDARD.decode(audio_base64)?;
        self.transcribe_bytes(&data)
    }

    pub fn latest_output(&self) -> Option<VoiceSynthesis> {
        self.latest.read().unwrap().clone()
    }

    pub fn play_command_config(&self) -> Option<CommandConfig> {
        self.play_command.clone()
    }

    pub fn backend_mode(&self) -> VoiceBackendMode {
        self.backend_mode
    }
}

fn metadata() -> Metadata {
    Metadata {
        schema: "v1".into(),
        extra: HashMap::new(),
    }
}

fn analysis_templates() -> Vec<CellTemplate> {
    vec![
        CellTemplate {
            id: TEXT_NORMALIZE_ID.into(),
            version: "0.1.0".into(),
            analysis_type: "text_normalize".into(),
            links: vec![],
            confidence_threshold: None,
            draft_content: None,
            metadata: metadata(),
        },
        CellTemplate {
            id: TEXT_TO_PHONEMES_ID.into(),
            version: "0.1.0".into(),
            analysis_type: "text_to_phonemes".into(),
            links: vec![TEXT_NORMALIZE_ID.into()],
            confidence_threshold: None,
            draft_content: None,
            metadata: metadata(),
        },
        CellTemplate {
            id: SPEAK_ADAPTER_ID.into(),
            version: "0.1.0".into(),
            analysis_type: "speak_adapter".into(),
            links: vec![TEXT_TO_PHONEMES_ID.into()],
            confidence_threshold: None,
            draft_content: None,
            metadata: metadata(),
        },
    ]
}

fn action_templates() -> Vec<ActionCellTemplate> {
    vec![ActionCellTemplate {
        id: SPEAK_ACTION_ID.into(),
        version: "0.1.0".into(),
        action_type: "speak_adapter".into(),
        links: vec![SPEAK_ADAPTER_ID.into()],
        confidence_threshold: None,
        draft_content: None,
        metadata: metadata(),
    }]
}
