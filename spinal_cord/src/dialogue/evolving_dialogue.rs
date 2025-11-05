/* neira:meta
id: NEI-20260131-metacognition-integration
intent: feature
summary: |
  Интеграция MetaCognition Engine в EvolvingDialogue — 9-шаговый цикл саморефлексии.
  Добавлен шаг 9 (META-REFLECT): запись процесса мышления, анализ когнитивных искажений,
  генерация задач самоулучшения. Поддерживает обнаружение 5 типов bias: ConfirmationBias,
  AnchoringBias, AvailabilityBias, OverconfidenceBias, ContextIgnorance. 
  
  Полный цикл: recall → learn → understand → generate → assess → remember → reflect → grow → meta-reflect.
*/

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// Импорт реальных компонентов из backend
use crate::memory::{SemanticMemory, SearchResult};
#[allow(unused_imports)]
use crate::embeddings::EmbeddingsClient;
use crate::consciousness::{MetaCognitionEngine, Decision};

/// Эволюционирующая система диалога — сердце "живой" Нейры
pub struct EvolvingDialogue {
    // Реальная семантическая память (из backend)
    semantic_memory: Arc<SemanticMemory>,
    
    // Трекер роста и развития
    growth_tracker: Arc<RwLock<GrowthTracker>>,
    
    // Журнал рефлексии
    reflection_journal: Arc<RwLock<Vec<ReflectionEntry>>>,
    
    // Контекст текущего разговора
    #[allow(dead_code)]
    conversation_context: Arc<RwLock<ConversationContext>>,
    
    // Движок метапознания
    metacognition: Arc<MetaCognitionEngine>,
}

/// Контекст диалога
#[derive(Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub session_id: String,
    pub chat_id: String,
    pub message_history: Vec<Message>,
    pub user_profile: Option<UserProfile>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String, // "user" or "assistant"
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub name: Option<String>,
    pub preferences: HashMap<String, String>,
    pub interaction_count: u64,
}

/// Вход пользователя
#[derive(Clone, Serialize, Deserialize)]
pub struct UserInput {
    pub text: String,
    pub session_id: String,
    pub timestamp: DateTime<Utc>,
}

