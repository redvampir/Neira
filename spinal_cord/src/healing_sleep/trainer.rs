/* neira:meta
id: NEI-20280430-120300-healing-trainer
intent: code
summary: |
  Мини-тренер подбирает формулировки вопросов и хранит примеры для «исцеляющего сна».
*/
use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use super::advisor::ConsultationPrompt;
use super::incident::{HealingIncident, IncidentSeverity};
use super::journal::LoggedIncident;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTrainingConfig {
    #[serde(default = "default_prompt_memory")]
    pub prompt_memory: usize,
    #[serde(default = "default_max_training_samples")]
    pub max_training_samples: usize,
    #[serde(default)]
    pub include_signals: bool,
}

impl Default for PromptTrainingConfig {
    fn default() -> Self {
        Self {
            prompt_memory: default_prompt_memory(),
            max_training_samples: default_max_training_samples(),
            include_signals: true,
        }
    }
}

const fn default_prompt_memory() -> usize {
    5
}

const fn default_max_training_samples() -> usize {
    16
}

#[derive(Debug, Clone)]
struct PromptExample {
    summary: String,
    advice: Option<String>,
}

#[derive(Debug)]
pub struct InsightTrainer {
    memory: VecDeque<PromptExample>,
}

impl InsightTrainer {
    pub fn new() -> Self {
        Self {
            memory: VecDeque::new(),
        }
    }

    pub fn learn(
        &mut self,
        incident: &HealingIncident,
        consultation: Option<&str>,
        config: &PromptTrainingConfig,
    ) {
        let example = PromptExample {
            summary: incident.summary.clone(),
            advice: consultation.map(ToOwned::to_owned),
        };
        self.memory.push_back(example);

        let target = config.max_training_samples.max(config.prompt_memory).max(8);
        while self.memory.len() > target {
            self.memory.pop_front();
        }
    }

    pub fn prepare_prompt(
        &self,
        incident: &HealingIncident,
        journal: &[LoggedIncident],
        config: &PromptTrainingConfig,
    ) -> ConsultationPrompt {
        let mut context_sections = Vec::new();

        if !journal.is_empty() {
            let mut lines = Vec::new();
            for logged in journal.iter().rev().take(config.prompt_memory) {
                let severity = logged.incident.severity;
                let scope = format!("{:?}", logged.incident.scope);
                let base = format!(
                    "- [{}][{}] {}",
                    format_severity(severity),
                    scope,
                    logged.incident.summary
                );
                lines.push(match &logged.consultation {
                    Some(advice) => format!("{base}\n  ↳ совет: {}", advice.advice),
                    None => base,
                });
            }
            context_sections.push(format!(
                "Последние похожие инциденты:\n{}",
                lines.join("\n")
            ));
        }

        if config.include_signals && !incident.signals.is_empty() {
            context_sections.push(format!(
                "Сигналы и метрики: {}",
                incident.signals.join(", ")
            ));
        }

        if !self.memory.is_empty() {
            let mut memo = Vec::new();
            for example in self.memory.iter().rev().take(config.prompt_memory) {
                let mut line = format!("• {}", example.summary);
                if let Some(advice) = &example.advice {
                    line.push_str(&format!(" → {}", advice));
                }
                memo.push(line);
            }
            context_sections.push(format!(
                "Извлечённые уроки («сонный дайджест»):\n{}",
                memo.join("\n")
            ));
        }

        let context = if context_sections.is_empty() {
            "Предыдущих подсказок нет; требуется свежий взгляд.".to_string()
        } else {
            context_sections.join("\n\n")
        };

        let expectations = "Ответь кратко и конкретно: что проверить, как воспроизвести и какие шаги принять. Если совет не применим, предложи альтернативы."
            .to_string();

        let mut prompt = ConsultationPrompt::new(
            format!(
                "Проблема: {} (детали: {}). Как стабилизировать систему?",
                incident.summary, incident.details
            ),
            context,
            expectations,
            incident.severity,
        );

        prompt = prompt.with_label("scope", format!("{:?}", incident.scope));
        prompt = prompt.with_label("incident_id", incident.id.clone());

        prompt
    }
}

fn format_severity(severity: IncidentSeverity) -> &'static str {
    match severity {
        IncidentSeverity::Low => "низкий",
        IncidentSeverity::Medium => "средний",
        IncidentSeverity::High => "высокий",
        IncidentSeverity::Critical => "критический",
    }
}
