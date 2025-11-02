/* neira:meta
id: NEI-20280430-120500-healing-sleep
intent: feature
summary: |
  «Исцеляющий сон» — модуль, который фиксирует инциденты, собирает советы и учится их уточнять.
*/
use std::path::Path;
use std::sync::{Arc, OnceLock};

use chrono::Utc;
use metrics::{counter, gauge};
use tokio::sync::RwLock;
use tracing::{debug, warn};

pub mod advisor;
pub mod config;
mod incident;
mod journal;
pub mod openai;
mod trainer;

pub use advisor::{
    ConsultationError, ConsultationPrompt, ConsultationRecord, ExternalConsultant, NoOpConsultant,
};
pub use config::{ConsultantSettings, HealingSleepConfig, HealingSleepConfigError};
pub use incident::{HealingIncident, IncidentScope, IncidentSeverity};
pub use journal::{HealingJournal, LoggedIncident};
pub use trainer::PromptTrainingConfig;

use openai::OpenAiConsultant;
use trainer::InsightTrainer;

pub static GLOBAL_HEALING_SLEEP: OnceLock<Arc<HealingSleep>> = OnceLock::new();

const INCIDENT_COUNTER: &str = "healing_sleep_incidents_total";
const CONSULT_COUNTER: &str = "healing_sleep_consultations_total";
const CONSULT_FAIL_COUNTER: &str = "healing_sleep_consultation_failures_total";
const JOURNAL_GAUGE: &str = "healing_sleep_journal_size";
const LAST_ACTIVITY_GAUGE: &str = "healing_sleep_last_incident_minutes";

#[derive(Debug, Clone)]
pub enum HealingOutcome {
    Disabled {
        incident: HealingIncident,
    },
    Logged {
        incident: HealingIncident,
        prompt: ConsultationPrompt,
    },
    LoggedWithReason {
        incident: HealingIncident,
        prompt: ConsultationPrompt,
        reason: String,
    },
    Consulted {
        incident: HealingIncident,
        prompt: ConsultationPrompt,
        consultation: ConsultationRecord,
    },
}

pub struct HealingSleep {
    config: HealingSleepConfig,
    journal: RwLock<HealingJournal>,
    trainer: RwLock<InsightTrainer>,
    consultant: RwLock<Option<Arc<dyn ExternalConsultant>>>,
}

impl HealingSleep {
    pub fn new(
        config: HealingSleepConfig,
        consultant: Option<Arc<dyn ExternalConsultant>>,
    ) -> Self {
        let journal_limit = config.journal_limit;
        Self {
            config,
            journal: RwLock::new(HealingJournal::new(journal_limit)),
            trainer: RwLock::new(InsightTrainer::new()),
            consultant: RwLock::new(consultant),
        }
    }

    pub fn from_config_file(
        path: impl AsRef<Path>,
        consultant: Option<Arc<dyn ExternalConsultant>>,
    ) -> Result<Self, HealingSleepConfigError> {
        let config = HealingSleepConfig::load(path)?;
        let auto_consultant = consultant.or_else(|| build_consultant_from_config(&config));
        Ok(Self::new(config, auto_consultant))
    }

    pub fn register_global(instance: Arc<Self>) {
        let _ = GLOBAL_HEALING_SLEEP.set(instance);
    }

