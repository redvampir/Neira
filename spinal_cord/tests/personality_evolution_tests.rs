/* neira:meta
id: NEI-20251104-personality-evolution-tests
intent: test
summary: |
  Тесты для Personality Evolution Tracker.
  Проверяет: создание трекера, обновление черт, историю эволюции,
  снапшоты, анализ изменений, соответствие ценностям.
*/

use backend::consciousness::{
    PersonalityEvolutionTracker,
};
use chrono::Utc;

#[tokio::test]
async fn test_initial_traits_created() {
    let tracker = PersonalityEvolutionTracker::new();
    let traits = tracker.get_current_traits().await;
    
    assert!(traits.contains_key("empathy"), "Should have empathy trait");
    assert!(traits.contains_key("curiosity"), "Should have curiosity trait");
    assert!(traits.contains_key("patience"), "Should have patience trait");
    assert!(traits.contains_key("creativity"), "Should have creativity trait");
    assert!(traits.contains_key("adaptability"), "Should have adaptability trait");
    assert!(traits.contains_key("assertiveness"), "Should have assertiveness trait");
    
    assert_eq!(traits.len(), 6, "Should have 6 initial traits");
}

#[tokio::test]
async fn test_get_specific_trait_value() {
    let tracker = PersonalityEvolutionTracker::new();
    
    let empathy = tracker.get_trait_value("empathy").await;
    assert!(empathy.is_some(), "Empathy should exist");
    assert_eq!(empathy.unwrap(), 0.7, "Initial empathy should be 0.7");
    
    let nonexistent = tracker.get_trait_value("nonexistent").await;
    assert!(nonexistent.is_none(), "Nonexistent trait should return None");
}

#[tokio::test]
async fn test_update_trait_changes_value() {
    let tracker = PersonalityEvolutionTracker::new();
    
    let old_value = tracker.get_trait_value("empathy").await.unwrap();
    
    tracker
        .update_trait("empathy", 0.9, Some("Practiced empathy responses".to_string()))
        .await
        .unwrap();
    
    let new_value = tracker.get_trait_value("empathy").await.unwrap();
    
    assert_eq!(new_value, 0.9);
    assert_ne!(old_value, new_value);
}

#[tokio::test]
async fn test_update_trait_clamps_values() {
    let tracker = PersonalityEvolutionTracker::new();
    
    // Попытка установить значение > 1.0
    tracker.update_trait("curiosity", 1.5, None).await.unwrap();
    let value = tracker.get_trait_value("curiosity").await.unwrap();
    assert_eq!(value, 1.0, "Value should be clamped to 1.0");
    
    // Попытка установить значение < 0.0
    tracker.update_trait("curiosity", -0.5, None).await.unwrap();
    let value = tracker.get_trait_value("curiosity").await.unwrap();
    assert_eq!(value, 0.0, "Value should be clamped to 0.0");
}

#[tokio::test]
async fn test_trait_evolution_history_recorded() {
    let tracker = PersonalityEvolutionTracker::new();
    
    // Провести серию изменений
    tracker.update_trait("patience", 0.5, Some("Change 1".to_string())).await.unwrap();
    tracker.update_trait("patience", 0.6, Some("Change 2".to_string())).await.unwrap();
    tracker.update_trait("patience", 0.7, Some("Change 3".to_string())).await.unwrap();
    
    let evolution = tracker.get_trait_evolution("patience").await;
    
    assert!(evolution.is_some(), "Evolution should be recorded");
    let evolution = evolution.unwrap();
    
    assert_eq!(evolution.snapshots.len(), 3, "Should have 3 snapshots");
    assert_eq!(evolution.snapshots[0].value, 0.5);
    assert_eq!(evolution.snapshots[1].value, 0.6);
    assert_eq!(evolution.snapshots[2].value, 0.7);
    
    assert_eq!(evolution.snapshots[0].change_reason, Some("Change 1".to_string()));
}

#[tokio::test]
async fn test_create_personality_snapshot() {
    let tracker = PersonalityEvolutionTracker::new();
    
    let snapshot = tracker
        .create_snapshot("Before training session".to_string())
        .await
        .unwrap();
    
    assert_eq!(snapshot.context, "Before training session");
    assert!(snapshot.traits.contains_key("empathy"));
    assert!(snapshot.traits.len() >= 6);
    
    // Проверить, что снапшот сохранился
    let snapshots = tracker.get_snapshots().await;
    assert_eq!(snapshots.len(), 1);
}

