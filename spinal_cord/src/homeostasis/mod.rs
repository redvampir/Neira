/* neira:meta
id: NEI-20251103-homeostasis-budgets-core
intent: feature
summary: |
  Реализация Homeostasis Budgets — автоматическая саморегуляция Нейры.
  Динамические бюджеты CPU/Memory/Latency с backpressure и адаптивным backoff.
*/

use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

/// Бюджеты ресурсов для саморегуляции
#[derive(Debug, Clone)]
pub struct ResourceBudgets {
    /// Бюджет CPU (% использования)
    pub cpu_budget: f64,
    /// Бюджет памяти (MB)
    pub memory_budget: u64,
    /// Бюджет задержки (ms)
    pub latency_budget: Duration,
    /// Текущее использование
    pub current_usage: ResourceUsage,
}

#[derive(Debug, Clone, Default)]
pub struct ResourceUsage {
    pub cpu_percent: f64,
    pub memory_mb: u64,
    pub avg_latency_ms: u64,
}

/// Homeostasis Engine - мозг саморегуляции
pub struct HomeostasisEngine {
    budgets: Arc<RwLock<ResourceBudgets>>,
    backpressure_active: Arc<RwLock<bool>>,
    last_adjustment: Arc<RwLock<Instant>>,
    adjustment_interval: Duration,
}

impl HomeostasisEngine {
    pub fn new() -> Self {
        Self {
            budgets: Arc::new(RwLock::new(ResourceBudgets {
                cpu_budget: 70.0,  // 70% CPU max
                memory_budget: 512, // 512 MB max
                latency_budget: Duration::from_millis(1000),
                current_usage: ResourceUsage::default(),
            })),
            backpressure_active: Arc::new(RwLock::new(false)),
            last_adjustment: Arc::new(RwLock::new(Instant::now())),
            adjustment_interval: Duration::from_secs(5), // проверка каждые 5 секунд
        }
    }

    /// Автоматическая подстройка под нагрузку (с автоопределением текущей нагрузки)
    pub async fn auto_adjust(&self) -> AdjustmentDecision {
        let mut last_adj = self.last_adjustment.write().await;
        if last_adj.elapsed() < self.adjustment_interval {
            return AdjustmentDecision::NoChange;
        }
        *last_adj = Instant::now();
        drop(last_adj);

        // Получить текущую нагрузку
        let current = self.measure_current_usage().await;
        
        let mut budgets = self.budgets.write().await;
        budgets.current_usage = current.clone();

        // Анализ состояния
        let cpu_stress = current.cpu_percent / budgets.cpu_budget;
        let mem_stress = current.memory_mb as f64 / budgets.memory_budget as f64;
        let lat_stress = current.avg_latency_ms as f64 / budgets.latency_budget.as_millis() as f64;

        let overall_stress = (cpu_stress + mem_stress + lat_stress) / 3.0;

        if overall_stress > 0.85 {
            // Перегрузка — активируем backpressure
            *self.backpressure_active.write().await = true;
            AdjustmentDecision::ActivateBackpressure {
                reason: format!("Stress level: {:.2}", overall_stress),
                throttle_factor: 0.5, // замедляем вдвое
            }
        } else if overall_stress < 0.3 {
            // Недогрузка — можно ускориться
            *self.backpressure_active.write().await = false;
            AdjustmentDecision::Accelerate {
                boost_factor: 1.5,
            }
        } else {
            *self.backpressure_active.write().await = false;
            AdjustmentDecision::MaintainPace
        }
    }

    /// Проверка: можно ли принять новую задачу?
    pub async fn can_accept_task(&self) -> bool {
        !*self.backpressure_active.read().await
    }

