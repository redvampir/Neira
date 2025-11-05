/* neira:meta
id: NEI-20260131-metacognition-engine-v2
intent: feature
summary: |
  MetaCognition Engine — ядро саморефлексии для анализа собственных мыслительных процессов.
  
  Компоненты:
  - ThoughtTrace: запись reasoning_steps и decisions для каждого диалога
  - BiasDetection: обнаружение 5 типов когнитивных искажений (ConfirmationBias, AnchoringBias, 
    AvailabilityBias, OverconfidenceBias, ContextIgnorance) с оценкой severity (Low/Medium/High/Critical)
  - ImprovementTask: задачи самоулучшения (PracticeSkill, LearnPattern, FixBias, ImproveEmpathy, 
    ImproveResponseQuality) с приоритетами и статусами
  
  Методы:
  - record_thought_trace(): запись процесса мышления с reasoning_steps и decisions
  - analyze_thought_process(): эвристики для обнаружения overconfidence (все conf > 0.9) 
    и confirmation bias (нет alternatives_considered)
  - generate_improvement_tasks(): анализ ReflectionEntry и GrowthTracker для создания задач 
    (frequent failures → tasks, low skills → PracticeSkill, high severity biases → FixBias)
  - get_active_tasks(), complete_task(), get_stats()
  
  Метрики: metacognition_thoughts_recorded, biases_detected, tasks_completed, improvement_tasks_pending.
*/

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::dialogue::{ReflectionEntry, GrowthTracker};

/// Движок метапознания — анализ собственного мышления
pub struct MetaCognitionEngine {
    /// История мыслительных процессов
    thought_history: Arc<RwLock<Vec<ThoughtTrace>>>,
    
    /// Обнаруженные когнитивные искажения
    detected_biases: Arc<RwLock<Vec<BiasDetection>>>,
    
    /// Задачи для самоулучшения
    improvement_tasks: Arc<RwLock<Vec<ImprovementTask>>>,
}

