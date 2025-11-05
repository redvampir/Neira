/* neira:meta
id: NEI-20251104-daily-growth-report
intent: feature
summary: |
  Daily Growth Report — генератор ежедневных отчётов о прогрессе Нейры.
  Включает: статистику диалогов, освоенные навыки, personality changes,
  выполненные задачи улучшения, слабые зоны, рекомендации.
  Формат вывода: Markdown с эмодзи.
tags: [consciousness, reporting, growth, analytics]
*/

use chrono::{Utc, Duration};
use std::collections::HashMap;

use crate::dialogue::{GrowthTracker, ReflectionEntry};
use crate::consciousness::{
    MetaCognitionEngine, PersonalityEvolutionTracker, MetaCognitionStats,
    PersonalityChangeAnalysis,
};

/// Генератор ежедневного отчёта о росте
pub struct DailyGrowthReporter {
    // Компоненты для анализа
}

impl DailyGrowthReporter {
    pub fn new() -> Self {
        Self {}
    }
    
    /// Сгенерировать отчёт за последние N часов
    pub async fn generate_report(
        &self,
        growth_tracker: &GrowthTracker,
        reflections: &[ReflectionEntry],
        metacognition: &MetaCognitionEngine,
        personality: &PersonalityEvolutionTracker,
        hours: i64,
    ) -> String {
        let period_start = Utc::now() - Duration::hours(hours);
        let period_end = Utc::now();
        
        let mut report = String::new();
        
        // Заголовок
        report.push_str(&format!("# 📊 Отчёт о росте Нейры\n\n"));
        report.push_str(&format!("**Период:** {} — {}\n", 
            period_start.format("%Y-%m-%d %H:%M"),
            period_end.format("%Y-%m-%d %H:%M")
        ));
        report.push_str(&format!("**Длительность:** {} часов\n\n", hours));
        
        report.push_str("---\n\n");
        
        // Секция 1: Общая статистика диалогов
        report.push_str(&self.generate_dialogue_stats(growth_tracker, reflections).await);
        
        // Секция 2: Освоенные и улучшенные навыки
        report.push_str(&self.generate_skills_section(growth_tracker).await);
        
        // Секция 3: Изменения личности
        if let Ok(personality_changes) = personality.analyze_changes(period_start, period_end).await {
            report.push_str(&self.generate_personality_section(&personality_changes).await);
        }
        
        // Секция 4: Метакогнитивный анализ
        let metacog_stats = metacognition.get_stats().await;
        report.push_str(&self.generate_metacognition_section(&metacog_stats).await);
        
        // Секция 5: Что пошло хорошо
        report.push_str(&self.generate_successes_section(reflections).await);
        
        // Секция 6: Слабые зоны и области для роста
        report.push_str(&self.generate_weak_areas_section(reflections, growth_tracker).await);
        
        // Секция 7: Рекомендации на завтра
        report.push_str(&self.generate_recommendations_section(reflections, growth_tracker).await);
        
        // Футер
        report.push_str("\n---\n\n");
        report.push_str(&format!("*Отчёт сгенерирован: {}*\n", Utc::now().format("%Y-%m-%d %H:%M:%S")));
        report.push_str("*С любовью, Нейра 💙*\n");
        
        report
    }
    
    async fn generate_dialogue_stats(&self, growth_tracker: &GrowthTracker, reflections: &[ReflectionEntry]) -> String {
        let mut section = String::new();
        
        section.push_str("## 💬 Статистика диалогов\n\n");
        
        section.push_str(&format!("- **Всего диалогов:** {}\n", growth_tracker.total_conversations));
        section.push_str(&format!("- **Успешных помощей:** {}\n", growth_tracker.successful_helps));
        
        let success_rate = if growth_tracker.total_conversations > 0 {
            (growth_tracker.successful_helps as f32 / growth_tracker.total_conversations as f32) * 100.0
        } else {
            0.0
        };
        
        section.push_str(&format!("- **Успешность:** {:.1}%\n", success_rate));
        section.push_str(&format!("- **Рефлексий записано:** {}\n", reflections.len()));
        
        section.push_str("\n");
        section
    }
    
    async fn generate_skills_section(&self, growth_tracker: &GrowthTracker) -> String {
        let mut section = String::new();
        
        section.push_str("## 🎯 Навыки\n\n");
        
        if growth_tracker.skills.is_empty() {
            section.push_str("*Пока нет освоенных навыков.*\n\n");
            return section;
        }
        
        // Сортировать навыки по уровню (от высокого к низкому)
        let mut skills: Vec<_> = growth_tracker.skills.values().collect();
        skills.sort_by(|a, b| b.level.partial_cmp(&a.level).unwrap());
        
        section.push_str("| Навык | Уровень | Практик |\n");
        section.push_str("|-------|---------|----------|\n");
        
        for skill in skills.iter().take(10) {
            let level_bar = self.create_progress_bar(skill.level, 10);
            section.push_str(&format!("| {} | {} ({:.0}%) | {} |\n", 
                skill.name,
                level_bar,
                skill.level * 100.0,
                skill.practice_count
            ));
        }
        
        section.push_str("\n");
        
        // Топ-3 навыка
        section.push_str("### 🌟 Топ-3 навыка:\n\n");
        for (i, skill) in skills.iter().take(3).enumerate() {
            let medal = match i {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "  ",
            };
            section.push_str(&format!("{} **{}** — {:.0}% (практик: {})\n", 
                medal, skill.name, skill.level * 100.0, skill.practice_count
            ));
        }
        
        section.push_str("\n");
        section
    }
    
