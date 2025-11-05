/* neira:meta
id: NEI-20260131-auto-improvement-loop
intent: feature
summary: |
  Auto-Improvement Loop — фоновая система автономного самоулучшения.
  
  Периодически (каждые N минут):
  1. Анализирует ReflectionJournal и GrowthTracker
  2. Генерирует improvement tasks через MetaCognitionEngine
  3. Выбирает задачи для выполнения (по приоритету)
  4. "Выполняет" задачи (симуляция практики/обучения)
  5. Обновляет skills в GrowthTracker
  6. Отмечает задачи как completed
  
  Метрики: auto_improvement_cycles, tasks_executed, skills_improved.
*/

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;
use chrono::Utc;

use crate::consciousness::{MetaCognitionEngine, TaskType};
use crate::dialogue::{GrowthTracker, ReflectionEntry, SkillLevel};

/// Auto-Improvement Loop — фоновая система самоулучшения
pub struct AutoImprovementLoop {
    metacognition: Arc<MetaCognitionEngine>,
    growth_tracker: Arc<RwLock<GrowthTracker>>,
    reflection_journal: Arc<RwLock<Vec<ReflectionEntry>>>,
    
    /// Интервал между циклами улучшения
    cycle_interval: Duration,
    
    /// Максимум задач для выполнения за цикл
    max_tasks_per_cycle: usize,
}

impl AutoImprovementLoop {
    pub fn new(
        metacognition: Arc<MetaCognitionEngine>,
        growth_tracker: Arc<RwLock<GrowthTracker>>,
        reflection_journal: Arc<RwLock<Vec<ReflectionEntry>>>,
    ) -> Self {
        Self {
            metacognition,
            growth_tracker,
            reflection_journal,
            cycle_interval: Duration::from_secs(300), // 5 минут
            max_tasks_per_cycle: 3,
        }
    }
    
    /// Установить интервал между циклами (для тестирования)
    pub fn with_cycle_interval(mut self, interval: Duration) -> Self {
        self.cycle_interval = interval;
        self
    }
    
    /// Установить максимум задач за цикл
    pub fn with_max_tasks(mut self, max: usize) -> Self {
        self.max_tasks_per_cycle = max;
        self
    }
    