/// Ответ системы
#[derive(Clone, Serialize, Deserialize)]
pub struct DialogueResponse {
    pub text: String,
    pub confidence: f32,
    pub sources: Vec<String>, // ID похожих диалогов, использованных для ответа
    pub growth_indicator: Option<GrowthIndicator>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GrowthIndicator {
    pub skills_used: Vec<String>,
    pub new_insights: u32,
    pub similarity_to_past: f32,
}

/// Трекер роста
#[derive(Clone)]
pub struct GrowthTracker {
    pub milestones: Vec<GrowthMilestone>,
    pub skills: HashMap<String, SkillLevel>,
    pub total_conversations: u64,
    pub successful_helps: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GrowthMilestone {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub milestone_type: MilestoneType,
    pub description: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum MilestoneType {
    FirstConversation,
    LearnedNewPattern { pattern: String },
    ImprovedSkill { skill: String, from: f32, to: f32 },
    ReachedConversationCount { count: u64 },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SkillLevel {
    pub name: String,
    pub level: f32,  // 0.0 - 1.0
    pub practice_count: u64,
    pub last_practiced: DateTime<Utc>,
}

/// Запись рефлексии
#[derive(Clone, Serialize, Deserialize)]
pub struct ReflectionEntry {
    pub timestamp: DateTime<Utc>,
    pub conversation_id: String,
    pub what_worked: Vec<String>,
    pub what_failed: Vec<String>,
    pub what_to_try_next: Vec<String>,
    pub confidence_before: f32,
    pub confidence_after: f32,
}

/// Самооценка ответа
#[derive(Clone, Serialize, Deserialize)]
pub struct SelfAssessment {
    pub quality_score: f32, // 0.0 - 1.0
    pub completeness: f32,
    pub empathy_level: f32,
    pub reasoning: Vec<String>,
}

impl EvolvingDialogue {
    /// Создать новую эволюционирующую систему диалога
    pub fn new(
        semantic_memory: Arc<SemanticMemory>,
    ) -> Self {
        Self {
            semantic_memory,
            growth_tracker: Arc::new(RwLock::new(GrowthTracker {
                milestones: Vec::new(),
                skills: HashMap::new(),
                total_conversations: 0,
                successful_helps: 0,
            })),
            reflection_journal: Arc::new(RwLock::new(Vec::new())),
            conversation_context: Arc::new(RwLock::new(ConversationContext {
                session_id: String::new(),
                chat_id: String::new(),
                message_history: Vec::new(),
                user_profile: None,
            })),
            metacognition: Arc::new(MetaCognitionEngine::new()),
        }
    }

    /// Главный метод — ответ с обучением и ростом
    /// 9 шагов: recall → learn → understand → generate → assess → remember → reflect → grow → meta-reflect
    pub async fn respond_with_growth(&self, input: UserInput) -> Result<DialogueResponse, String> {
        let start_time = std::time::Instant::now();
        
        // 1. ВСПОМНИТЬ: Искать похожие диалоги в прошлом
        let similar_conversations = self.semantic_memory
            .recall_similar(&input.text, 5)
            .await
            .map_err(|e| format!("Memory recall failed: {}", e))?;
        
        tracing::info!(
            "Step 1 (Recall): Found {} similar conversations",
            similar_conversations.len()
        );

        // 2. УЧИТЬСЯ: Извлечь уроки из прошлого
        let lessons = self.extract_lessons_from_past(&similar_conversations).await;
        tracing::info!("Step 2 (Learn): Extracted {} lessons", lessons.len());

        // 3. ПОНЯТЬ: Проанализировать контекст и намерения
        let understanding = self.understand_user_intent(&input, &similar_conversations).await;
        tracing::info!("Step 3 (Understand): Intent = {:?}", understanding);

        // 4. СГЕНЕРИРОВАТЬ: Создать ответ с учётом всего опыта
        let response = self.generate_response(&input, &similar_conversations, &lessons).await?;
        tracing::info!("Step 4 (Generate): Response length = {} chars", response.text.len());

        // 5. ОЦЕНИТЬ: Насколько хорош ответ?
        let assessment = self.assess_response_quality(&input, &response).await;
        tracing::info!("Step 5 (Assess): Quality score = {:.2}", assessment.quality_score);

        // 6. ЗАПОМНИТЬ: Сохранить диалог в долгосрочную память
        let conversation_id = format!("conv_{}_{}", input.session_id, chrono::Utc::now().timestamp());
        self.remember_conversation(&conversation_id, &input, &response).await?;
        tracing::info!("Step 6 (Remember): Saved as {}", conversation_id);

        // 7. РЕФЛЕКСИЯ: Что можно улучшить?
        self.reflect_on_interaction(&conversation_id, &input, &response, &assessment).await?;
        tracing::info!("Step 7 (Reflect): Reflection saved");

        // 8. РОСТ: Отследить прогресс
        self.track_growth(&input, &response, &assessment).await?;
        tracing::info!("Step 8 (Grow): Growth tracked");

        // 9. МЕТА-РЕФЛЕКСИЯ: Анализ собственного мышления
        let reasoning_steps = vec![
            "Step 1: Recall similar conversations".to_string(),
            format!("Step 2: Learn from {} past patterns", similar_conversations.len()),
            format!("Step 3: Understand intent as {:?}", understanding),
            format!("Step 4: Generate response with {} lessons", lessons.len()),
            format!("Step 5: Assess quality: {:.2}", assessment.quality_score),
            "Step 6: Remember this conversation".to_string(),
            "Step 7: Reflect on what to improve".to_string(),
            "Step 8: Track growth metrics".to_string(),
        ];

        let decisions = vec![
            Decision {
                step: "Intent Classification".to_string(),
                rationale: format!("Classified user intent as {:?} based on keywords and similarity", understanding),
                confidence: 0.8,
                alternatives_considered: vec!["SeekingHelp".to_string(), "Gratitude".to_string(), "General".to_string()],
            },
            Decision {
                step: "Response Generation".to_string(),
                rationale: format!("Generated response using {} similar memories and {} lessons", similar_conversations.len(), lessons.len()),
                confidence: response.confidence,
                alternatives_considered: vec![],
            },
        ];

        let thought_trace_id = self.metacognition
            .record_thought_trace(conversation_id.clone(), reasoning_steps, decisions)
            .await?;

        let detected_biases = self.metacognition
            .analyze_thought_process(&thought_trace_id)
            .await?;

        if !detected_biases.is_empty() {
            tracing::warn!(
                "Step 9 (Meta-Reflect): Detected {} cognitive biases in conversation {}",
                detected_biases.len(),
                conversation_id
            );
            for bias in &detected_biases {
                tracing::debug!("  → {:?} (severity: {:?}): {}", bias.bias_type, bias.severity, bias.description);
            }
        } else {
            tracing::info!("Step 9 (Meta-Reflect): No cognitive biases detected");
        }

        let elapsed = start_time.elapsed();
        metrics::histogram!("evolving_dialogue_response_time_ms").record(elapsed.as_millis() as f64);
        metrics::counter!("evolving_dialogue_responses_total").increment(1);

        Ok(response)
    }

    /// Извлечение уроков из прошлых диалогов
    async fn extract_lessons_from_past(&self, past: &[SearchResult]) -> Vec<String> {
        let mut lessons = Vec::new();

        for result in past {
            if result.similarity > 0.7 {
                // Высокая схожесть — извлекаем паттерн
                lessons.push(format!(
                    "Похожий случай (similarity: {:.2}): '{}'",
                    result.similarity,
                    result.conversation.text.chars().take(100).collect::<String>()
                ));
            }
        }

        lessons
    }

    /// Понимание намерений пользователя
    async fn understand_user_intent(&self, input: &UserInput, similar: &[SearchResult]) -> UserIntent {
        let text_lower = input.text.to_lowercase();

        if text_lower.contains("помоги") || text_lower.contains("как") {
            UserIntent::SeekingHelp
        } else if text_lower.contains("спасибо") || text_lower.contains("отлично") {
            UserIntent::Gratitude
        } else if text_lower.contains("не понимаю") || text_lower.contains("объясни") {
            UserIntent::NeedsExplanation
        } else if !similar.is_empty() && similar[0].similarity > 0.8 {
            UserIntent::RepeatQuestion
        } else {
            UserIntent::General
        }
    }

    /// Генерация ответа
    async fn generate_response(
        &self,
        input: &UserInput,
        similar: &[SearchResult],
        lessons: &[String],
    ) -> Result<DialogueResponse, String> {
        let mut response_text = String::new();

        // Базовый ответ на основе похожих диалогов
        if !similar.is_empty() && similar[0].similarity > 0.6 {
            response_text.push_str(&format!(
                "Я помню похожий случай. Тогда я отвечала: '{}'\n\n",
                similar[0].conversation.text.chars().take(150).collect::<String>()
            ));
        }

        // Основной ответ (пока простая эхо-логика, потом можно подключить LLM)
        response_text.push_str(&format!("Вы сказали: '{}'\n", input.text));
        response_text.push_str("Я учусь отвечать лучше с каждым диалогом.\n");

        // Добавить уроки
        if !lessons.is_empty() {
            response_text.push_str("\nИз прошлого опыта:\n");
            for (i, lesson) in lessons.iter().take(2).enumerate() {
                response_text.push_str(&format!("{}. {}\n", i + 1, lesson));
            }
        }

        Ok(DialogueResponse {
            text: response_text,
            confidence: if similar.is_empty() { 0.5 } else { similar[0].similarity },
            sources: similar.iter().take(3).map(|r| r.conversation.id.clone()).collect(),
            growth_indicator: Some(GrowthIndicator {
                skills_used: vec!["memory_recall".to_string(), "pattern_matching".to_string()],
                new_insights: lessons.len() as u32,
                similarity_to_past: if similar.is_empty() { 0.0 } else { similar[0].similarity },
            }),
        })
    }

    /// Самооценка качества ответа
    async fn assess_response_quality(&self, _input: &UserInput, response: &DialogueResponse) -> SelfAssessment {
        SelfAssessment {
            quality_score: response.confidence,
            completeness: if response.text.len() > 50 { 0.8 } else { 0.5 },
            empathy_level: 0.6, // TODO: реальная оценка эмпатии
            reasoning: vec![
                format!("Confidence: {:.2}", response.confidence),
                format!("Used {} past sources", response.sources.len()),
            ],
        }
    }

    /// Сохранение диалога в память
    async fn remember_conversation(
        &self,
        conversation_id: &str,
        input: &UserInput,
        response: &DialogueResponse,
    ) -> Result<(), String> {
        let full_text = format!("User: {}\nNeira: {}", input.text, response.text);
        
        let mut metadata = HashMap::new();
        metadata.insert("session_id".to_string(), input.session_id.clone());
        metadata.insert("confidence".to_string(), response.confidence.to_string());
        metadata.insert("timestamp".to_string(), input.timestamp.to_rfc3339());

        self.semantic_memory
            .remember(conversation_id.to_string(), full_text, metadata)
            .await
            .map_err(|e| format!("Failed to remember: {}", e))?;

        metrics::counter!("evolving_dialogue_memories_stored").increment(1);

        Ok(())
    }

    /// Рефлексия о диалоге
    async fn reflect_on_interaction(
        &self,
        conversation_id: &str,
        _input: &UserInput,
        response: &DialogueResponse,
        assessment: &SelfAssessment,
    ) -> Result<(), String> {
        let mut what_worked = Vec::new();
        let mut what_to_improve = Vec::new();

        if assessment.quality_score > 0.7 {
            what_worked.push("High confidence response".to_string());
        }

        if !response.sources.is_empty() {
            what_worked.push(format!("Used {} past sources", response.sources.len()));
        } else {
            what_to_improve.push("No past sources found - need more data".to_string());
        }

        if assessment.completeness < 0.6 {
            what_to_improve.push("Response too short - elaborate more".to_string());
        }

        let entry = ReflectionEntry {
            timestamp: Utc::now(),
            conversation_id: conversation_id.to_string(),
            what_worked,
            what_failed: Vec::new(),
            what_to_try_next: what_to_improve,
            confidence_before: 0.5,
            confidence_after: assessment.quality_score,
        };

        self.reflection_journal.write().await.push(entry);
        metrics::counter!("evolving_dialogue_reflections_total").increment(1);

        Ok(())
    }

    /// Отслеживание роста
    async fn track_growth(
        &self,
        _input: &UserInput,
        response: &DialogueResponse,
        assessment: &SelfAssessment,
    ) -> Result<(), String> {
        let mut tracker = self.growth_tracker.write().await;
        tracker.total_conversations += 1;

        if assessment.quality_score > 0.7 {
            tracker.successful_helps += 1;
        }

        // Обновить навыки
        if let Some(indicator) = &response.growth_indicator {
            for skill_name in &indicator.skills_used {
                let skill = tracker.skills.entry(skill_name.clone()).or_insert(SkillLevel {
                    name: skill_name.clone(),
                    level: 0.1,
                    practice_count: 0,
                    last_practiced: Utc::now(),
                });
                
                skill.practice_count += 1;
                skill.last_practiced = Utc::now();
                
                // Увеличиваем level на 0.01 за каждую практику (но не выше 1.0)
                skill.level = (skill.level + 0.01).min(1.0);
            }
        }

        // Вехи развития
        if tracker.total_conversations == 1 {
            let milestones_len = tracker.milestones.len();
            tracker.milestones.push(GrowthMilestone {
                id: format!("milestone_{}", milestones_len),
                timestamp: Utc::now(),
                milestone_type: MilestoneType::FirstConversation,
                description: "Первый диалог с использованием памяти!".to_string(),
            });
        } else if tracker.total_conversations % 10 == 0 {
            let milestones_len = tracker.milestones.len();
            let conv_count = tracker.total_conversations;
            tracker.milestones.push(GrowthMilestone {
                id: format!("milestone_{}", milestones_len),
                timestamp: Utc::now(),
                milestone_type: MilestoneType::ReachedConversationCount {
                    count: conv_count,
                },
                description: format!("Провела {} диалогов!", conv_count),
            });
        }

        metrics::gauge!("evolving_dialogue_total_conversations").set(tracker.total_conversations as f64);
        metrics::gauge!("evolving_dialogue_successful_helps").set(tracker.successful_helps as f64);

        Ok(())
    }

    /// Получить статистику роста
    pub async fn get_growth_stats(&self) -> GrowthStats {
        let tracker = self.growth_tracker.read().await;
        
        GrowthStats {
            total_conversations: tracker.total_conversations,
            successful_helps: tracker.successful_helps,
            success_rate: if tracker.total_conversations > 0 {
                tracker.successful_helps as f32 / tracker.total_conversations as f32
            } else {
                0.0
            },
            skills_count: tracker.skills.len(),
            milestones_count: tracker.milestones.len(),
            top_skills: tracker.skills
                .values()
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
                .take(5)
                .collect(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GrowthStats {
    pub total_conversations: u64,
    pub successful_helps: u64,
    pub success_rate: f32,
    pub skills_count: usize,
    pub milestones_count: usize,
    pub top_skills: Vec<SkillLevel>,
}

#[derive(Debug, Clone)]
enum UserIntent {
    SeekingHelp,
    Gratitude,
    NeedsExplanation,
    RepeatQuestion,
    General,
}
