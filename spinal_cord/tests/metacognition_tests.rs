/* neira:meta
id: NEI-20260131-metacognition-tests
intent: test
summary: |
  Unit и интеграционные тесты для MetaCognition Engine.
  Проверяет: запись thought traces, обнаружение bias (overconfidence, confirmation),
  генерацию improvement tasks из рефлексий и growth tracker.
*/

use backend::consciousness::{
    MetaCognitionEngine, Decision, BiasType, BiasSeverity, TaskType, TaskStatus,
};
use backend::dialogue::{ReflectionEntry, GrowthTracker, SkillLevel};
use std::collections::HashMap;
use chrono::Utc;

#[tokio::test]
async fn test_record_thought_trace() {
    let engine = MetaCognitionEngine::new();
    
    let reasoning_steps = vec![
        "Step 1: Analyze user input".to_string(),
        "Step 2: Search similar memories".to_string(),
        "Step 3: Generate response".to_string(),
    ];
    
    let decisions = vec![
        Decision {
            step: "Intent Classification".to_string(),
            rationale: "User seems to be asking for help".to_string(),
            confidence: 0.85,
            alternatives_considered: vec!["Gratitude".to_string(), "General".to_string()],
        },
    ];
    
    let trace_id = engine
        .record_thought_trace("conv_test_123".to_string(), reasoning_steps, decisions)
        .await
        .expect("Failed to record thought trace");
    
    assert!(trace_id.starts_with("thought_"));
    
    let stats = engine.get_stats().await;
    assert_eq!(stats.total_thoughts_recorded, 1);
    assert_eq!(stats.total_biases_detected, 0); // Ещё не анализировали
}

#[tokio::test]
async fn test_detect_overconfidence_bias() {
    let engine = MetaCognitionEngine::new();
    
    // Все решения с очень высокой уверенностью (>0.9) И их больше 2
    let decisions = vec![
        Decision {
            step: "Step 1".to_string(),
            rationale: "Very confident".to_string(),
            confidence: 0.95,
            alternatives_considered: vec!["Alt1".to_string()],
        },
        Decision {
            step: "Step 2".to_string(),
            rationale: "Also confident".to_string(),
            confidence: 0.92,
            alternatives_considered: vec!["Alt2".to_string()],
        },
        Decision {
            step: "Step 3".to_string(),
            rationale: "Third confident decision".to_string(),
            confidence: 0.93,
            alternatives_considered: vec!["Alt3".to_string()],
        },
    ];
    
    let trace_id = engine
        .record_thought_trace(
            "conv_overconfident".to_string(),
            vec!["Step 1".to_string(), "Step 2".to_string(), "Step 3".to_string()],
            decisions,
        )
        .await
        .unwrap();
    
    let biases = engine.analyze_thought_process(&trace_id).await.unwrap();
    
    // Должен обнаружить OverconfidenceBias
    assert!(!biases.is_empty(), "Expected to detect overconfidence bias");
    
    let overconfidence = biases.iter().find(|b| matches!(b.bias_type, BiasType::OverconfidenceBias));
    assert!(overconfidence.is_some(), "Should detect overconfidence bias");
    
    if let Some(bias) = overconfidence {
        assert!(matches!(bias.severity, BiasSeverity::Medium | BiasSeverity::High));
    }
    
    let stats = engine.get_stats().await;
    assert_eq!(stats.total_biases_detected, 1);
}

#[tokio::test]
async fn test_detect_confirmation_bias() {
    let engine = MetaCognitionEngine::new();
    
    // Решения без рассмотрения альтернатив
    let decisions = vec![
        Decision {
            step: "Quick decision".to_string(),
            rationale: "First idea is best".to_string(),
            confidence: 0.7,
            alternatives_considered: vec![], // Нет альтернатив!
        },
    ];
    
    let trace_id = engine
        .record_thought_trace(
            "conv_confirmation".to_string(),
            vec!["Quick step".to_string()],
            decisions,
        )
        .await
        .unwrap();
    
    let biases = engine.analyze_thought_process(&trace_id).await.unwrap();
    
    // Должен обнаружить ConfirmationBias
    let confirmation = biases.iter().find(|b| matches!(b.bias_type, BiasType::ConfirmationBias));
    assert!(confirmation.is_some(), "Should detect confirmation bias when no alternatives considered");
}

