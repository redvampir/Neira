/* neira:meta
id: NEI-20280430-120200-healing-journal
intent: code
summary: |
  Журнал инцидентов и советов для цикла «Исцеляющий сон».
*/
use std::collections::VecDeque;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::advisor::ConsultationRecord;
use super::incident::HealingIncident;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggedIncident {
    pub incident: HealingIncident,
    pub recorded_at: DateTime<Utc>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub consultation: Option<ConsultationRecord>,
}

impl LoggedIncident {
    pub fn new(
        incident: HealingIncident,
        notes: Option<String>,
        consultation: Option<ConsultationRecord>,
    ) -> Self {
        Self {
            incident,
            recorded_at: Utc::now(),
            notes,
            consultation,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HealingJournal {
    limit: usize,
    entries: VecDeque<LoggedIncident>,
}

impl HealingJournal {
    pub fn new(limit: usize) -> Self {
        Self {
            limit: limit.max(8),
            entries: VecDeque::new(),
        }
    }

    pub fn push(&mut self, entry: LoggedIncident) {
        if self.entries.len() >= self.limit {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn recent(&self, count: usize) -> Vec<LoggedIncident> {
        self.entries.iter().rev().take(count).cloned().collect()
    }

    pub fn entries(&self) -> Vec<LoggedIncident> {
        self.entries.iter().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
