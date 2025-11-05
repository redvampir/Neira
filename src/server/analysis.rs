use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogueIntent {
    Greeting,
    StatusCheck,
    IdentityQuery,
    CapabilitiesQuery,
    AssistanceRequest,
    TrainingStart,
    StudyRussian,
    Gratitude,
    Farewell,
    StatusReport,
    Reflection,
    IntroduceSelf,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct AnalysisOutcome {
    pub intent: DialogueIntent,
    pub matched: Vec<String>,
    pub confidence: f32,
}

pub struct DialogueAnalysisCell {
    lexicon: Vec<(DialogueIntent, Vec<&'static str>)>,
    priority: HashMap<DialogueIntent, u8>,
}

impl Default for DialogueAnalysisCell {
    fn default() -> Self {
        Self::new()
    }
}

impl DialogueAnalysisCell {
    pub fn new() -> Self {
        let lexicon = vec![
            (
                DialogueIntent::StatusCheck,
                vec![
                    "как дела",
                    "как ты",
                    "как настроение",
                    "как поживаешь",
                    "как сам",
                    "как сама",
                    "как чувствуешь себя",
                ],
            ),
            (
                DialogueIntent::Greeting,
                vec![
                    "привет",
                    "здравствуй",
                    "здравствуйте",
                    "приветствую",
                    "добрый день",
                    "добрый вечер",
                    "доброе утро",
                    "хай",
                ],
            ),
            (
                DialogueIntent::IdentityQuery,
                vec!["кто ты", "что ты", "расскажи о себе", "ты кто", "кто такая"],
            ),
            (
                DialogueIntent::CapabilitiesQuery,
                vec![
                    "что умеешь",
                    "что можешь",
                    "какие функции",
                    "что умеешь делать",
                    "что ты умеешь",
                ],
            ),
            (
                DialogueIntent::AssistanceRequest,
                vec![
                    "помоги",
                    "нужна помощь",
                    "подскажи",
                    "что делать",
                    "как поступить",
                    "совет",
                ],
            ),
            (
                DialogueIntent::StudyRussian,
                vec![
                    "учить русский",
                    "изучать русский",
                    "урок русского",
                    "орфография",
                    "грамматика",
                    "правила русского",
                ],
            ),
            (
                DialogueIntent::Gratitude,
                vec!["спасибо", "благодарю", "благодарен", "благодарна"],
            ),
            (
                DialogueIntent::Farewell,
                vec!["пока", "до связи", "до встречи", "увидимся", "до завтра"],
            ),
            (
                DialogueIntent::StatusReport,
                vec!["статус", "отчет", "отчёт", "как продвигается", "сводку"],
            ),
            (
                DialogueIntent::Reflection,
                vec![
                    "как ты сама думаешь",
                    "как думаешь ты сама",
                    "что ты сама думаешь",
                    "а ты как считаешь",
                ],
            ),
        ];

        let priority = HashMap::from([
            (DialogueIntent::StatusCheck, 6),
            (DialogueIntent::Greeting, 1),
            (DialogueIntent::IdentityQuery, 3),
            (DialogueIntent::CapabilitiesQuery, 4),
            (DialogueIntent::AssistanceRequest, 5),
            (DialogueIntent::TrainingStart, 6),
            (DialogueIntent::StudyRussian, 5),
            (DialogueIntent::Gratitude, 2),
            (DialogueIntent::Farewell, 2),
            (DialogueIntent::StatusReport, 5),
            (DialogueIntent::Reflection, 4),
            (DialogueIntent::IntroduceSelf, 7),
            (DialogueIntent::Unknown, 0),
        ]);

        Self { lexicon, priority }
    }

    pub fn analyze(&self, text: &str) -> AnalysisOutcome {
        let sanitized = Self::sanitize(text);
        let total_len = sanitized.chars().filter(|c| !c.is_whitespace()).count() as f32;

        if sanitized.is_empty() || total_len == 0.0 {
            return AnalysisOutcome {
                intent: DialogueIntent::Unknown,
                matched: Vec::new(),
                confidence: 0.0,
            };
        }

        let mut best_intent = DialogueIntent::Unknown;
        let mut best_matches: Vec<String> = Vec::new();
        let mut best_score = 0.0;
        let mut best_priority = 0_u8;

        for (intent, keywords) in &self.lexicon {
            let mut matches = Vec::new();
            let mut score = 0.0;

            for &keyword in keywords {
                if sanitized.contains(keyword) {
                    matches.push(keyword.to_string());
                    score += keyword.chars().filter(|c| !c.is_whitespace()).count() as f32;
                }
            }

            if !matches.is_empty() {
                let intent_priority = *self.priority.get(intent).unwrap_or(&1);
                if score > best_score || (score == best_score && intent_priority > best_priority) {
                    best_score = score;
                    best_matches = matches;
                    best_intent = *intent;
                    best_priority = intent_priority;
                }
            }
        }

        let confidence = if best_score > 0.0 {
            (best_score / total_len).clamp(0.0, 1.0)
        } else {
            0.0
        };

        AnalysisOutcome {
            intent: best_intent,
            matched: best_matches,
            confidence,
        }
    }

    fn sanitize(input: &str) -> String {
        let lower = input.to_lowercase();
        lower
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
}
