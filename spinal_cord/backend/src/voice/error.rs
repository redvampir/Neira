/* neira:meta
id: NEI-20280105-voice-error
intent: code
summary: |-
  Унифицированная ошибка голосового контура: покрывает I/O, команды,
  декодирование и интеграцию с фабрикой.
*/

use thiserror::Error;

#[derive(Error, Debug)]
pub enum VoiceError {
    #[error("ошибка ввода-вывода: {0}")]
    Io(#[from] std::io::Error),
    #[error("неверные данные: {0}")]
    InvalidInput(String),
    #[error("ошибка внешней команды: {0}")]
    Command(String),
    #[error("ошибка преобразования UTF-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("ошибка base64: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("ошибка JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("ошибка схемы фабрики: {0}")]
    Validation(String),
    #[error("недоступен SynapseHub для голосового органа")]
    HubUnavailable,
}

impl VoiceError {
    pub fn with_context(msg: impl Into<String>) -> Self {
        VoiceError::InvalidInput(msg.into())
    }
}
