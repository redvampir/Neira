/* neira:meta
id: NEI-20280430-120000-healing-incident
intent: code
summary: |
  Описывает инциденты «Исцеляющего сна»: уровень, область, детали и полезные сигналы.
*/
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

fn default_timestamp() -> DateTime<Utc> {
    Utc::now()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for IncidentSeverity {
    fn default() -> Self {
        IncidentSeverity::Low
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentScope {
    Build,
    Runtime,
    DataPipeline,
    Interface,
    Connectivity,
    Unknown,
}

impl Default for IncidentScope {
    fn default() -> Self {
        IncidentScope::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingIncident {
    pub id: String,
    #[serde(default = "default_timestamp")]
    pub occurred_at: DateTime<Utc>,
    #[serde(default)]
    pub scope: IncidentScope,
    #[serde(default)]
    pub severity: IncidentSeverity,
    pub summary: String,
    #[serde(default)]
    pub details: String,
    #[serde(default)]
    pub signals: Vec<String>,
}

impl HealingIncident {
    pub fn new(id: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            occurred_at: Utc::now(),
            scope: IncidentScope::Unknown,
            severity: IncidentSeverity::Low,
            summary: summary.into(),
            details: String::new(),
            signals: Vec::new(),
        }
    }

    pub fn with_severity(mut self, severity: IncidentSeverity) -> Self {
        self.severity = severity;
        self
    }

    pub fn with_scope(mut self, scope: IncidentScope) -> Self {
        self.scope = scope;
        self
    }

    pub fn with_details<S: Into<String>>(mut self, details: S) -> Self {
        self.details = details.into();
        self
    }

    pub fn add_signal<S: Into<String>>(mut self, signal: S) -> Self {
        self.signals.push(signal.into());
        self
    }
}
