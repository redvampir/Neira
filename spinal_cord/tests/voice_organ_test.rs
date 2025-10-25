/* neira:meta
id: NEI-20280106-120000-voice-tests
intent: chore
summary: Покрывают голосовой орган: регистрация клеток, интеграция с фабрикой и TTS/STT через кодек.
*/
use backend::action::diagnostics_cell::DiagnosticsCell;
use backend::action::metrics_collector_cell::MetricsCollectorCell;
use backend::cell_registry::CellRegistry;
use backend::config::Config;
use backend::memory_cell::MemoryCell;
use backend::synapse_hub::SynapseHub;
use backend::voice::cells::{
    SPEAK_ACTION_ID, SPEAK_ADAPTER_ID, TEXT_NORMALIZE_ID, TEXT_TO_PHONEMES_ID,
};
use backend::voice::{VoiceConfig, VoiceOrgan};
use serial_test::serial;
use std::sync::Arc;

mod common;
use common::init_recorder;

struct EnvGuard {
    keys: Vec<&'static str>,
}

impl EnvGuard {
    fn set(pairs: &[(&'static str, String)]) -> Self {
        for (key, value) in pairs {
            std::env::set_var(key, value);
        }
        Self {
            keys: pairs.iter().map(|(key, _)| *key).collect(),
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for key in &self.keys {
            std::env::remove_var(key);
        }
    }
}

#[tokio::test]
#[serial]
async fn voice_bootstrap_registers_cells_and_factory_records() {
    let _metrics = init_recorder();
    let registry_dir = tempfile::tempdir().expect("registry dir");
    let output_dir = tempfile::tempdir().expect("voice output dir");
    let _env = EnvGuard::set(&[
        (
            "VOICE_OUTPUT_DIR",
            output_dir.path().to_string_lossy().into_owned(),
        ),
        ("FACTORY_ADAPTER_ENABLED", "1".to_string()),
        ("LYMPHATIC_FILTER_ENABLED", "0".to_string()),
    ]);

    let registry = Arc::new(CellRegistry::new(registry_dir.path()).expect("registry"));
    let memory = Arc::new(MemoryCell::new());
    let (metrics_cell, rx) = MetricsCollectorCell::channel();
    let (diagnostics, _dev_rx, _alert_rx) = DiagnosticsCell::new(rx, 5, metrics_cell.clone());
    let cfg = Config::default();
    let hub = Arc::new(SynapseHub::new(
        registry.clone(),
        memory,
        metrics_cell.clone(),
        diagnostics,
        &cfg,
    ));

    assert!(hub.factory_is_adapter_enabled());

    let voice_config = VoiceConfig::from_env();
    let organ = VoiceOrgan::new(Some(hub.clone()), registry.clone(), voice_config)
        .expect("voice organ init");
    organ.bootstrap().expect("voice organ bootstrap");

    assert!(registry.get(TEXT_NORMALIZE_ID).is_some());
    assert!(registry.get(TEXT_TO_PHONEMES_ID).is_some());
    assert!(registry.get(SPEAK_ADAPTER_ID).is_some());
    assert!(registry.get_analysis_cell(TEXT_NORMALIZE_ID).is_some());
    assert!(registry.get_analysis_cell(SPEAK_ADAPTER_ID).is_some());
    assert!(
        registry
            .action_cells()
            .iter()
            .any(|cell| cell.id() == SPEAK_ACTION_ID)
    );

    let (total, active) = hub.factory_counts();
    assert_eq!(total, 3, "фабрика должна создать три записи шаблонов");
    assert_eq!(active, 3, "все шаблоны должны быть активны после регистрации");
}

#[test]
#[serial]
fn voice_codec_backend_roundtrip() {
    let _metrics = init_recorder();
    let registry_dir = tempfile::tempdir().expect("registry dir");
    let output_dir = tempfile::tempdir().expect("voice output dir");
    let _env = EnvGuard::set(&[(
        "VOICE_OUTPUT_DIR",
        output_dir.path().to_string_lossy().into_owned(),
    )]);

    let registry = Arc::new(CellRegistry::new(registry_dir.path()).expect("registry"));
    let voice_config = VoiceConfig::from_env();
    let organ = VoiceOrgan::new(None, registry.clone(), voice_config).expect("voice organ init");
    organ.bootstrap().expect("voice organ bootstrap");

    let phrase = "Привет, Нейра!";
    let synthesis = organ
        .synthesize(None, phrase, None)
        .expect("synthesis successful");
    assert_eq!(synthesis.text, phrase);
    assert!(synthesis.file_path.exists());
    assert!(!synthesis.audio_base64.is_empty());

    let transcription = organ
        .transcribe_file(&synthesis.file_path)
        .expect("transcription successful");
    assert_eq!(transcription.text, phrase);
}
