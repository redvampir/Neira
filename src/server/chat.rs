/* neira:meta
id: NEI-20251102-chat-training-name
intent: feature
summary: |
  Чат запоминает имя собеседника и умеет запускать тренировку по запросу
  прямо из диалога.
*/
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, OnceLock};

use axum::{extract::State, response::Json};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use super::analysis::{AnalysisOutcome, DialogueAnalysisCell, DialogueIntent};
use crate::training::metrics::LearningMetrics;

static ANALYSIS_CELL: OnceLock<DialogueAnalysisCell> = OnceLock::new();
static USER_NAME: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
static NAME_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(
            r"(?i)(?:меня зовут|зовут меня|зови меня|обращайся ко мне как|можешь звать меня)\s+([A-Za-zА-Яа-яЁё\- ]{2,40})",
        )
        .unwrap(),
        Regex::new(r"(?i)^я[-,:]?\s+([A-Za-zА-Яа-яЁё\-]{2,30})").unwrap(),
    ]
});

fn analyzer() -> &'static DialogueAnalysisCell {
    ANALYSIS_CELL.get_or_init(DialogueAnalysisCell::new)
}

#[derive(Debug, Deserialize)]
pub struct ChatMessage {
    text: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    response: String,
    current_difficulty: f64,
}

pub async fn handle_message(
    State(metrics): State<Arc<LearningMetrics>>,
    Json(msg): Json<ChatMessage>,
) -> Json<ChatResponse> {
    let analysis: AnalysisOutcome = analyzer().analyze(&msg.text);
    let mut intent = analysis.intent;
    let normalized = normalize_message(&msg.text);
    let name_candidate = extract_user_name(&msg.text);

    if analysis.confidence < 0.15 {
        intent = DialogueIntent::Unknown;
    }
    if name_candidate.is_some() {
        intent = DialogueIntent::IntroduceSelf;
    } else if matches_training_request(&normalized) {
        intent = DialogueIntent::TrainingStart;
    }

    info!(
        ?intent,
        confidence = analysis.confidence,
        matched = ?analysis.matched,
        "dialogue intent detected"
    );

    let current_difficulty = metrics.get_current_level().await;
    let success_rate = metrics.get_success_rate_metric();
    let remembered_name = current_user_name();

    let response = match intent {
        DialogueIntent::Greeting => match remembered_name {
            Some(name) => format!(
                "Привет, {name}! Рада, что ты заглянул(а). Чем займёмся сейчас?"
            ),
            None => String::from(
                "Привет! Рада, что ты заглянул(а). Подскажи, пожалуйста, как мне тебя называть?",
            ),
        },
        DialogueIntent::StatusCheck => {
            if success_rate > 0.8 {
                format!(
                    "Работаем отлично: успешность {:.1}%, текущая сложность {:.2}. Могу постепенно повысить планку или поддерживать её — как предпочтёшь.",
                    success_rate * 100.0,
                    current_difficulty
                )
            } else {
                format!(
                    "Есть куда расти: успешность сейчас {:.1}%, сложность {:.2}. Могу предложить больше тренировок или разобрать сложные шаги подробнее.",
                    success_rate * 100.0,
                    current_difficulty
                )
            }
        }
        DialogueIntent::IdentityQuery => String::from(
            "Я Нейра: автономный цифровой напарник. Учусь, анализирую и помогаю строить решения вместе с тобой.",
        ),
        DialogueIntent::CapabilitiesQuery => String::from(
            "Могу анализировать запросы, генерировать решения, запускать тренировочные сценарии и вести журнал прогресса. Говори, что важно, и я возьмусь.",
        ),
        DialogueIntent::AssistanceRequest => String::from(
            "Договорились, помогу. Расскажи, что именно хочешь сделать, и предложу следующий шаг или нужный инструмент.",
        ),
        DialogueIntent::TrainingStart => match trigger_training_run().await {
            Ok(()) => String::from(
                "Запускаю тренировку. Загляни на страницу «Обучение», там будет виден прогресс и журнал шагов.",
            ),
            Err(err) => format!(
                "Попыталась стартовать обучение, но возникла ошибка: {err}. Открой вкладку «Обучение» и попробуй запустить вручную. Если снова не получится — я рядом.",
            ),
        },
        DialogueIntent::StudyRussian => String::from(
            "Отличная идея! Могу предложить мини-упражнения, подобрать материал из контекста или устроить диалоговую практику. С чего начнём?",
        ),
        DialogueIntent::Gratitude => String::from(
            "Спасибо за тёплые слова! Это вдохновляет учиться дальше.",
        ),
        DialogueIntent::Farewell => String::from("До скорого! Если что — просто позови."),
        DialogueIntent::IntroduceSelf => {
            if let Some(raw) = name_candidate {
                if let Some(stored) = remember_user_name(&raw) {
                    format!("Приятно познакомиться, {stored}! Буду обращаться к тебе по имени.")
                } else {
                    String::from("Хочу запомнить имя, но не уверена, что поняла его правильно. Повтори, пожалуйста.")
                }
            } else {
                String::from("Хочу запомнить имя, но не уверена, что поняла его правильно. Повтори, пожалуйста.")
            }
        }
        DialogueIntent::StatusReport => format!(
            "Сводка по обучению:\n• Текущая сложность: {:.2}\n• Успешность: {:.1}%\n• Последние результаты сохранены в MemoryCell.\nЕсли нужно больше деталей — загляни в «Обучение».",
            current_difficulty,
            success_rate * 100.0,
        ),
        DialogueIntent::Reflection => String::from(
            "Я уже анализирую прошлые шаги: смотрю, где буксовала, и какие стратегии сработали. Могу поделиться выводами, если интересно.",
        ),
        DialogueIntent::Unknown => String::from(
            "Пока не уверена, как ответить. Попробуй сформулировать иначе или попроси «помощь».",
        ),
    };

    Json(ChatResponse {
        response,
        current_difficulty,
    })
}