#[tokio::test]
async fn test_generate_improvement_tasks_from_failures() {
    let engine = MetaCognitionEngine::new();
    
    // Reflection entries с частыми неудачами (одна и та же проблема повторяется)
    let reflection_entries = vec![
        ReflectionEntry {
            conversation_id: "conv1".to_string(),
            timestamp: chrono::Utc::now(),
            what_worked: vec![],
            what_failed: vec!["Failed to show empathy".to_string()],
            what_to_try_next: vec!["Need better empathy".to_string()],
            confidence_before: 0.5,
            confidence_after: 0.3,
        },
        ReflectionEntry {
            conversation_id: "conv2".to_string(),
            timestamp: chrono::Utc::now(),
            what_worked: vec![],
            what_failed: vec!["Failed to show empathy".to_string()],
            what_to_try_next: vec!["Need better empathy".to_string()],
            confidence_before: 0.5,
            confidence_after: 0.3,
        },
        ReflectionEntry {
            conversation_id: "conv3".to_string(),
            timestamp: chrono::Utc::now(),
            what_worked: vec![],
            what_failed: vec!["Failed to show empathy".to_string()],
            what_to_try_next: vec!["Need better empathy".to_string()],
            confidence_before: 0.5,
            confidence_after: 0.2,
        },
        ReflectionEntry {
            conversation_id: "conv4".to_string(),
            timestamp: chrono::Utc::now(),
            what_worked: vec![],
            what_failed: vec!["Failed to show empathy".to_string()],
            what_to_try_next: vec!["Need better empathy".to_string()],
            confidence_before: 0.5,
            confidence_after: 0.3,
        },
        ReflectionEntry {
            conversation_id: "conv5".to_string(),
            timestamp: chrono::Utc::now(),
            what_worked: vec![],
            what_failed: vec!["Failed to show empathy".to_string()],
            what_to_try_next: vec!["Need better empathy".to_string()],
            confidence_before: 0.5,
            confidence_after: 0.2,
        },
        ReflectionEntry {
            conversation_id: "conv6".to_string(),
            timestamp: chrono::Utc::now(),
            what_worked: vec![],
            what_failed: vec!["Failed to show empathy".to_string()],
            what_to_try_next: vec!["Need better empathy".to_string()],
            confidence_before: 0.5,
            confidence_after: 0.3,
        },
    ];
    
    let mut skills = HashMap::new();
    skills.insert("empathy".to_string(), SkillLevel {
        name: "empathy".to_string(),
        level: 0.3, // Низкий уровень
        practice_count: 10, // Достаточно практики
        last_practiced: Utc::now(),
    });
    
    let growth_tracker = GrowthTracker {
        milestones: vec![],
        skills,
        total_conversations: 6,
        successful_helps: 0,
    };
    
    let tasks = engine
        .generate_improvement_tasks(&reflection_entries, &growth_tracker)
        .await
        .unwrap();
    
    println!("Generated {} tasks", tasks.len());
    for task in &tasks {
        println!("Task: {:?} - {}", task.task_type, task.description);
    }
    
    assert!(!tasks.is_empty(), "Should generate improvement tasks");
    
    // Проверяем, что есть задача ImproveResponseQuality (т.к. 6 неудач повторяются)
    let quality_task = tasks.iter().find(|t| matches!(t.task_type, TaskType::ImproveResponseQuality { .. }));
    assert!(quality_task.is_some(), "Should generate ImproveResponseQuality task for frequent failures");
    
    // Проверяем задачу PracticeSkill для низкого empathy skill
    let practice_task = tasks.iter().find(|t| {
        matches!(&t.task_type, TaskType::PracticeSkill { skill_name } if skill_name.contains("empathy"))
    });
    assert!(practice_task.is_some(), "Should generate PracticeSkill task for low empathy skill");
}

