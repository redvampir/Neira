use std::sync::Arc;

use crate::training::{
    curriculum::RussianLiteracyCurriculum,
    metrics::LearningMetrics,
    types::{Error, Lesson, SchedulerConfig},
};

pub struct AdaptiveScheduler {
    curriculum: Arc<RussianLiteracyCurriculum>,
    metrics: Arc<LearningMetrics>,
    config: SchedulerConfig,
}

impl AdaptiveScheduler {
    pub fn new(curriculum: Arc<RussianLiteracyCurriculum>, metrics: Arc<LearningMetrics>) -> Self {
        Self {
            curriculum,
            metrics,
            config: SchedulerConfig::default(),
        }
    }

    pub async fn next_lesson(&self) -> Lesson {
        let difficulty = self.metrics.adjust_difficulty().await;
        self.build_lesson(difficulty)
    }

    pub async fn complete_lesson(&self, lesson: &Lesson, success_rate: f64) -> Result<(), Error> {
        self.metrics
            .record_attempt(success_rate >= 0.7, lesson.difficulty)
            .await;

        self.metrics
            .save_progress()
            .await
            .map_err(|err| Error::Metrics(err.to_string()))
    }

    fn build_lesson(&self, difficulty: f64) -> Lesson {
        let words = self.candidate_words(difficulty);

        Lesson { words, difficulty }
    }

    fn candidate_words(&self, difficulty: f64) -> Vec<String> {
        let mut candidates: Vec<String> = self
            .curriculum
            .filter_by_difficulty(difficulty)
            .into_iter()
            .map(|entry| entry.word)
            .collect();

        if candidates.len() < self.config.min_words {
            candidates = self
                .curriculum
                .build_inquiry_seed()
                .into_iter()
                .map(|entry| entry.word)
                .collect();
        }

        candidates
            .into_iter()
            .take(self.config.max_words.max(self.config.min_words))
            .collect()
    }
}
