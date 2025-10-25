/* neira:meta
id: NEI-20270318-120090-training-orchestrator
intent: feature
summary: |
  Добавлен TrainingOrchestrator: автоматический запуск Scripted Training во
  время простоя, учёт ошибок, метрики и регистрация в Anti-Idle микрозадачах.
*/
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use tracing::{error, info, warn};

use crate::action::scripted_training_cell::ScriptedTrainingCell;
use crate::nervous_system::anti_idle_microtasks::{
    register_microtask, MicrotaskRegistration, MicrotaskResult, TaskEnabled, TaskRunner,
};
use crate::synapse_hub::SynapseHub;

pub struct TrainingOrchestrator {
    id: String,
    hub: Arc<SynapseHub>,
    cell: Arc<ScriptedTrainingCell>,
    running: AtomicBool,
    failures: AtomicU32,
    max_failures: u32,
    min_idle_state: u32,
    cooldown: Duration,
}

impl TrainingOrchestrator {
    pub fn new(hub: Arc<SynapseHub>) -> Arc<Self> {
        let min_idle_state = std::env::var("TRAINING_AUTORUN_MIN_IDLE_STATE")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(2)
            .clamp(1, 3);
        let cooldown_minutes = std::env::var("TRAINING_AUTORUN_INTERVAL_MINUTES")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(120);
        let max_failures = std::env::var("TRAINING_AUTORUN_MAX_FAILURES")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(3)
            .max(1);
        Arc::new(Self {
            id: "training.orchestrator".into(),
            hub,
            cell: Arc::new(ScriptedTrainingCell::from_env()),
            running: AtomicBool::new(false),
            failures: AtomicU32::new(0),
            max_failures,
            min_idle_state,
            cooldown: Duration::from_secs(cooldown_minutes * 60),
        })
    }

    pub fn register(self: Arc<Self>) -> Result<(), String> {
        let enabled_hub = self.hub.clone();
        let enabled: TaskEnabled = Arc::new(move || {
            enabled_hub.learning_microtasks_enabled()
                && enabled_hub.training_pipeline_enabled()
                && enabled_hub.training_autorun_enabled()
                && !enabled_hub.is_safe_mode()
        });
        let runner_self = self.clone();
        let runner: TaskRunner = Arc::new(move || {
            let orchestrator = runner_self.clone();
            Box::pin(async move { orchestrator.run_cycle().await })
        });
        register_microtask(MicrotaskRegistration::new(
            self.id.clone(),
            "Автообучение Scripted Training",
            self.min_idle_state,
            self.cooldown,
            enabled,
            runner,
        ))
    }

    async fn run_cycle(self: Arc<Self>) -> MicrotaskResult {
        if !self.hub.training_pipeline_enabled() {
            return MicrotaskResult::skipped(Some("тренинг заблокирован".into()));
        }
        if !self.hub.training_autorun_enabled() {
            return MicrotaskResult::skipped(Some("автозапуск отключён".into()));
        }
        if self
            .running
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return MicrotaskResult::skipped(Some("уже выполняется".into()));
        }
        metrics::counter!("training_iterations_total", "mode" => "auto").increment(1);
        let started = Instant::now();
        info!("старт автоматического обучения");
        let result = self.cell.run().await;
        self.running.store(false, Ordering::Release);
        match result {
            Ok(()) => {
                self.failures.store(0, Ordering::Relaxed);
                metrics::counter!("training_converged_total", "mode" => "auto").increment(1);
                let secs = started.elapsed().as_secs();
                info!(duration = secs, "обучение завершено");
                MicrotaskResult::completed(Some(format!("завершено за {} с", secs)))
            }
            Err(err) => {
                let current = self.failures.fetch_add(1, Ordering::Relaxed) + 1;
                error!("ошибка обучающего цикла: {}", err);
                if current >= self.max_failures {
                    warn!(
                        failures = current,
                        "превышен лимит ошибок, автозапуск приостановлен"
                    );
                    return MicrotaskResult::failed(Some(format!(
                        "превышен лимит ошибок ({}), требуется вмешательство",
                        self.max_failures
                    )));
                }
                MicrotaskResult::failed(Some(err))
            }
        }
    }
}