    /// Получить текущий уровень стресса (0.0 - 1.0)
    pub async fn get_stress_level(&self) -> f64 {
        let budgets = self.budgets.read().await;
        let current = &budgets.current_usage;
        
        let cpu_stress = current.cpu_percent / budgets.cpu_budget;
        let mem_stress = current.memory_mb as f64 / budgets.memory_budget as f64;
        let lat_stress = current.avg_latency_ms as f64 / budgets.latency_budget.as_millis() as f64;

        (cpu_stress + mem_stress + lat_stress) / 3.0
    }
    
    /// Измерить текущую нагрузку (простая версия для прототипа)
    async fn measure_current_usage(&self) -> ResourceUsage {
        // TODO: реальная интеграция с системными метриками
        // Пока возвращаем mock-значения
        ResourceUsage {
            cpu_percent: 30.0,
            memory_mb: 256,
            avg_latency_ms: 100,
        }
    }
    
    /// Получить текущие бюджеты (для API)
    pub async fn current_budgets(&self) -> CurrentBudgets {
        let budgets = self.budgets.read().await;
        CurrentBudgets {
            cpu_usage_percent: budgets.current_usage.cpu_percent,
            memory_used_mb: budgets.current_usage.memory_mb,
            avg_latency_ms: budgets.current_usage.avg_latency_ms,
            cpu_max_percent: budgets.cpu_budget,
            memory_max_mb: budgets.memory_budget,
            latency_max_ms: budgets.latency_budget.as_millis() as u64,
        }
    }
    
    /// Получить текущий уровень стресса (wrapper для API)
    pub async fn current_stress_level(&self) -> f32 {
        self.get_stress_level().await as f32
    }
}

#[derive(Debug, Clone)]
pub struct CurrentBudgets {
    pub cpu_usage_percent: f64,
    pub memory_used_mb: u64,
    pub avg_latency_ms: u64,
    pub cpu_max_percent: f64,
    pub memory_max_mb: u64,
    pub latency_max_ms: u64,
}

#[derive(Debug)]
pub enum AdjustmentDecision {
    NoChange,
    ActivateBackpressure {
        reason: String,
        throttle_factor: f64,
    },
    Accelerate {
        boost_factor: f64,
    },
    MaintainPace,
}

/// Adaptive Backoff Strategy - умное замедление
pub struct AdaptiveBackoff {
    base_delay: Duration,
    max_delay: Duration,
    current_delay: Duration,
    failure_count: u32,
}

impl AdaptiveBackoff {
    pub fn new() -> Self {
        Self {
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            current_delay: Duration::from_millis(100),
            failure_count: 0,
        }
    }

    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        // Экспоненциальный рост задержки
        self.current_delay = std::cmp::min(
            self.current_delay * 2,
            self.max_delay,
        );
    }

    pub fn record_success(&mut self) {
        self.failure_count = 0;
        // Постепенное снижение задержки
        self.current_delay = std::cmp::max(
            self.current_delay / 2,
            self.base_delay,
        );
    }

    pub fn get_delay(&self) -> Duration {
        self.current_delay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_homeostasis_backpressure() {
        let engine = HomeostasisEngine::new();
        
        // Обновляем текущее использование вручную для теста
        {
            let mut budgets = engine.budgets.write().await;
            budgets.current_usage = ResourceUsage {
                cpu_percent: 90.0,
                memory_mb: 600,
                avg_latency_ms: 2000,
            };
        }

        let decision = engine.auto_adjust().await;
        
        // После авторегулировки должен быть backpressure
        assert!(!engine.can_accept_task().await, "Backpressure should be active");
    }

    #[tokio::test]
    async fn test_adaptive_backoff() {
        let mut backoff = AdaptiveBackoff::new();
        
        // Первая ошибка
        backoff.record_failure();
        let delay1 = backoff.get_delay();
        
        // Вторая ошибка
        backoff.record_failure();
        let delay2 = backoff.get_delay();
        
        assert!(delay2 > delay1, "Delay should increase");
        
        // Успех — задержка уменьшается
        backoff.record_success();
        let delay3 = backoff.get_delay();
        assert!(delay3 < delay2, "Delay should decrease after success");
    }
}
