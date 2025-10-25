/* neira:meta
id: NEI-20280401-120040-curriculum-test
intent: feature
summary: Проверяет загрузку курса русской грамоты: память, событие и данные.
*/
/* neira:meta
id: NEI-20280425-120250-curriculum-cli-tests
intent: test
summary: |-
  Добавлены проверки конфигурации лимитов и сценарии CLI-инструмента
  curriculum_editor для гарантии совместимости со статистикой тем.
*/
use std::collections::HashSet;
use std::fs;
use std::process::Command;
use std::sync::{Arc, Mutex};

use backend::action::diagnostics_cell::DiagnosticsCell;
use backend::action::metrics_collector_cell::MetricsCollectorCell;
use backend::cell_registry::CellRegistry;
use backend::config::Config;
use backend::digestive_pipeline::ParsedInput;
use backend::event_bus::{Event, Subscriber};
use backend::memory_cell::MemoryCell;
use backend::training::curriculum::{
    default_curriculum_path,
    INQUIRY_SEED_LIMIT,
    RUSSIAN_CURRICULUM_ID,
    CurriculumError,
    RussianLiteracyCurriculum,
};
use backend::synapse_hub::SynapseHub;
use tempfile::NamedTempFile;

struct CaptureSubscriber {
    events: Arc<Mutex<Vec<serde_json::Value>>>,
}

impl Subscriber for CaptureSubscriber {
    fn on_event(&self, event: &dyn Event) {
        if event.name() == "training.curriculum.loaded" {
            if let Some(data) = event.data() {
                if let Ok(mut guard) = self.events.lock() {
                    guard.push(data);
                }
            }
        }
    }
}

#[tokio::test]
async fn literacy_curriculum_is_loaded_into_memory_and_event_bus() {
    let templates_dir = tempfile::tempdir().expect("templates dir");
    let registry = Arc::new(CellRegistry::new(templates_dir.path()).expect("registry"));
    let memory = Arc::new(MemoryCell::new());
    let (metrics, rx) = MetricsCollectorCell::channel();
    let (diagnostics, _dev_rx, _alert_rx) = DiagnosticsCell::new(rx, 8, metrics.clone());
    let cfg = Config::default();
    let hub = SynapseHub::new(registry, memory.clone(), metrics, diagnostics, &cfg);

    let captured = Arc::new(Mutex::new(Vec::new()));
    hub.subscribe_event(Arc::new(CaptureSubscriber {
        events: captured.clone(),
    }));

    let curriculum = hub
        .train_russian_literacy(None)
        .expect("curriculum loaded");

    assert_eq!(curriculum.id(), RUSSIAN_CURRICULUM_ID);
    let word_count = curriculum.words.len();
    assert!(
        word_count > 120,
        "curriculum should load expanded vocabulary beyond 120 words"
    );
    let summary = curriculum.summary();
    assert_eq!(summary.letters, 33);
    assert_eq!(summary.words, word_count);

    let parsed = memory.parsed_inputs();
    let last = parsed.last().expect("parsed input stored");
    match last {
        ParsedInput::Json(value) => {
            assert_eq!(value.get("id"), Some(&serde_json::Value::String(RUSSIAN_CURRICULUM_ID.into())));
        }
        ParsedInput::Text(_) => panic!("expected json payload"),
    }

    let events = captured.lock().expect("event lock");
    assert_eq!(events.len(), 1);
    let data = &events[0];
    assert_eq!(data.get("curriculum_id"), Some(&serde_json::Value::String(RUSSIAN_CURRICULUM_ID.into())));
    assert_eq!(data.get("letters"), Some(&serde_json::Value::from(33)));
    assert_eq!(
        data.get("words"),
        Some(&serde_json::Value::from(word_count))
    );
    let themes = data
        .get("themes")
        .and_then(|value| value.as_object())
        .expect("themes map should be published");
    let total_in_themes: usize = themes.values().map(|value| value.as_u64().unwrap_or_default() as usize).sum();
    assert_eq!(
        total_in_themes,
        word_count,
        "сумма слов по темам должна совпадать с общим числом слов"
    );

    let seed = curriculum.build_inquiry_seed();
    assert!(!seed.is_empty(), "seed selection should not be empty");
    assert!(
        seed.len() <= INQUIRY_SEED_LIMIT,
        "seed should respect configured limit"
    );
    let seed_words: Vec<&str> = seed.iter().map(|word| word.word.as_str()).collect();
    let question_words = [
        "что",
        "кто",
        "где",
        "когда",
        "почему",
        "как",
        "это",
        "там",
        "здесь",
        "какой",
        "какая",
    ];
    for expected in question_words {
        assert!(
            seed_words.contains(&expected),
            "seed must contain question word {expected}"
        );
    }
    let question_theme_count = seed
        .iter()
        .filter(|entry| entry.theme == "вопросы")
        .count();
    assert!(
        question_theme_count >= question_words.len(),
        "all question words should be marked with theme 'вопросы'"
    );
    assert!(
        seed.iter().all(|entry| entry.level <= 1),
        "seed words should remain in the basic difficulty range"
    );
}