/* neira:meta
id: NEI-20270319-training-tests
intent: test
summary: Покрыт оркестратор обучения тестами: успешный запуск и обработка ошибки сценария.
*/
#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::diagnostics_cell::DiagnosticsCell;
    use crate::action::metrics_collector_cell::MetricsCollectorCell;
    use crate::cell_registry::CellRegistry;
    use crate::config::Config;
    use crate::memory_cell::MemoryCell;
    use crate::nervous_system::anti_idle_microtasks::MicrotaskStatus;
    use serial_test::serial;
    use std::path::Path;
    use tempfile::tempdir;

    fn build_hub(dir: &Path) -> Arc<SynapseHub> {
        let registry = Arc::new(CellRegistry::new(dir).expect("registry"));
        let memory = Arc::new(MemoryCell::new());
        let (metrics, rx) = MetricsCollectorCell::channel();
        let (diagnostics, _dev_rx, _alert_rx) = DiagnosticsCell::new(rx, 10, metrics.clone());
        let cfg = Config::default();
        Arc::new(SynapseHub::new(registry, memory, metrics, diagnostics, &cfg))
    }

    fn set_env(key: &str, value: &str) {
        std::env::set_var(key, value);
    }

    fn clear_env(key: &str) {
        std::env::remove_var(key);
    }

    fn cleanup_env(vars: &[&str]) {
        for key in vars {
            clear_env(key);
        }
    }

    #[tokio::test]
    #[serial]
    async fn run_cycle_успешен_при_пустом_сценарии() {
        let dir = tempdir().expect("tempdir");
        let script_path = dir.path().join("script.yaml");
        let progress_path = dir.path().join("progress.json");
        std::fs::write(&script_path, "name: 'Empty'\nsteps: []\n").expect("script");
        let script_path_str = script_path.to_string_lossy().to_string();
        let progress_path_str = progress_path.to_string_lossy().to_string();
        set_env("LEARNING_MICROTASKS_ENABLED", "1");
        set_env("TRAINING_PIPELINE_ENABLED", "1");
        set_env("TRAINING_AUTORUN_ENABLED", "1");
        set_env("TRAINING_SCRIPT", &script_path_str);
        set_env("TRAINING_PROGRESS", &progress_path_str);
        set_env("TRAINING_AUTORUN_MAX_FAILURES", "3");
        set_env("TRAINING_AUTORUN_INTERVAL_MINUTES", "1");
        set_env("TRAINING_AUTORUN_MIN_IDLE_STATE", "2");
        let hub = build_hub(dir.path());
        let orchestrator = TrainingOrchestrator::new(hub);
        let result = orchestrator.clone().run_cycle().await;
        cleanup_env(&[
            "LEARNING_MICROTASKS_ENABLED",
            "TRAINING_PIPELINE_ENABLED",
            "TRAINING_AUTORUN_ENABLED",
            "TRAINING_SCRIPT",
            "TRAINING_PROGRESS",
            "TRAINING_AUTORUN_MAX_FAILURES",
            "TRAINING_AUTORUN_INTERVAL_MINUTES",
            "TRAINING_AUTORUN_MIN_IDLE_STATE",
        ]);
        assert_eq!(result.status, MicrotaskStatus::Completed);
    }

    #[tokio::test]
    #[serial]
    async fn run_cycle_останавливает_автостарт_при_ошибке() {
        let dir = tempdir().expect("tempdir");
        let missing_script = dir.path().join("missing.yaml");
        let progress_path = dir.path().join("progress.json");
        let missing_script_str = missing_script.to_string_lossy().to_string();
        let progress_path_str = progress_path.to_string_lossy().to_string();
        set_env("LEARNING_MICROTASKS_ENABLED", "1");
        set_env("TRAINING_PIPELINE_ENABLED", "1");
        set_env("TRAINING_AUTORUN_ENABLED", "1");
        set_env("TRAINING_SCRIPT", &missing_script_str);
        set_env("TRAINING_PROGRESS", &progress_path_str);
        set_env("TRAINING_AUTORUN_MAX_FAILURES", "1");
        set_env("TRAINING_AUTORUN_INTERVAL_MINUTES", "1");
        set_env("TRAINING_AUTORUN_MIN_IDLE_STATE", "2");
        let hub = build_hub(dir.path());
        let orchestrator = TrainingOrchestrator::new(hub);
        let result = orchestrator.clone().run_cycle().await;
        cleanup_env(&[
            "LEARNING_MICROTASKS_ENABLED",
            "TRAINING_PIPELINE_ENABLED",
            "TRAINING_AUTORUN_ENABLED",
            "TRAINING_SCRIPT",
            "TRAINING_PROGRESS",
            "TRAINING_AUTORUN_MAX_FAILURES",
            "TRAINING_AUTORUN_INTERVAL_MINUTES",
            "TRAINING_AUTORUN_MIN_IDLE_STATE",
        ]);
        assert_eq!(result.status, MicrotaskStatus::Failed);
        let msg = result.message.unwrap_or_default();
        assert!(msg.contains("превышен лимит ошибок"), "ожидали приостановку, получили: {}", msg);
    }
}