#[tokio::test]
async fn test_complete_task() {
    let engine = MetaCognitionEngine::new();
    
    // Генерируем задачи
    let reflection_entries = vec![
        ReflectionEntry {
            conversation_id: "conv1".to_string(),
            timestamp: chrono::Utc::now(),
            what_worked: vec![],
            what_failed: vec!["Issue 1".to_string()],
            what_to_try_next: vec!["Lesson 1".to_string()],
            confidence_before: 0.5,
            confidence_after: 0.5,
        },
    ];
    
    let growth_tracker = GrowthTracker {
        milestones: vec![],
        skills: HashMap::new(),
        total_conversations: 1,
        successful_helps: 0,
    };
    
    let tasks = engine
        .generate_improvement_tasks(&reflection_entries, &growth_tracker)
        .await
        .unwrap();
    
    if let Some(task) = tasks.first() {
        let task_id = task.id.clone();
        
        // Завершаем задачу
        engine.complete_task(&task_id).await.unwrap();
        
        // Проверяем статистику
        let stats = engine.get_stats().await;
        assert_eq!(stats.completed_tasks, 1);
        assert_eq!(stats.pending_tasks, 0);
    }
}

#[tokio::test]
async fn test_get_active_tasks() {
    let engine = MetaCognitionEngine::new();
    
    let reflection_entries = vec![
        ReflectionEntry {
            conversation_id: "conv1".to_string(),
            timestamp: chrono::Utc::now(),
            what_worked: vec![],
            what_failed: vec!["Same issue".to_string()],
            what_to_try_next: vec!["Lesson".to_string()],
            confidence_before: 0.5,
            confidence_after: 0.5,
        },
        ReflectionEntry {
            conversation_id: "conv2".to_string(),
            timestamp: chrono::Utc::now(),
            what_worked: vec![],
            what_failed: vec!["Same issue".to_string()],
            what_to_try_next: vec!["Lesson".to_string()],
            confidence_before: 0.5,
            confidence_after: 0.5,
        },
        ReflectionEntry {
            conversation_id: "conv3".to_string(),
            timestamp: chrono::Utc::now(),
            what_worked: vec![],
            what_failed: vec!["Same issue".to_string()],
            what_to_try_next: vec!["Lesson".to_string()],
            confidence_before: 0.5,
            confidence_after: 0.5,
        },
    ];
    
    let growth_tracker = GrowthTracker {
        milestones: vec![],
        skills: HashMap::new(),
        total_conversations: 1,
        successful_helps: 0,
    };
    
    let tasks_generated = engine
        .generate_improvement_tasks(&reflection_entries, &growth_tracker)
        .await
        .unwrap();
    
    println!("Generated {} tasks", tasks_generated.len());
    
    // Теперь получаем активные задачи
    let active = engine.get_active_tasks().await;
    println!("Active tasks: {}", active.len());
    
    assert!(!active.is_empty(), "Should have active tasks");
    
    // Все задачи должны быть в статусе Pending
    for task in &active {
        assert!(matches!(task.status, TaskStatus::Pending));
    }
}

#[tokio::test]
async fn test_no_bias_detection_for_good_reasoning() {
    let engine = MetaCognitionEngine::new();
    
    // Хорошие решения: умеренная уверенность, рассмотрены альтернативы
    let decisions = vec![
        Decision {
            step: "Careful analysis".to_string(),
            rationale: "Considered multiple options".to_string(),
            confidence: 0.7,
            alternatives_considered: vec!["Option A".to_string(), "Option B".to_string()],
        },
        Decision {
            step: "Balanced choice".to_string(),
            rationale: "Weighted pros and cons".to_string(),
            confidence: 0.6,
            alternatives_considered: vec!["Choice 1".to_string(), "Choice 2".to_string()],
        },
    ];
    
    let trace_id = engine
        .record_thought_trace(
            "conv_good_reasoning".to_string(),
            vec!["Step 1".to_string(), "Step 2".to_string()],
            decisions,
        )
        .await
        .unwrap();
    
    let biases = engine.analyze_thought_process(&trace_id).await.unwrap();
    
    // Не должно быть bias при хорошем рассуждении
    assert!(biases.is_empty(), "Should not detect biases for good reasoning");
}
