/* neira:meta
id: NEI-20250829-175425-trigger-detector
intent: docs
summary: |
  Выявляет ключевые слова и запускает микрорефлексы.
*/
/* neira:meta
id: NEI-20270405-trigger-detector-lowercase
intent: refactor
summary: Убрана дублирующая проверка регистра в detect_text.
*/

use crate::digestive_pipeline::{DigestivePipeline, ParsedInput};
use std::sync::RwLock;

type ReflexAction = Box<dyn Fn() + Send + Sync>;

struct MicroReflex {
    pattern: String,
    action: ReflexAction,
}

pub struct TriggerDetector {
    keywords: RwLock<Vec<String>>,
    micro_reflexes: RwLock<Vec<MicroReflex>>,
}

impl Default for TriggerDetector {
    fn default() -> Self {
        let defaults = vec![
            "биология".to_string(),
            "программирование".to_string(),
            "rust".to_string(),
            "математика".to_string(),
            "нейросети".to_string(),
        ];
        Self {
            keywords: RwLock::new(defaults),
            micro_reflexes: RwLock::new(Vec::new()),
        }
    }
}

impl TriggerDetector {
    pub fn add_keyword(&self, keyword: String) {
        self.keywords.write().unwrap().push(keyword.to_lowercase());
    }

    pub fn add_micro_reflex<F>(&self, pattern: impl Into<String>, action: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.micro_reflexes.write().unwrap().push(MicroReflex {
            pattern: pattern.into().to_lowercase(),
            action: Box::new(action),
        });
    }

    /* neira:meta
    id: NEI-20260530-trigger-digest
    intent: refactor
    summary: Использует DigestivePipeline для предварительной обработки входа.
    */
    pub fn detect(&self, raw: &str) -> Vec<String> {
        let text = match DigestivePipeline::ingest(raw) {
            Ok(ParsedInput::Json(v)) => v.to_string(),
            Ok(ParsedInput::Text(t)) => t,
            Err(_) => DigestivePipeline::sanitize(raw),
        };
        self.detect_text(&text)
    }

    fn detect_text(&self, text: &str) -> Vec<String> {
        let lower = text.to_lowercase();
        let kws = self.keywords.read().unwrap();
        let found: Vec<String> = kws
            .iter()
            .filter(|k| lower.contains(k.as_str()))
            .cloned()
            .collect();
        let reflexes = self.micro_reflexes.read().unwrap();
        for reflex in reflexes.iter() {
            if lower.contains(&reflex.pattern) {
                (reflex.action)();
            }
        }
        found
    }
}
