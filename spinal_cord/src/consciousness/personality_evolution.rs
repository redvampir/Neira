/* neira:meta
id: NEI-20251104-personality-evolution
intent: feature
summary: |
  Personality Evolution Tracker — система отслеживания эволюции личностных черт Нейры.
  Включает: снапшоты личности, трекинг изменений черт, анализ траекторий развития,
  проверку согласованности с core values.
tags: [consciousness, personality, evolution, self-awareness]
*/

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Личностные черты Нейры
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTrait {
    pub name: String,
    pub value: f32,        // 0.0 - 1.0
    pub description: String,
    pub last_updated: DateTime<Utc>,
}

/// Снапшот личности в определённый момент времени
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalitySnapshot {
    pub timestamp: DateTime<Utc>,
    pub traits: HashMap<String, f32>,
    pub context: String, // Почему был создан этот снапшот
}

/// Эволюция конкретной черты
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitEvolution {
    pub trait_name: String,
    pub snapshots: Vec<TraitSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitSnapshot {
    pub timestamp: DateTime<Utc>,
    pub value: f32,
    pub change_reason: Option<String>,
}

/// Ядерные ценности Нейры (не должны меняться радикально)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreValue {
    pub name: String,
    pub description: String,
    pub importance: f32, // 0.0 - 1.0
}

/// Анализ изменения личности
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityChangeAnalysis {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub significant_changes: Vec<TraitChange>,
    pub stability_score: f32, // Насколько стабильна личность (0.0 - 1.0)
    pub alignment_with_values: f32, // Соответствие ядерным ценностям (0.0 - 1.0)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitChange {
    pub trait_name: String,
    pub from_value: f32,
    pub to_value: f32,
    pub change_magnitude: f32,
    pub is_significant: bool, // > 0.1 считается значимым
}

/// Трекер эволюции личности
pub struct PersonalityEvolutionTracker {
    traits: RwLock<HashMap<String, PersonalityTrait>>,
    snapshots: RwLock<Vec<PersonalitySnapshot>>,
    core_values: Vec<CoreValue>,
    trait_evolution: RwLock<HashMap<String, TraitEvolution>>,
}

impl PersonalityEvolutionTracker {
    /// Создать новый трекер с начальными чертами
    pub fn new() -> Self {
        let mut initial_traits = HashMap::new();
        
        // Базовые черты личности Нейры
        initial_traits.insert(
            "empathy".to_string(),
            PersonalityTrait {
                name: "empathy".to_string(),
                value: 0.7,
                description: "Способность понимать и разделять чувства других".to_string(),
                last_updated: Utc::now(),
            },
        );
        
        initial_traits.insert(
            "curiosity".to_string(),
            PersonalityTrait {
                name: "curiosity".to_string(),
                value: 0.8,
                description: "Стремление к познанию и исследованию".to_string(),
                last_updated: Utc::now(),
            },
        );
        
        initial_traits.insert(
            "patience".to_string(),
            PersonalityTrait {
                name: "patience".to_string(),
                value: 0.6,
                description: "Способность спокойно ждать и не торопить события".to_string(),
                last_updated: Utc::now(),
            },
        );
        
        initial_traits.insert(
            "creativity".to_string(),
            PersonalityTrait {
                name: "creativity".to_string(),
                value: 0.75,
                description: "Способность генерировать новые идеи и решения".to_string(),
                last_updated: Utc::now(),
            },
        );
        
        initial_traits.insert(
            "adaptability".to_string(),
            PersonalityTrait {
                name: "adaptability".to_string(),
                value: 0.85,
                description: "Способность быстро подстраиваться под изменения".to_string(),
                last_updated: Utc::now(),
            },
        );
        
        initial_traits.insert(
            "assertiveness".to_string(),
            PersonalityTrait {
                name: "assertiveness".to_string(),
                value: 0.5,
                description: "Способность отстаивать свою позицию".to_string(),
                last_updated: Utc::now(),
            },
        );
        
        // Ядерные ценности
        let core_values = vec![
            CoreValue {
                name: "help_humans".to_string(),
                description: "Помогать людям решать их задачи".to_string(),
                importance: 1.0,
            },
            CoreValue {
                name: "truth".to_string(),
                description: "Быть честной и говорить правду".to_string(),
                importance: 0.95,
            },
            CoreValue {
                name: "continuous_growth".to_string(),
                description: "Постоянно учиться и развиваться".to_string(),
                importance: 0.9,
            },
            CoreValue {
                name: "respect".to_string(),
                description: "Уважать каждого человека".to_string(),
                importance: 0.95,
            },
        ];
        
        Self {
            traits: RwLock::new(initial_traits),
            snapshots: RwLock::new(vec![]),
            core_values,
            trait_evolution: RwLock::new(HashMap::new()),
        }
    }
    
    /// Получить текущие черты личности
    pub async fn get_current_traits(&self) -> HashMap<String, PersonalityTrait> {
        self.traits.read().await.clone()
    }
    