#[tokio::test]
async fn test_multiple_snapshots_stored() {
    let tracker = PersonalityEvolutionTracker::new();
    
    tracker.create_snapshot("Snapshot 1".to_string()).await.unwrap();
    tracker.update_trait("empathy", 0.8, None).await.unwrap();
    tracker.create_snapshot("Snapshot 2".to_string()).await.unwrap();
    tracker.update_trait("curiosity", 0.9, None).await.unwrap();
    tracker.create_snapshot("Snapshot 3".to_string()).await.unwrap();
    
    let snapshots = tracker.get_snapshots().await;
    assert_eq!(snapshots.len(), 3);
    
    assert_eq!(snapshots[0].context, "Snapshot 1");
    assert_eq!(snapshots[1].context, "Snapshot 2");
    assert_eq!(snapshots[2].context, "Snapshot 3");
}

#[tokio::test]
async fn test_analyze_personality_changes() {
    let tracker = PersonalityEvolutionTracker::new();
    
    // Создать начальный снапшот
    tracker.create_snapshot("Start of period".to_string()).await.unwrap();
    
    // Изменить несколько черт значительно
    tracker.update_trait("empathy", 0.95, Some("Major improvement".to_string())).await.unwrap();
    tracker.update_trait("curiosity", 0.5, Some("Decreased due to fatigue".to_string())).await.unwrap();
    tracker.update_trait("patience", 0.65, Some("Small increase".to_string())).await.unwrap();
    
    // Создать конечный снапшот
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    tracker.create_snapshot("End of period".to_string()).await.unwrap();
    
    // Анализ за широкий период
    let start = Utc::now() - chrono::Duration::hours(1);
    let end = Utc::now() + chrono::Duration::hours(1);
    
    let analysis = tracker.analyze_changes(start, end).await.unwrap();
    
    assert!(!analysis.significant_changes.is_empty(), "Should have detected changes");
    assert!(analysis.stability_score >= 0.0 && analysis.stability_score <= 1.0);
    assert!(analysis.alignment_with_values >= 0.0 && analysis.alignment_with_values <= 1.0);
    
    // Проверить значимые изменения
    let significant = analysis.significant_changes.iter().filter(|c| c.is_significant).count();
    assert!(significant >= 1, "Should have at least 1 significant change (empathy changed by 0.25)");
}

#[tokio::test]
async fn test_core_values_exist() {
    let tracker = PersonalityEvolutionTracker::new();
    let values = tracker.get_core_values();
    
    assert!(!values.is_empty(), "Should have core values");
    
    let help_humans = values.iter().find(|v| v.name == "help_humans");
    assert!(help_humans.is_some(), "Should have 'help_humans' value");
    assert_eq!(help_humans.unwrap().importance, 1.0, "help_humans should be most important");
}

#[tokio::test]
async fn test_evolution_stats() {
    let tracker = PersonalityEvolutionTracker::new();
    
    // Создать несколько снапшотов и изменений
    tracker.create_snapshot("Snap 1".to_string()).await.unwrap();
    tracker.update_trait("empathy", 0.8, None).await.unwrap();
    tracker.update_trait("patience", 0.7, None).await.unwrap();
    tracker.create_snapshot("Snap 2".to_string()).await.unwrap();
    
    let stats = tracker.get_evolution_stats().await;
    
    assert_eq!(stats.total_traits, 6);
    assert_eq!(stats.total_snapshots, 2);
    assert_eq!(stats.traits_with_history, 2); // empathy и patience
    assert!(stats.oldest_snapshot.is_some());
    assert!(stats.newest_snapshot.is_some());
}

#[tokio::test]
async fn test_trait_evolution_history_limited_to_100() {
    let tracker = PersonalityEvolutionTracker::new();
    
    // Создать 150 изменений
    for i in 0..150 {
        let value = 0.5 + (i as f32 * 0.001);
        tracker.update_trait("creativity", value.min(1.0), None).await.unwrap();
    }
    
    let evolution = tracker.get_trait_evolution("creativity").await.unwrap();
    
    assert_eq!(evolution.snapshots.len(), 100, "Should limit history to 100 snapshots");
}

#[tokio::test]
async fn test_stability_score_high_for_no_changes() {
    let tracker = PersonalityEvolutionTracker::new();
    
    tracker.create_snapshot("Start".to_string()).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    tracker.create_snapshot("End".to_string()).await.unwrap();
    
    let start = Utc::now() - chrono::Duration::hours(1);
    let end = Utc::now() + chrono::Duration::hours(1);
    
    let analysis = tracker.analyze_changes(start, end).await.unwrap();
    
    // Без изменений стабильность должна быть близка к 1.0
    assert!(analysis.stability_score > 0.95, "Stability should be high with no changes, got {}", analysis.stability_score);
}
