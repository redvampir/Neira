/* neira:meta
id: NEI-20280105-voice-backend
intent: code
summary: |
  Бэкенды голосового контура: командный исполнитель и офлайн кодек,
  обеспечивающие двустороннее преобразование текста и аудио.
*/

use std::io::Write;
use std::process::{Command, Stdio};

use crate::hearing;

use super::error::VoiceError;

pub trait VoiceBackend: Send + Sync {
    fn synthesize(&self, text: &str) -> Result<Vec<u8>, VoiceError>;
    fn transcribe(&self, audio: &[u8]) -> Result<String, VoiceError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoiceBackendMode {
    Codec,
    Command,
}

#[derive(Clone, Debug)]
pub struct CommandVoiceBackend {
    tts_program: String,
    tts_args: Vec<String>,
    stt_program: String,
    stt_args: Vec<String>,
}

impl CommandVoiceBackend {
    pub fn new(
        tts_program: String,
        tts_args: Vec<String>,
        stt_program: String,
        stt_args: Vec<String>,
    ) -> Self {
        Self {
            tts_program,
            tts_args,
            stt_program,
            stt_args,
        }
    }

    fn run_command(
        program: &str,
        args: &[String],
        input: Option<&[u8]>,
    ) -> Result<Vec<u8>, VoiceError> {
        let mut cmd = Command::new(program);
        cmd.args(args);
        if input.is_some() {
            cmd.stdin(Stdio::piped());
        }
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        let mut child = cmd.spawn()?;
        if let Some(data) = input {
            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(data)?;
            }
        }
        let output = child.wait_with_output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            hearing::warn(&format!(
                "voice command failed; program={} code={:?} stderr={}",
                program,
                output.status.code(),
                stderr
            ));
            return Err(VoiceError::Command(format!(
                "команда {program} завершилась с кодом {:?}",
                output.status.code()
            )));
        }
        Ok(output.stdout)
    }
}

impl VoiceBackend for CommandVoiceBackend {
    fn synthesize(&self, text: &str) -> Result<Vec<u8>, VoiceError> {
        Self::run_command(&self.tts_program, &self.tts_args, Some(text.as_bytes()))
    }

    fn transcribe(&self, audio: &[u8]) -> Result<String, VoiceError> {
        let stdout = Self::run_command(&self.stt_program, &self.stt_args, Some(audio))?;
        let text = String::from_utf8(stdout)?.trim().to_string();
        Ok(text)
    }
}

#[derive(Clone, Debug)]
pub struct CodecVoiceBackend {
    sample_rate: u32,
}

impl Default for CodecVoiceBackend {
    fn default() -> Self {
        Self {
            sample_rate: 16_000,
        }
    }
}

impl CodecVoiceBackend {
    pub fn new(sample_rate: u32) -> Self {
        Self { sample_rate }
    }

    fn encode(&self, text: &str) -> Result<Vec<u8>, VoiceError> {
        let chars: Vec<char> = text.chars().collect();
        let samples = chars.len() * 2;
        let data_size = (samples * 2) as u32;
        let mut buffer = Vec::with_capacity(44 + data_size as usize);
        buffer.extend_from_slice(b"RIFF");
        buffer.extend_from_slice(&(36 + data_size).to_le_bytes());
        buffer.extend_from_slice(b"WAVE");
        buffer.extend_from_slice(b"fmt ");
        buffer.extend_from_slice(&16u32.to_le_bytes());
        buffer.extend_from_slice(&1u16.to_le_bytes());
        buffer.extend_from_slice(&1u16.to_le_bytes());
        buffer.extend_from_slice(&self.sample_rate.to_le_bytes());
        let byte_rate = self.sample_rate * 2;
        buffer.extend_from_slice(&byte_rate.to_le_bytes());
        buffer.extend_from_slice(&2u16.to_le_bytes());
        buffer.extend_from_slice(&16u16.to_le_bytes());
        buffer.extend_from_slice(b"data");
        buffer.extend_from_slice(&data_size.to_le_bytes());
        for ch in chars {
            let code = ch as u32;
            let lower = (code & 0xFFFF) as u16;
            let upper = ((code >> 16) & 0xFFFF) as u16;
            buffer.extend_from_slice(&lower.to_le_bytes());
            buffer.extend_from_slice(&upper.to_le_bytes());
        }
        Ok(buffer)
    }

    fn decode(&self, audio: &[u8]) -> Result<String, VoiceError> {
        if audio.len() < 44 {
            return Err(VoiceError::with_context("слишком короткий WAV"));
        }
        if &audio[0..4] != b"RIFF" || &audio[8..12] != b"WAVE" {
            return Err(VoiceError::with_context("неподдерживаемый формат WAV"));
        }
        if &audio[12..16] != b"fmt " {
            return Err(VoiceError::with_context("отсутствует блок fmt"));
        }
        let bits_per_sample = u16::from_le_bytes([audio[34], audio[35]]);
        if bits_per_sample != 16 {
            return Err(VoiceError::with_context(
                "поддерживается только 16-битный PCM",
            ));
        }
        let channels = u16::from_le_bytes([audio[22], audio[23]]);
        if channels != 1 {
            return Err(VoiceError::with_context("поддерживается только моно"));
        }
        let mut offset = 36;
        while offset + 8 <= audio.len() {
            let chunk_id = &audio[offset..offset + 4];
            let chunk_size = u32::from_le_bytes([
                audio[offset + 4],
                audio[offset + 5],
                audio[offset + 6],
                audio[offset + 7],
            ]) as usize;
            offset += 8;
            if chunk_id == b"data" {
                if offset + chunk_size > audio.len() {
                    return Err(VoiceError::with_context("повреждённый блок data"));
                }
                let data = &audio[offset..offset + chunk_size];
                if data.len() % 4 != 0 {
                    return Err(VoiceError::with_context(
                        "длина блока data не кратна четырём",
                    ));
                }
                let mut text = String::new();
                for chunk in data.chunks_exact(4) {
                    let lower = u16::from_le_bytes([chunk[0], chunk[1]]) as u32;
                    let upper = u16::from_le_bytes([chunk[2], chunk[3]]) as u32;
                    let code = lower | (upper << 16);
                    match char::from_u32(code) {
                        Some(ch) => text.push(ch),
                        None => {
                            return Err(VoiceError::with_context("получен неверный код символа"));
                        }
                    }
                }
                return Ok(text);
            } else {
                offset += chunk_size;
            }
        }
        Err(VoiceError::with_context("в WAV нет блока data"))
    }
}

impl VoiceBackend for CodecVoiceBackend {
    fn synthesize(&self, text: &str) -> Result<Vec<u8>, VoiceError> {
        self.encode(text)
    }

    fn transcribe(&self, audio: &[u8]) -> Result<String, VoiceError> {
        self.decode(audio)
    }
}

#[cfg(test)]
mod tests {
    use super::{CodecVoiceBackend, VoiceBackend};

    #[test]
    fn codec_roundtrip_preserves_text() {
        let backend = CodecVoiceBackend::default();
        let phrase = "Привет, Нейра!";
        let wav = backend.synthesize(phrase).expect("encode");
        assert!(wav.len() > 44);
        let decoded = backend.transcribe(&wav).expect("decode");
        assert_eq!(decoded, phrase);
    }
}