    pub async fn record_incident(&self, incident: HealingIncident) -> HealingOutcome {
        counter!(INCIDENT_COUNTER).increment(1);

        if !self.config.enabled {
            return HealingOutcome::Disabled { incident };
        }

        let snapshot = {
            let journal = self.journal.read().await;
            journal.recent(self.config.prompt.prompt_memory)
        };

        let prompt = {
            let trainer = self.trainer.read().await;
            trainer.prepare_prompt(&incident, &snapshot, &self.config.prompt)
        };

        let mut reason: Option<String> = None;
        if incident.severity < self.config.min_severity {
            reason = Some(format!(
                "серьёзность {:?} ниже порога {:?}",
                incident.severity, self.config.min_severity
            ));
        }

        if reason.is_none() && !self.config.allow_external_consultant {
            reason = Some("политика отключила внешние консультации".into());
        }

        let consultation = if reason.is_none() {
            let consultant = {
                let guard = self.consultant.read().await;
                guard.clone()
            };
            match consultant {
                Some(provider) => match provider.consult(&prompt).await {
                    Ok(record) => {
                        counter!(CONSULT_COUNTER).increment(1);
                        Some(record)
                    }
                    Err(err) => {
                        counter!(CONSULT_FAIL_COUNTER).increment(1);
                        reason = Some(err.to_string());
                        None
                    }
                },
                None => {
                    reason = Some("консультант не настроен".into());
                    None
                }
            }
        } else {
            None
        };

        {
            let mut trainer = self.trainer.write().await;
            trainer.learn(
                &incident,
                consultation.as_ref().map(|c| c.advice.as_str()),
                &self.config.prompt,
            );
        }

        let outcome = match (consultation.clone(), reason.clone()) {
            (Some(record), _) => HealingOutcome::Consulted {
                incident: incident.clone(),
                prompt: prompt.clone(),
                consultation: record,
            },
            (None, Some(reason)) if reason.is_empty() => HealingOutcome::Logged {
                incident: incident.clone(),
                prompt: prompt.clone(),
            },
            (None, Some(reason)) => HealingOutcome::LoggedWithReason {
                incident: incident.clone(),
                prompt: prompt.clone(),
                reason,
            },
            (None, None) => HealingOutcome::Logged {
                incident: incident.clone(),
                prompt: prompt.clone(),
            },
        };

        {
            let mut journal = self.journal.write().await;
            journal.push(LoggedIncident::new(incident.clone(), reason, consultation));
            gauge!(JOURNAL_GAUGE).set(journal.len() as f64);
        }

        if let Some(minutes) = self.last_activity_minutes().await {
            gauge!(LAST_ACTIVITY_GAUGE).set(minutes as f64);
        }

        debug!(incident_id = %incident.id, "healing sleep recorded incident");
        outcome
    }

    pub async fn journal_snapshot(&self, count: usize) -> Vec<LoggedIncident> {
        let journal = self.journal.read().await;
        journal.recent(count)
    }

    pub async fn full_journal(&self) -> Vec<LoggedIncident> {
        let journal = self.journal.read().await;
        journal.entries()
    }

    pub fn config(&self) -> &HealingSleepConfig {
        &self.config
    }

    pub async fn set_consultant(&self, consultant: Option<Arc<dyn ExternalConsultant>>) {
        let mut guard = self.consultant.write().await;
        *guard = consultant;
    }

    pub async fn incidents_recorded(&self) -> usize {
        let journal = self.journal.read().await;
        journal.len()
    }

    pub async fn last_activity_minutes(&self) -> Option<u64> {
        let journal = self.journal.read().await;
        journal
            .entries()
            .last()
            .map(|entry| (Utc::now() - entry.recorded_at).num_minutes() as u64)
    }
}

fn build_consultant_from_config(
    config: &HealingSleepConfig,
) -> Option<Arc<dyn ExternalConsultant>> {
    if !config.allow_external_consultant {
        return None;
    }
    let provider = config
        .consultant
        .provider
        .as_deref()
        .or(config.preferred_consultant.as_deref())
        .unwrap_or("openai");

    match provider {
        "openai" | "gpt" | "openai:gpt" | "openai-chat" => {
            match OpenAiConsultant::from_config(config) {
                Ok(consultant) => {
                    debug!("создан консультант OpenAI для исцеляющего сна");
                    Some(Arc::new(consultant))
                }
                Err(err) => {
                    warn!(%err, "не удалось инициализировать консультанта OpenAI");
                    None
                }
            }
        }
        other => {
            warn!(provider = other, "неизвестный поставщик консультанта");
            None
        }
    }
}

pub async fn record_global_incident(incident: HealingIncident) -> Option<HealingOutcome> {
    let healing = GLOBAL_HEALING_SLEEP.get()?;
    Some(healing.record_incident(incident).await)
}
