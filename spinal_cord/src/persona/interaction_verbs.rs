/* neira:meta
id: NEI-20280502-120000-interaction-verbs
intent: feature
summary: |-
  Детектор глаголов взаимодействия определяет ожидаемое действие собеседника,
  нормализует сообщения и публикует событие для шины EventBus.
*/

use crate::event_bus::Event;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use serde_json::json;
use std::any::Any;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionVerb {
    Give,
    Show,
    Explain,
    Repeat,
    Find,
}

impl InteractionVerb {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Give => "give",
            Self::Show => "show",
            Self::Explain => "explain",
            Self::Repeat => "repeat",
            Self::Find => "find",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionVerbActor {
    User,
    Assistant,
}

impl InteractionVerbActor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Assistant => "assistant",
        }
    }
}

#[derive(Debug)]
struct VerbPattern {
    verb: InteractionVerb,
    regex: Regex,
}

static VERB_PATTERNS: Lazy<Vec<VerbPattern>> = Lazy::new(|| {
    vec![
        VerbPattern {
            verb: InteractionVerb::Give,
            regex: Regex::new(r"(?iu)\bдай(те)?\b").unwrap(),
        },
        VerbPattern {
            verb: InteractionVerb::Show,
            regex: Regex::new(r"(?iu)\bпокаж(и|ите)\b").unwrap(),
        },
        VerbPattern {
            verb: InteractionVerb::Explain,
            regex: Regex::new(r"(?iu)\bобъясн(и|ите)\b").unwrap(),
        },
        VerbPattern {
            verb: InteractionVerb::Repeat,
            regex: Regex::new(r"(?iu)\bповтор(и|ите)\b").unwrap(),
        },
        VerbPattern {
            verb: InteractionVerb::Find,
            regex: Regex::new(r"(?iu)\bнайд(и|ите)\b").unwrap(),
        },
    ]
});

#[derive(Debug, Clone, Default)]
pub struct InteractionVerbDetector;

impl InteractionVerbDetector {
    pub fn detect(&self, text: &str) -> Vec<InteractionVerb> {
        let mut found = Vec::new();
        for pattern in VERB_PATTERNS.iter() {
            if pattern.regex.is_match(text) && !found.contains(&pattern.verb) {
                found.push(pattern.verb);
            }
        }
        found
    }

    pub fn detect_primary(&self, text: &str) -> Option<InteractionVerb> {
        VERB_PATTERNS
            .iter()
            .filter_map(|pattern| pattern.regex.find(text).map(|m| (m.start(), pattern.verb)))
            .min_by_key(|(idx, _)| *idx)
            .map(|(_, verb)| verb)
    }
}

#[derive(Debug, Clone)]
pub struct InteractionVerbObserved {
    actor: InteractionVerbActor,
    verb: InteractionVerb,
    chat_id: String,
    session_id: Option<String>,
    message_preview: String,
}

impl InteractionVerbObserved {
    pub fn new(
        actor: InteractionVerbActor,
        verb: InteractionVerb,
        chat_id: impl Into<String>,
        session_id: Option<String>,
        message: &str,
    ) -> Self {
        Self {
            actor,
            verb,
            chat_id: chat_id.into(),
            session_id,
            message_preview: sanitize_preview(message),
        }
    }

    pub fn verb(&self) -> InteractionVerb {
        self.verb
    }

    pub fn actor(&self) -> InteractionVerbActor {
        self.actor
    }

    pub fn chat_id(&self) -> &str {
        &self.chat_id
    }

    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    pub fn message_preview(&self) -> &str {
        &self.message_preview
    }
}

impl Event for InteractionVerbObserved {
    fn name(&self) -> &str {
        "persona.interaction_verb.observed"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn data(&self) -> Option<serde_json::Value> {
        Some(json!({
            "actor": self.actor.as_str(),
            "verb": self.verb.as_str(),
            "chat_id": self.chat_id,
            "session_id": self.session_id.clone(),
            "message_preview": self.message_preview,
        }))
    }
}

const PREVIEW_MAX_CHARS: usize = 120;

fn sanitize_preview(message: &str) -> String {
    let normalized = message
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let normalized = normalized.trim();
    if normalized.is_empty() {
        return String::new();
    }
    let mut preview = String::new();
    let mut truncated = false;
    let mut count = 0usize;
    for ch in normalized.chars() {
        if count >= PREVIEW_MAX_CHARS {
            truncated = true;
            break;
        }
        preview.push(ch);
        count += 1;
    }
    if truncated {
        preview.push('…');
    }
    preview
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_multiple_verbs_and_primary() {
        let detector = InteractionVerbDetector::default();
        let verbs = detector.detect("Пожалуйста, покажите и объясните ещё раз.");
        assert!(verbs.contains(&InteractionVerb::Show));
        assert!(verbs.contains(&InteractionVerb::Explain));
        assert_eq!(verbs.len(), 2);
        let primary = detector
            .detect_primary("Объясни и покажи, а потом найди")
            .expect("primary verb");
        assert_eq!(primary, InteractionVerb::Explain);
    }

    #[test]
    fn sanitize_preview_trims_and_limits() {
        let preview = sanitize_preview("  дай\nпожалуйста   список   ");
        assert_eq!(preview, "дай пожалуйста список");
        let long_text = "найди ".repeat(40);
        let preview_long = sanitize_preview(&long_text);
        assert!(preview_long.chars().count() <= PREVIEW_MAX_CHARS + 1);
        if preview_long.chars().count() > PREVIEW_MAX_CHARS {
            assert!(preview_long.ends_with('…'));
        }
    }
}