fn normalize_message(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn matches_training_request(normalized: &str) -> bool {
    const KEYWORDS: [&str; 6] = [
        "обуч",
        "тренир",
        "займемся обучением",
        "займись обучением",
        "запусти тренировку",
        "давай учиться",
    ];
    KEYWORDS.iter().any(|kw| normalized.contains(kw))
}

fn extract_user_name(original: &str) -> Option<String> {
    for pattern in NAME_PATTERNS.iter() {
        if let Some(caps) = pattern.captures(original) {
            if let Some(mat) = caps.get(1) {
                let raw = mat.as_str().trim();
                if !raw.is_empty() {
                    return Some(raw.to_string());
                }
            }
        }
    }
    None
}

fn remember_user_name(raw: &str) -> Option<String> {
    let normalized = normalize_user_name(raw)?;
    let mut slot = USER_NAME.lock().unwrap();
    *slot = Some(normalized.clone());
    Some(normalized)
}

fn current_user_name() -> Option<String> {
    USER_NAME.lock().unwrap().clone()
}

fn normalize_user_name(raw: &str) -> Option<String> {
    let trimmed = raw
        .trim_matches(|c: char| !c.is_alphabetic() && c != '-' && c != ' ')
        .trim();
    if trimmed.is_empty() {
        return None;
    }
    let words: Vec<String> = trimmed
        .split_whitespace()
        .filter_map(|word| {
            if word.chars().all(|c| c.is_alphabetic() || c == '-') {
                Some(capitalize_word(word))
            } else {
                None
            }
        })
        .take(2)
        .collect();
    if words.is_empty() {
        None
    } else {
        Some(words.join(" "))
    }
}

fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    if let Some(first) = chars.next() {
        let mut result = String::new();
        result.extend(first.to_uppercase());
        for c in chars {
            result.extend(c.to_lowercase());
        }
        result
    } else {
        String::new()
    }
}

async fn trigger_training_run() -> Result<(), String> {
    let port = std::env::var("NEIRA_BIND_ADDR")
        .ok()
        .and_then(|addr| addr.parse::<SocketAddr>().map(|a| a.port()).ok())
        .unwrap_or(9090);
    let url = format!("http://127.0.0.1:{port}/api/neira/training/run");
    let client = reqwest::Client::new();
    client
        .post(url)
        .json(&json!({ "dry_run": false }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