    /// Запустить фоновый цикл самоулучшения
    pub async fn start(self: Arc<Self>) {
        tracing::info!("🔄 Auto-Improvement Loop started (interval: {:?})", self.cycle_interval);
        
        let mut interval = time::interval(self.cycle_interval);
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.run_improvement_cycle().await {
                tracing::error!("❌ Auto-Improvement cycle failed: {}", e);
                metrics::counter!("auto_improvement_errors").increment(1);
            } else {
                metrics::counter!("auto_improvement_cycles").increment(1);
            }
        }
    }
    
    /// Выполнить один цикл улучшения
    pub async fn run_improvement_cycle(&self) -> Result<(), String> {
        tracing::debug!("🔄 Starting improvement cycle");
        
        // 1. Собрать данные для анализа
        let reflections = self.reflection_journal.read().await.clone();
        let growth_tracker_guard = self.growth_tracker.read().await;
        let growth_tracker = (*growth_tracker_guard).clone();
        
        if reflections.is_empty() {
            tracing::debug!("⏭️ No reflections yet, skipping cycle");
            return Ok(());
        }
        
        // 2. Генерировать задачи (если нужно)
        let active_tasks = self.metacognition.get_active_tasks().await;
        
        if active_tasks.len() < 5 {
            tracing::debug!("📝 Generating new improvement tasks");
            let new_tasks = self.metacognition
                .generate_improvement_tasks(&reflections, &growth_tracker)
                .await?;
            
            tracing::info!("✨ Generated {} new improvement tasks", new_tasks.len());
        }
        
        // 3. Выбрать задачи для выполнения (по приоритету)
        let tasks_to_execute = self.select_tasks_to_execute().await?;
        
        if tasks_to_execute.is_empty() {
            tracing::debug!("⏭️ No tasks to execute in this cycle");
            return Ok(());
        }
        
        tracing::info!("🎯 Executing {} tasks", tasks_to_execute.len());
        
        // 4. Выполнить задачи
        for task_id in tasks_to_execute {
            if let Err(e) = self.execute_task(&task_id).await {
                tracing::error!("❌ Failed to execute task {}: {}", task_id, e);
            } else {
                tracing::info!("✅ Task {} completed", task_id);
                metrics::counter!("auto_improvement_tasks_executed").increment(1);
            }
        }
        
        Ok(())
    }
    
    /// Выбрать задачи для выполнения (по приоритету)
    async fn select_tasks_to_execute(&self) -> Result<Vec<String>, String> {
        let active_tasks = self.metacognition.get_active_tasks().await;
        
        // Сортировать по приоритету (High → Medium → Low)
        let mut sorted_tasks = active_tasks.clone();
        sorted_tasks.sort_by(|a, b| {
            use crate::consciousness::Priority;
            
            let priority_order = |p: &Priority| match p {
                Priority::Critical => 0,
                Priority::High => 1,
                Priority::Medium => 2,
                Priority::Low => 3,
            };
            
            priority_order(&a.priority).cmp(&priority_order(&b.priority))
        });
        
        // Взять первые N задач
        let selected: Vec<String> = sorted_tasks
            .iter()
            .take(self.max_tasks_per_cycle)
            .map(|t| t.id.clone())
            .collect();
        
        Ok(selected)
    }
    
    /// Выполнить задачу (симуляция практики/обучения)
    async fn execute_task(&self, task_id: &str) -> Result<(), String> {
        let active_tasks = self.metacognition.get_active_tasks().await;
        let task = active_tasks
            .iter()
            .find(|t| t.id == task_id)
            .ok_or_else(|| format!("Task {} not found", task_id))?;
        
        tracing::debug!("🎯 Executing task: {} (type: {:?})", task.description, task.task_type);
        
        match &task.task_type {
            TaskType::PracticeSkill { skill_name } => {
                self.practice_skill(skill_name).await?;
            }
            TaskType::LearnPattern { pattern_name } => {
                self.learn_pattern(pattern_name).await?;
            }
            TaskType::FixBias { bias_type } => {
                self.fix_bias(bias_type).await?;
            }
            TaskType::ImproveEmpathy { context } => {
                self.improve_empathy(context).await?;
            }
            TaskType::ImproveResponseQuality { area } => {
                self.improve_response_quality(area).await?;
            }
        }
        
        // Отметить задачу как выполненную
        self.metacognition.complete_task(task_id).await?;
        
        Ok(())
    }
    
    /// Практиковать навык (увеличить level)
    pub async fn practice_skill(&self, skill_name: &str) -> Result<(), String> {
        let mut tracker = self.growth_tracker.write().await;
        
        if let Some(skill) = tracker.skills.get_mut(skill_name) {
            // Увеличить уровень навыка на 0.05 (5%)
            skill.level = (skill.level + 0.05).min(1.0);
            skill.practice_count += 1;
            skill.last_practiced = Utc::now();
            
            tracing::info!("📈 Skill '{}' improved to {:.2}", skill_name, skill.level);
            metrics::counter!("auto_improvement_skills_practiced").increment(1);
        } else {
            // Создать новый навык
            tracker.skills.insert(
                skill_name.to_string(),
                SkillLevel {
                    name: skill_name.to_string(),
                    level: 0.1, // Начальный уровень
                    practice_count: 1,
                    last_practiced: Utc::now(),
                },
            );
            
            tracing::info!("✨ New skill learned: '{}'", skill_name);
        }
        
        Ok(())
    }
    
    /// Изучить паттерн
    async fn learn_pattern(&self, pattern_name: &str) -> Result<(), String> {
        tracing::info!("🧠 Learning pattern: {}", pattern_name);
        
        // Симуляция обучения паттерну
        // В реальной реализации здесь будет обновление knowledge base
        
        metrics::counter!("auto_improvement_patterns_learned").increment(1);
        Ok(())
    }
    
    /// Исправить когнитивное искажение
    async fn fix_bias(&self, bias_type: &crate::consciousness::BiasType) -> Result<(), String> {
        tracing::info!("🔧 Fixing bias: {:?}", bias_type);
        
        // Симуляция работы над исправлением искажения
        // В реальной реализации здесь будет корректировка decision-making логики
        
        metrics::counter!("auto_improvement_biases_fixed").increment(1);
        Ok(())
    }
    
    /// Улучшить эмпатию
    async fn improve_empathy(&self, context: &str) -> Result<(), String> {
        tracing::info!("💖 Improving empathy in context: {}", context);
        
        // Практиковать навык эмпатии
        self.practice_skill("empathy").await?;
        
        Ok(())
    }
    
    /// Улучшить качество ответов
    async fn improve_response_quality(&self, area: &str) -> Result<(), String> {
        tracing::info!("📝 Improving response quality in area: {}", area);
        
        // Симуляция улучшения качества
        // В реальной реализации здесь будет fine-tuning промптов или параметров
        
        metrics::counter!("auto_improvement_quality_improved").increment(1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[tokio::test]
    async fn test_auto_improvement_cycle_basic() {
        let metacognition = Arc::new(MetaCognitionEngine::new());
        let growth_tracker = Arc::new(RwLock::new(GrowthTracker {
            milestones: vec![],
            skills: HashMap::new(),
            total_conversations: 0,
            successful_helps: 0,
        }));
        
        let reflections = vec![
            ReflectionEntry {
                conversation_id: "conv1".to_string(),
                timestamp: Utc::now(),
                what_worked: vec![],
                what_failed: vec!["Low empathy".to_string()],
                what_to_try_next: vec!["Practice empathy".to_string()],
                confidence_before: 0.5,
                confidence_after: 0.3,
            },
        ];
        
        let reflection_journal = Arc::new(RwLock::new(reflections));
        
        let loop_instance = AutoImprovementLoop::new(
            metacognition.clone(),
            growth_tracker.clone(),
            reflection_journal,
        )
        .with_max_tasks(1);
        
        // Запустить один цикл
        loop_instance.run_improvement_cycle().await.unwrap();
        
        // Проверить, что задачи не были созданы (нужно больше данных)
        let active_tasks = metacognition.get_active_tasks().await;
        assert_eq!(active_tasks.len(), 0, "Should not create tasks with insufficient data");
    }
    
    #[tokio::test]
    async fn test_practice_skill_improvement() {
        let metacognition = Arc::new(MetaCognitionEngine::new());
        
        let mut skills = HashMap::new();
        skills.insert("empathy".to_string(), SkillLevel {
            name: "empathy".to_string(),
            level: 0.5,
            practice_count: 5,
            last_practiced: Utc::now(),
        });
        
        let growth_tracker = Arc::new(RwLock::new(GrowthTracker {
            milestones: vec![],
            skills,
            total_conversations: 10,
            successful_helps: 5,
        }));
        
        let reflection_journal = Arc::new(RwLock::new(vec![]));
        
        let loop_instance = AutoImprovementLoop::new(
            metacognition,
            growth_tracker.clone(),
            reflection_journal,
        );
        
        // Практиковать навык
        loop_instance.practice_skill("empathy").await.unwrap();
        
        // Проверить улучшение
        let tracker = growth_tracker.read().await;
        let empathy_skill = tracker.skills.get("empathy").unwrap();
        
        assert_eq!(empathy_skill.level, 0.55, "Skill level should increase by 0.05");
        assert_eq!(empathy_skill.practice_count, 6, "Practice count should increment");
    }
    
    #[tokio::test]
    async fn test_create_new_skill_when_practicing() {
        let metacognition = Arc::new(MetaCognitionEngine::new());
        let growth_tracker = Arc::new(RwLock::new(GrowthTracker {
            milestones: vec![],
            skills: HashMap::new(),
            total_conversations: 0,
            successful_helps: 0,
        }));
        
        let reflection_journal = Arc::new(RwLock::new(vec![]));
        
        let loop_instance = AutoImprovementLoop::new(
            metacognition,
            growth_tracker.clone(),
            reflection_journal,
        );
        
        // Практиковать новый навык
        loop_instance.practice_skill("new_skill").await.unwrap();
        
        // Проверить создание нового навыка
        let tracker = growth_tracker.read().await;
        assert!(tracker.skills.contains_key("new_skill"), "New skill should be created");
        
        let new_skill = tracker.skills.get("new_skill").unwrap();
        assert_eq!(new_skill.level, 0.1, "New skill should start at 0.1");
        assert_eq!(new_skill.practice_count, 1);
    }
}