#[test]
fn training_config_limit_has_priority_over_env() {
    let temp_config = NamedTempFile::new().expect("temp config");
    std::fs::write(
        &temp_config,
        "[training]\nmax_words = 10\n",
    )
    .expect("write config");
    std::env::set_var("TRAINING_CONFIG_PATH", temp_config.path());
    std::env::set_var("RUSSIAN_CURRICULUM_MAX_WORDS", "200");

    let result = RussianLiteracyCurriculum::load_default();
    std::env::remove_var("TRAINING_CONFIG_PATH");
    std::env::remove_var("RUSSIAN_CURRICULUM_MAX_WORDS");

    match result {
        Err(CurriculumError::Validation(message)) => {
            assert!(
                message.contains("не более"),
                "ожидаем сообщение об ограничении слов, получили: {message}"
            );
        }
        other => panic!("ожидалась ошибка из-за конфигурации, получено {other:?}"),
    }
}

#[test]
fn curriculum_editor_reports_stats() {
    let binary = env!("CARGO_BIN_EXE_curriculum_editor");
    let output = Command::new(binary)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .arg("--stats")
        .output()
        .expect("запуск curriculum_editor --stats");
    assert!(output.status.success(), "утилита завершилась с ошибкой");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Темы словаря"),
        "ожидаем увидеть статистику тем, получили: {stdout}"
    );
}

#[test]
fn curriculum_editor_adds_word_non_interactively() {
    let default_path = default_curriculum_path();
    let dataset = fs::read_to_string(&default_path).expect("read dataset");
    let temp_dataset = NamedTempFile::new().expect("temp dataset");
    fs::write(temp_dataset.path(), dataset).expect("write dataset copy");

    let curriculum = RussianLiteracyCurriculum::load_from_path(temp_dataset.path())
        .expect("load temp dataset");
    let existing: HashSet<String> =
        curriculum.words.iter().map(|entry| entry.word.clone()).collect();
    let mut candidate: Option<(String, Vec<String>)> = None;
    for first in &curriculum.syllables {
        let syll_a = first.syllable.trim();
        if syll_a.is_empty() || syll_a.contains(' ') {
            continue;
        }
        for second in &curriculum.syllables {
            let syll_b = second.syllable.trim();
            if syll_b.is_empty() || syll_b.contains(' ') {
                continue;
            }
            let word = format!("{}{}", syll_a, syll_b);
            if !existing.contains(&word) {
                candidate = Some((
                    word,
                    vec![syll_a.to_string(), syll_b.to_string()],
                ));
                break;
            }
        }
        if candidate.is_some() {
            break;
        }
    }
    let (new_word, syllables) = candidate.expect("failed to generate unique word");
    let syllables_arg = syllables.join(",");

    let binary = env!("CARGO_BIN_EXE_curriculum_editor");
    let output = Command::new(binary)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--path",
            temp_dataset.path().to_str().expect("dataset path"),
            "--add-word",
            "--word",
            &new_word,
            "--syllables",
            &syllables_arg,
            "--meaning",
            "тестовое слово",
            "--theme",
            "тесты",
            "--level",
            "1",
        ])
        .output()
        .expect("run curriculum_editor add word");
    assert!(
        output.status.success(),
        "curriculum_editor завершилась с ошибкой: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let updated = RussianLiteracyCurriculum::load_from_path(temp_dataset.path())
        .expect("load updated dataset");
    assert!(
        updated.words.iter().any(|entry| entry.word == new_word),
        "слово должно быть добавлено"
    );
}

#[test]
fn russian_curriculum_handles_extended_wordset() {
    std::env::remove_var("RUSSIAN_CURRICULUM_MAX_WORDS");
    let curriculum = RussianLiteracyCurriculum::load_default()
        .expect("curriculum should load without word limit");
    assert!(
        curriculum.words.len() > 120,
        "expanded word list must exceed previous 120-word cap"
    );
    let summary = curriculum.summary();
    assert_eq!(summary.words, curriculum.words.len());
    assert!(summary.syllables >= summary.words);
}

