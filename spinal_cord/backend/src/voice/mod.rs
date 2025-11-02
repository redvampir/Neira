/* neira:meta
id: NEI-20280105-voice-module
intent: code
summary: |
  Голосовой контур: конфигурация, бэкенды TTS/STT и клетки органа речи
  с интеграцией фабрики и HTTP-обработчиками.
*/

pub mod backend;
pub mod cells;
pub mod error;
pub mod organ;
pub mod phonemes;

pub use backend::{CodecVoiceBackend, CommandVoiceBackend, VoiceBackend, VoiceBackendMode};
pub use error::VoiceError;
pub use organ::{VoiceConfig, VoiceOrgan, VoiceSynthesis, VoiceTranscription};