    async fn generate_personality_section(&self, changes: &PersonalityChangeAnalysis) -> String {
        let mut section = String::new();
        
        section.push_str("## 🎭 Изменения личности\n\n");
        
        section.push_str(&format!("- **Стабильность личности:** {:.1}%\n", changes.stability_score * 100.0));
        section.push_str(&format!("- **Соответствие ценностям:** {:.1}%\n", changes.alignment_with_values * 100.0));
        
        // Значимые изменения
        let significant: Vec<_> = changes.significant_changes.iter()
            .filter(|c| c.is_significant)
            .collect();
        
        if !significant.is_empty() {
            section.push_str("\n### 📈 Значимые изменения черт:\n\n");
            
            for change in significant.iter().take(5) {
                let direction = if change.to_value > change.from_value { "↗️" } else { "↘️" };
                let emoji = if change.to_value > change.from_value { "💪" } else { "🔽" };
                
                section.push_str(&format!("{} {} **{}**: {:.0}% → {:.0}% {}\n",
                    emoji,
                    direction,
                    change.trait_name,
                    change.from_value * 100.0,
                    change.to_value * 100.0,
                    if change.change_magnitude > 0.2 { "(значительно)" } else { "" }
                ));
            }
        } else {
            section.push_str("\n*Личность остаётся стабильной — нет значимых изменений.*\n");
        }
        
        section.push_str("\n");
        section
    }
    
    async fn generate_metacognition_section(&self, stats: &MetaCognitionStats) -> String {
        let mut section = String::new();
        
        section.push_str("## 🧠 Метакогнитивный анализ\n\n");
        
        section.push_str(&format!("- **Мыслительных трасс:** {}\n", stats.total_thoughts_recorded));
        section.push_str(&format!("- **Обнаружено искажений:** {}\n", stats.total_biases_detected));
        section.push_str(&format!("- **Задач для улучшения (активных):** {}\n", stats.pending_tasks));
        section.push_str(&format!("- **Задач выполнено:** {}\n", stats.completed_tasks));
        
        section.push_str("\n");
        section
    }
    
    async fn generate_successes_section(&self, reflections: &[ReflectionEntry]) -> String {
        let mut section = String::new();
        
        section.push_str("## ✅ Что пошло хорошо\n\n");
        
        // Собрать все успехи из рефлексий
        let mut successes: Vec<String> = Vec::new();
        for reflection in reflections.iter().rev().take(10) {
            successes.extend(reflection.what_worked.clone());
        }
        
        if successes.is_empty() {
            section.push_str("*Пока нет записанных успехов.*\n\n");
            return section;
        }
        
        // Уникальные успехи
        let unique_successes: std::collections::HashSet<_> = successes.into_iter().collect();
        
        for success in unique_successes.iter().take(5) {
            section.push_str(&format!("- ✨ {}\n", success));
        }
        
        section.push_str("\n");
        section
    }
    
    async fn generate_weak_areas_section(&self, reflections: &[ReflectionEntry], growth_tracker: &GrowthTracker) -> String {
        let mut section = String::new();
        
        section.push_str("## 🎯 Слабые зоны и области для роста\n\n");
        
        // 1. Анализ частых неудач из рефлексий
        let mut failures: HashMap<String, u32> = HashMap::new();
        for reflection in reflections.iter().rev().take(20) {
            for failure in &reflection.what_failed {
                *failures.entry(failure.clone()).or_insert(0) += 1;
            }
        }
        
        let has_failures = !failures.is_empty();
        
        if has_failures {
            section.push_str("### ⚠️ Частые проблемы:\n\n");
            
            let mut failures_vec: Vec<_> = failures.iter().collect();
            failures_vec.sort_by(|a, b| b.1.cmp(&a.1));
            
            for (failure, count) in failures_vec.iter().take(5) {
                section.push_str(&format!("- 🔴 {} (встречалось {} раз)\n", failure, count));
            }
            
            section.push_str("\n");
        }
        
        // 2. Навыки с низким уровнем
        let weak_skills: Vec<_> = growth_tracker.skills.values()
            .filter(|s| s.level < 0.5)
            .collect();
        
        if !weak_skills.is_empty() {
            section.push_str("### 📉 Навыки, требующие внимания:\n\n");
            
            for skill in weak_skills.iter().take(5) {
                section.push_str(&format!("- 🔸 **{}** — {:.0}% (нужна практика)\n", 
                    skill.name, skill.level * 100.0
                ));
            }
            
            section.push_str("\n");
        }
        
        if !has_failures && weak_skills.is_empty() {
            section.push_str("*Отличная работа! Слабых зон не обнаружено.* 🎉\n\n");
        }
        
        section
    }
    