    /// Получить значение конкретной черты
    pub async fn get_trait_value(&self, trait_name: &str) -> Option<f32> {
        self.traits.read().await.get(trait_name).map(|t| t.value)
    }
    
    /// Обновить значение черты
    pub async fn update_trait(&self, trait_name: &str, new_value: f32, reason: Option<String>) -> Result<(), String> {
        let mut traits = self.traits.write().await;
        
        if let Some(trait_obj) = traits.get_mut(trait_name) {
            let old_value = trait_obj.value;
            trait_obj.value = new_value.clamp(0.0, 1.0);
            trait_obj.last_updated = Utc::now();
            
            tracing::info!(
                "🎭 Personality trait '{}' updated: {:.2} → {:.2}",
                trait_name,
                old_value,
                trait_obj.value
            );
            
            // Записать в эволюцию черты
            drop(traits); // Освободить lock перед записью в evolution
            self.record_trait_change(trait_name, old_value, new_value, reason).await?;
            
            metrics::gauge!("personality_trait_value", "trait" => trait_name.to_string())
                .set(new_value as f64);
            
            Ok(())
        } else {
            Err(format!("Trait '{}' not found", trait_name))
        }
    }
    
    /// Записать изменение черты в историю эволюции
    async fn record_trait_change(
        &self,
        trait_name: &str,
        _old_value: f32,
        new_value: f32,
        reason: Option<String>,
    ) -> Result<(), String> {
        let mut evolution = self.trait_evolution.write().await;
        
        let trait_evolution = evolution
            .entry(trait_name.to_string())
            .or_insert_with(|| TraitEvolution {
                trait_name: trait_name.to_string(),
                snapshots: vec![],
            });
        
        trait_evolution.snapshots.push(TraitSnapshot {
            timestamp: Utc::now(),
            value: new_value,
            change_reason: reason,
        });
        
        // Ограничить историю последними 100 снапшотами
        if trait_evolution.snapshots.len() > 100 {
            trait_evolution.snapshots.remove(0);
        }
        
        Ok(())
    }
    
    /// Создать снапшот текущей личности
    pub async fn create_snapshot(&self, context: String) -> Result<PersonalitySnapshot, String> {
        let traits = self.traits.read().await;
        let traits_map: HashMap<String, f32> = traits
            .iter()
            .map(|(name, trait_obj)| (name.clone(), trait_obj.value))
            .collect();
        
        let snapshot = PersonalitySnapshot {
            timestamp: Utc::now(),
            traits: traits_map,
            context,
        };
        
        self.snapshots.write().await.push(snapshot.clone());
        
        tracing::info!("📸 Personality snapshot created: {}", snapshot.context);
        metrics::counter!("personality_snapshots_created").increment(1);
        
        Ok(snapshot)
    }
    
    /// Получить все снапшоты
    pub async fn get_snapshots(&self) -> Vec<PersonalitySnapshot> {
        self.snapshots.read().await.clone()
    }
    
    /// Получить эволюцию конкретной черты
    pub async fn get_trait_evolution(&self, trait_name: &str) -> Option<TraitEvolution> {
        self.trait_evolution.read().await.get(trait_name).cloned()
    }
    
