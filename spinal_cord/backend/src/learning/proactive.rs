use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningRequest {
    pub phrase: String,
    pub context: String,
    pub approved: bool,
    pub source: Option<String>,
}

pub struct ProactiveLearning {
    unknown_phrases: RwLock<HashSet<String>>,
    learning_requests: RwLock<Vec<LearningRequest>>,
}

impl ProactiveLearning {
    pub fn new() -> Self {
        Self {
            unknown_phrases: RwLock::new(HashSet::new()),
            learning_requests: RwLock::new(Vec::new()),
        }
    }

    pub async fn is_unknown(&self, phrase: &str) -> bool {
        let phrases = self.unknown_phrases.read().await;
        phrases.contains(phrase)
    }

    pub async fn request_learning(&self, phrase: &str, context: &str) -> LearningRequest {
        let req = LearningRequest {
            phrase: phrase.to_string(),
            context: context.to_string(),
            approved: false,
            source: None,
        };
        self.learning_requests.write().await.push(req.clone());
        req
    }
}