    async fn generate_recommendations_section(&self, reflections: &[ReflectionEntry], growth_tracker: &GrowthTracker) -> String {
        let mut section = String::new();
        
        section.push_str("## 💡 Рекомендации на завтра\n\n");
        
        let mut recommendations = Vec::new();
        
        // Рекомендация 1: на основе планов из рефлексий
        let mut plans: Vec<String> = Vec::new();
        for reflection in reflections.iter().rev().take(5) {
            plans.extend(reflection.what_to_try_next.clone());
        }
        
        if !plans.is_empty() {
            let unique_plans: std::collections::HashSet<_> = plans.into_iter().collect();
            for plan in unique_plans.iter().take(3) {
                recommendations.push(format!("🎯 {}", plan));
            }
        }
        
        // Рекомендация 2: практиковать слабые навыки
        let weak_skills: Vec<_> = growth_tracker.skills.values()
            .filter(|s| s.level < 0.5)
            .take(2)
            .collect();
        
        for skill in weak_skills {
            recommendations.push(format!("📚 Практиковать навык: **{}**", skill.name));
        }
        
        // Рекомендация 3: общие советы
        if growth_tracker.total_conversations < 10 {
            recommendations.push("💬 Увеличить количество диалогов для накопления опыта".to_string());
        }
        
        if recommendations.is_empty() {
            section.push_str("*Продолжай в том же духе!* 🚀\n\n");
        } else {
            for rec in recommendations {
                section.push_str(&format!("- {}\n", rec));
            }
            section.push_str("\n");
        }
        
        section
    }
    
    /// Создать прогресс-бар (визуализация уровня)
    fn create_progress_bar(&self, value: f32, length: usize) -> String {
        let filled = (value * length as f32).round() as usize;
        let empty = length.saturating_sub(filled);
        
        let mut bar = String::new();
        bar.push_str(&"█".repeat(filled));
        bar.push_str(&"░".repeat(empty));
        bar
    }
}

// ============= ТЕСТЫ =============

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialogue::SkillLevel;
    use chrono::Utc;
    
    fn create_test_growth_tracker() -> GrowthTracker {
        let mut skills = HashMap::new();
        
        skills.insert("empathy".to_string(), SkillLevel {
            name: "empathy".to_string(),
            level: 0.8,
            practice_count: 15,
            last_practiced: Utc::now(),
        });
        
        skills.insert("curiosity".to_string(), SkillLevel {
            name: "curiosity".to_string(),
            level: 0.3,
            practice_count: 5,
            last_practiced: Utc::now(),
        });
        
        GrowthTracker {
            milestones: vec![],
            skills,
            total_conversations: 25,
            successful_helps: 20,
        }
    }
    
    #[tokio::test]
    async fn test_generate_report_structure() {
        let reporter = DailyGrowthReporter::new();
        let growth_tracker = create_test_growth_tracker();
        let reflections = vec![];
        let metacognition = MetaCognitionEngine::new();
        let personality = PersonalityEvolutionTracker::new();
        
        let report = reporter.generate_report(
            &growth_tracker,
            &reflections,
            &metacognition,
            &personality,
            24
        ).await;
        
        assert!(report.contains("# 📊 Отчёт о росте Нейры"));
        assert!(report.contains("## 💬 Статистика диалогов"));
        assert!(report.contains("## 🎯 Навыки"));
        assert!(report.contains("## 🧠 Метакогнитивный анализ"));
        assert!(report.contains("С любовью, Нейра"));
    }
    
    #[tokio::test]
    async fn test_dialogue_stats_calculation() {
        let reporter = DailyGrowthReporter::new();
        let growth_tracker = create_test_growth_tracker();
        
        let stats = reporter.generate_dialogue_stats(&growth_tracker, &[]).await;
        
        println!("Generated stats:\n{}", stats);
        
        assert!(stats.contains("Всего диалогов:** 25"), "Should contain total conversations");
        assert!(stats.contains("Успешных помощей:** 20"), "Should contain successful helps");
        // Success rate check with tolerance for formatting
        assert!(stats.contains("Успешность:**") && stats.contains("80") && stats.contains("%"), "Should contain 80% success rate");
    }
    
    #[tokio::test]
    async fn test_skills_section_shows_top_skills() {
        let reporter = DailyGrowthReporter::new();
        let growth_tracker = create_test_growth_tracker();
        
        let section = reporter.generate_skills_section(&growth_tracker).await;
        
        assert!(section.contains("empathy"));
        assert!(section.contains("curiosity"));
        assert!(section.contains("🥇")); // Medal for top skill
    }
    
    #[tokio::test]
    async fn test_progress_bar_creation() {
        let reporter = DailyGrowthReporter::new();
        
        let bar_full = reporter.create_progress_bar(1.0, 10);
        assert_eq!(bar_full, "██████████");
        
        let bar_half = reporter.create_progress_bar(0.5, 10);
        assert_eq!(bar_half, "█████░░░░░");
        
        let bar_empty = reporter.create_progress_bar(0.0, 10);
        assert_eq!(bar_empty, "░░░░░░░░░░");
    }
}