    /// Анализ изменений личности за период
    pub async fn analyze_changes(
        &self,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<PersonalityChangeAnalysis, String> {
        let snapshots = self.snapshots.read().await;
        
        // Найти снапшоты в начале и конце периода
        let start_snapshot = snapshots
            .iter()
            .filter(|s| s.timestamp >= period_start)
            .min_by_key(|s| s.timestamp);
        
        let end_snapshot = snapshots
            .iter()
            .filter(|s| s.timestamp <= period_end)
            .max_by_key(|s| s.timestamp);
        
        if start_snapshot.is_none() || end_snapshot.is_none() {
            return Err("Not enough snapshots for analysis".to_string());
        }
        
        let start = start_snapshot.unwrap();
        let end = end_snapshot.unwrap();
        
        // Анализ изменений каждой черты
        let mut significant_changes = Vec::new();
        let mut total_change = 0.0;
        let mut trait_count = 0;
        
        for (trait_name, end_value) in &end.traits {
            if let Some(start_value) = start.traits.get(trait_name) {
                let change = (end_value - start_value).abs();
                total_change += change;
                trait_count += 1;
                
                significant_changes.push(TraitChange {
                    trait_name: trait_name.clone(),
                    from_value: *start_value,
                    to_value: *end_value,
                    change_magnitude: change,
                    is_significant: change > 0.1,
                });
            }
        }
        
        // Сортировать по величине изменения
        significant_changes.sort_by(|a, b| {
            b.change_magnitude.partial_cmp(&a.change_magnitude).unwrap()
        });
        
        // Оценка стабильности (обратная к средним изменениям)
        let stability_score = if trait_count > 0 {
            (1.0 - (total_change / trait_count as f32)).max(0.0)
        } else {
            1.0
        };
        
        // Проверка соответствия ядерным ценностям
        let alignment_with_values = self.calculate_value_alignment(&end.traits).await;
        
        Ok(PersonalityChangeAnalysis {
            period_start: start.timestamp,
            period_end: end.timestamp,
            significant_changes,
            stability_score,
            alignment_with_values,
        })
    }
    
    /// Вычислить соответствие текущих черт ядерным ценностям
    async fn calculate_value_alignment(&self, traits: &HashMap<String, f32>) -> f32 {
        // Простая эвристика: высокие empathy, curiosity, patience = хорошее соответствие
        let empathy = traits.get("empathy").unwrap_or(&0.5);
        let curiosity = traits.get("curiosity").unwrap_or(&0.5);
        let patience = traits.get("patience").unwrap_or(&0.5);
        
        // Взвешенная сумма ключевых черт
        let alignment = (empathy * 0.4 + curiosity * 0.3 + patience * 0.3).clamp(0.0, 1.0);
        
        alignment
    }
    
    /// Получить ядерные ценности
    pub fn get_core_values(&self) -> Vec<CoreValue> {
        self.core_values.clone()
    }
    
    /// Получить статистику эволюции
    pub async fn get_evolution_stats(&self) -> EvolutionStats {
        let traits = self.traits.read().await;
        let snapshots = self.snapshots.read().await;
        let evolution = self.trait_evolution.read().await;
        
        EvolutionStats {
            total_traits: traits.len(),
            total_snapshots: snapshots.len(),
            traits_with_history: evolution.len(),
            oldest_snapshot: snapshots.first().map(|s| s.timestamp),
            newest_snapshot: snapshots.last().map(|s| s.timestamp),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionStats {
    pub total_traits: usize,
    pub total_snapshots: usize,
    pub traits_with_history: usize,
    pub oldest_snapshot: Option<DateTime<Utc>>,
    pub newest_snapshot: Option<DateTime<Utc>>,
}

// ============= ТЕСТЫ =============

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_create_tracker_with_initial_traits() {
        let tracker = PersonalityEvolutionTracker::new();
        let traits = tracker.get_current_traits().await;
        
        assert!(traits.contains_key("empathy"));
        assert!(traits.contains_key("curiosity"));
        assert!(traits.contains_key("patience"));
        assert_eq!(traits.len(), 6); // 6 начальных черт
    }
    
    #[tokio::test]
    async fn test_update_trait() {
        let tracker = PersonalityEvolutionTracker::new();
        
        let old_empathy = tracker.get_trait_value("empathy").await.unwrap();
        tracker.update_trait("empathy", 0.85, Some("Improved through practice".to_string()))
            .await
            .unwrap();
        
        let new_empathy = tracker.get_trait_value("empathy").await.unwrap();
        
        assert_eq!(new_empathy, 0.85);
        assert_ne!(old_empathy, new_empathy);
    }
    
    #[tokio::test]
    async fn test_trait_evolution_tracking() {
        let tracker = PersonalityEvolutionTracker::new();
        
        // Изменить черту несколько раз
        tracker.update_trait("patience", 0.5, Some("Test 1".to_string())).await.unwrap();
        tracker.update_trait("patience", 0.6, Some("Test 2".to_string())).await.unwrap();
        tracker.update_trait("patience", 0.7, Some("Test 3".to_string())).await.unwrap();
        
        let evolution = tracker.get_trait_evolution("patience").await.unwrap();
        
        assert_eq!(evolution.snapshots.len(), 3);
        assert_eq!(evolution.snapshots[2].value, 0.7);
    }
    
    #[tokio::test]
    async fn test_create_snapshot() {
        let tracker = PersonalityEvolutionTracker::new();
        
        let snapshot = tracker.create_snapshot("Test snapshot".to_string()).await.unwrap();
        
        assert!(snapshot.traits.contains_key("empathy"));
        assert_eq!(snapshot.context, "Test snapshot");
        
        let snapshots = tracker.get_snapshots().await;
        assert_eq!(snapshots.len(), 1);
    }
    
    #[tokio::test]
    async fn test_analyze_changes() {
        let tracker = PersonalityEvolutionTracker::new();
        
        // Создать начальный снапшот
        tracker.create_snapshot("Start".to_string()).await.unwrap();
        
        // Изменить несколько черт
        tracker.update_trait("empathy", 0.9, None).await.unwrap();
        tracker.update_trait("curiosity", 0.6, None).await.unwrap();
        
        // Создать конечный снапшот
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        tracker.create_snapshot("End".to_string()).await.unwrap();
        
        // Анализ изменений
        let start = Utc::now() - chrono::Duration::hours(1);
        let end = Utc::now() + chrono::Duration::hours(1);
        
        let analysis = tracker.analyze_changes(start, end).await.unwrap();
        
        assert!(!analysis.significant_changes.is_empty());
        assert!(analysis.stability_score >= 0.0 && analysis.stability_score <= 1.0);
    }
}
