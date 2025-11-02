use super::*;
use tokio::test;
use mock::MockCurriculum;

#[test]
async fn test_adaptive_difficulty() {
    let metrics = Arc::new(LearningMetrics::new());
    let curriculum = Arc::new(MockCurriculum::default());
    let scheduler = AdaptiveScheduler::new(curriculum, metrics.clone());

    // Тестируем адаптацию при высоком успехе
    metrics.record_attempt(true, 0.5).await;
    metrics.record_attempt(true, 0.5).await;
    let lesson = scheduler.next_lesson().await;
    assert!(lesson.difficulty > 0.5);

    // Тестируем адаптацию при низком успехе
    metrics.record_attempt(false, 0.7).await;
    metrics.record_attempt(false, 0.7).await;
    let lesson = scheduler.next_lesson().await;
    assert!(lesson.difficulty < 0.7);
}
