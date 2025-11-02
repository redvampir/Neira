use super::*;
use tempfile::TempDir;
use std::env;

#[tokio::test]
async fn test_metrics_recording() {
    let metrics = LearningMetrics::new();
    
    metrics.record_attempt(true, 0.5).await;
    metrics.record_attempt(false, 0.6).await;
    
    let stats = metrics.stats.read().await;
    assert_eq!(stats.success_count, 1);
    assert_eq!(stats.total_attempts, 2);
}

#[tokio::test]
async fn test_metrics_persistence() -> Result<(), Box<dyn std::error::Error>> {
    let temp = TempDir::new()?;
    env::set_var("NEIRA_DATA_DIR", temp.path());
    
    let metrics = LearningMetrics::new();
    metrics.record_attempt(true, 0.5).await;
    metrics.save_progress().await?;
    
    // Создаем новый экземпляр и проверяем загрузку
    let new_metrics = LearningMetrics::new();
    new_metrics.load_progress().await?;
    
    let stats = new_metrics.stats.read().await;
    assert_eq!(stats.success_count, 1);
    
    Ok(())
}