/// Трейс мыслительного процесса
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ThoughtTrace {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub dialogue_id: String,
    
    /// Шаги рассуждения
    pub reasoning_steps: Vec<String>,
    
    /// Принятые решения
    pub decisions: Vec<Decision>,
    
    /// Качество процесса (самооценка)
    pub process_quality: f32, // 0.0 - 1.0
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Decision {
    pub step: String,
    pub rationale: String,
    pub confidence: f32,
    pub alternatives_considered: Vec<String>,
}

/// Обнаруженное когнитивное искажение
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BiasDetection {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub bias_type: BiasType,
    pub description: String,
    pub dialogue_id: String,
    pub severity: BiasSeverity,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BiasType {
    /// Подтверждающее искажение (искать только подтверждения)
    ConfirmationBias,
    
    /// Якорение (чрезмерная опора на первую информацию)
    AnchoringBias,
    
    /// Доступность (судить по легко вспоминаемым примерам)
    AvailabilityBias,
    
    /// Чрезмерная уверенность
    OverconfidenceBias,
    
    /// Игнорирование контекста
    ContextIgnorance,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BiasSeverity {
    Low,
    Medium,
    High,
}

/// Задача для самоулучшения
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ImprovementTask {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub task_type: TaskType,
    pub description: String,
    pub rationale: String,
    pub status: TaskStatus,
    pub priority: Priority,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TaskType {
    /// Практиковать навык
    PracticeSkill { skill_name: String },
    
    /// Изучить новый паттерн
    LearnPattern { pattern_name: String },
    
    /// Исправить искажение
    FixBias { bias_type: BiasType },
    
    /// Улучшить эмпатию
    ImproveEmpathy { context: String },
    
    /// Повысить качество ответов
    ImproveResponseQuality { area: String },
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

impl MetaCognitionEngine {
    pub fn new() -> Self {
        Self {
            thought_history: Arc::new(RwLock::new(Vec::new())),
            detected_biases: Arc::new(RwLock::new(Vec::new())),
            improvement_tasks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Записать трейс мыслительного процесса
    pub async fn record_thought_trace(
        &self,
        dialogue_id: String,
        reasoning_steps: Vec<String>,
        decisions: Vec<Decision>,
    ) -> Result<String, String> {
        let trace_id = format!("thought_{}_{}", dialogue_id, Utc::now().timestamp());
        
        let trace = ThoughtTrace {
            id: trace_id.clone(),
            timestamp: Utc::now(),
            dialogue_id,
            reasoning_steps,
            decisions,
            process_quality: 0.5, // Будет обновлено после анализа
        };

        self.thought_history.write().await.push(trace);
        metrics::counter!("metacognition_thoughts_recorded").increment(1);

        Ok(trace_id)
    }

    /// Анализ мыслительного процесса (обнаружение искажений)
    pub async fn analyze_thought_process(&self, trace_id: &str) -> Result<Vec<BiasDetection>, String> {
        let history = self.thought_history.read().await;
        let trace = history
            .iter()
            .find(|t| t.id == trace_id)
            .ok_or_else(|| "Thought trace not found".to_string())?;

        let mut biases = Vec::new();

        // Проверка на overconfidence (все решения с confidence > 0.9)
        let high_conf_count = trace.decisions.iter().filter(|d| d.confidence > 0.9).count();
        if high_conf_count == trace.decisions.len() && trace.decisions.len() > 2 {
            biases.push(BiasDetection {
                id: format!("bias_{}_{}", trace_id, biases.len()),
                timestamp: Utc::now(),
                bias_type: BiasType::OverconfidenceBias,
                description: "Все решения имеют очень высокую уверенность — возможна чрезмерная самоуверенность".to_string(),
                dialogue_id: trace.dialogue_id.clone(),
                severity: BiasSeverity::Medium,
            });
        }

        // Проверка на игнорирование альтернатив
        let no_alternatives = trace.decisions.iter().filter(|d| d.alternatives_considered.is_empty()).count();
        if no_alternatives > trace.decisions.len() / 2 {
            biases.push(BiasDetection {
                id: format!("bias_{}_{}", trace_id, biases.len()),
                timestamp: Utc::now(),
                bias_type: BiasType::ConfirmationBias,
                description: "Более половины решений не рассматривали альтернативы".to_string(),
                dialogue_id: trace.dialogue_id.clone(),
                severity: BiasSeverity::High,
            });
        }

        // Сохранить обнаруженные искажения
        if !biases.is_empty() {
            self.detected_biases.write().await.extend(biases.clone());
            metrics::counter!("metacognition_biases_detected").increment(biases.len() as u64);
        }

        Ok(biases)
    }

    /// Генерация задач для самоулучшения на основе рефлексии
    pub async fn generate_improvement_tasks(
        &self,
        reflection_entries: &[ReflectionEntry],
        growth_tracker: &GrowthTracker,
    ) -> Result<Vec<ImprovementTask>, String> {
        let mut tasks = Vec::new();

        // Анализ паттернов неудач в рефлексии
        let mut failure_patterns: HashMap<String, u32> = HashMap::new();
        for entry in reflection_entries.iter().rev().take(10) {
            for failure in &entry.what_failed {
                *failure_patterns.entry(failure.clone()).or_insert(0) += 1;
            }
        }

        // Генерация задач на основе частых неудач
        for (failure, count) in failure_patterns.iter() {
            if *count >= 3 {
                tasks.push(ImprovementTask {
                    id: format!("task_{}_{}", Utc::now().timestamp(), tasks.len()),
                    created_at: Utc::now(),
                    task_type: TaskType::ImproveResponseQuality {
                        area: failure.clone(),
                    },
                    description: format!("Улучшить: {}", failure),
                    rationale: format!("Проблема повторялась {} раз в последних диалогах", count),
                    status: TaskStatus::Pending,
                    priority: if *count >= 5 { Priority::High } else { Priority::Medium },
                });
            }
        }

        // Генерация задач на основе низких навыков
        for (skill_name, skill) in &growth_tracker.skills {
            if skill.level < 0.5 && skill.practice_count > 5 {
                tasks.push(ImprovementTask {
                    id: format!("task_{}_{}", Utc::now().timestamp(), tasks.len()),
                    created_at: Utc::now(),
                    task_type: TaskType::PracticeSkill {
                        skill_name: skill_name.clone(),
                    },
                    description: format!("Практиковать навык: {}", skill_name),
                    rationale: format!("Уровень {} после {} практик — нужно больше внимания", skill.level, skill.practice_count),
                    status: TaskStatus::Pending,
                    priority: Priority::Medium,
                });
            }
        }

        // Генерация задач на основе обнаруженных искажений
        let biases = self.detected_biases.read().await;
        let recent_high_severity_biases = biases
            .iter()
            .rev()
            .take(10)
            .filter(|b| matches!(b.severity, BiasSeverity::High))
            .collect::<Vec<_>>();

        for bias in recent_high_severity_biases {
            tasks.push(ImprovementTask {
                id: format!("task_{}_{}", Utc::now().timestamp(), tasks.len()),
                created_at: Utc::now(),
                task_type: TaskType::FixBias {
                    bias_type: bias.bias_type.clone(),
                },
                description: format!("Исправить когнитивное искажение: {:?}", bias.bias_type),
                rationale: bias.description.clone(),
                status: TaskStatus::Pending,
                priority: Priority::High,
            });
        }

        // Сохранить задачи
        self.improvement_tasks.write().await.extend(tasks.clone());
        metrics::gauge!("metacognition_improvement_tasks_pending")
            .set(tasks.iter().filter(|t| t.status == TaskStatus::Pending).count() as f64);

        Ok(tasks)
    }

    /// Получить активные задачи для самоулучшения
    pub async fn get_active_tasks(&self) -> Vec<ImprovementTask> {
        let tasks = self.improvement_tasks.read().await;
        tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Pending || t.status == TaskStatus::InProgress)
            .cloned()
            .collect()
    }

    /// Отметить задачу как выполненную
    pub async fn complete_task(&self, task_id: &str) -> Result<(), String> {
        let mut tasks = self.improvement_tasks.write().await;
        let task = tasks
            .iter_mut()
            .find(|t| t.id == task_id)
            .ok_or_else(|| "Task not found".to_string())?;

        task.status = TaskStatus::Completed;
        metrics::counter!("metacognition_tasks_completed").increment(1);

        Ok(())
    }

    /// Получить статистику метапознания
    pub async fn get_stats(&self) -> MetaCognitionStats {
        let thought_history = self.thought_history.read().await;
        let biases = self.detected_biases.read().await;
        let tasks = self.improvement_tasks.read().await;

        MetaCognitionStats {
            total_thoughts_recorded: thought_history.len(),
            total_biases_detected: biases.len(),
            total_improvement_tasks: tasks.len(),
            pending_tasks: tasks.iter().filter(|t| t.status == TaskStatus::Pending).count(),
            completed_tasks: tasks.iter().filter(|t| t.status == TaskStatus::Completed).count(),
            avg_process_quality: if thought_history.is_empty() {
                0.0
            } else {
                thought_history.iter().map(|t| t.process_quality).sum::<f32>() / thought_history.len() as f32
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MetaCognitionStats {
    pub total_thoughts_recorded: usize,
    pub total_biases_detected: usize,
    pub total_improvement_tasks: usize,
    pub pending_tasks: usize,
    pub completed_tasks: usize,
    pub avg_process_quality: f32,
}
