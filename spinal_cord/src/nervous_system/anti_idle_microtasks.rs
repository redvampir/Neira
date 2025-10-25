/* neira:meta
id: NEI-20270318-120070-anti-idle-microtasks
intent: feature
summary: |
  Реализован диспетчер микрозадач простоя: очередь, запуск, метрики и снимки
  состояния для анти-айдла и обучающих циклов.
*/
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock, RwLock};
use std::time::{Duration, Instant};

use serde::Serialize;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

type TaskFuture = Pin<Box<dyn Future<Output = MicrotaskResult> + Send + 'static>>;
pub type TaskRunner = Arc<dyn Fn() -> TaskFuture + Send + Sync + 'static>;
pub type TaskEnabled = Arc<dyn Fn() -> bool + Send + Sync + 'static>;

#[derive(Clone)]
pub struct MicrotaskRegistration {
    pub id: String,
    pub display_name: String,
    pub min_idle_state: u32,
    pub cooldown: Duration,
    pub enabled: TaskEnabled,
    pub runner: TaskRunner,
}

impl MicrotaskRegistration {
    pub fn new(
        id: impl Into<String>,
        display_name: impl Into<String>,
        min_idle_state: u32,
        cooldown: Duration,
        enabled: TaskEnabled,
        runner: TaskRunner,
    ) -> Self {
        Self {
            id: id.into(),
            display_name: display_name.into(),
            min_idle_state,
            cooldown,
            enabled,
            runner,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub enum MicrotaskStatus {
    Completed,
    Skipped,
    Failed,
}

#[derive(Clone, Debug, Serialize)]
pub struct MicrotaskResult {
    pub status: MicrotaskStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl MicrotaskResult {
    pub fn completed(message: impl Into<Option<String>>) -> Self {
        Self {
            status: MicrotaskStatus::Completed,
            message: message.into(),
        }
    }

    pub fn skipped(message: impl Into<Option<String>>) -> Self {
        Self {
            status: MicrotaskStatus::Skipped,
            message: message.into(),
        }
    }

    pub fn failed(message: impl Into<Option<String>>) -> Self {
        Self {
            status: MicrotaskStatus::Failed,
            message: message.into(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct MicrotaskSnapshot {
    pub id: String,
    pub display_name: String,
    pub enabled: bool,
    pub running: bool,
    pub min_idle_state: u32,
    pub cooldown_seconds: u64,
    pub cooldown_remaining_seconds: u64,
}

pub struct AntiIdleMicrotaskService {
    tasks: RwLock<Vec<Arc<TaskEntry>>>,
    last_depth: AtomicUsize,
}

impl AntiIdleMicrotaskService {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            tasks: RwLock::new(Vec::new()),
            last_depth: AtomicUsize::new(0),
        })
    }

    pub fn instance() -> &'static Arc<Self> {
        static INSTANCE: OnceLock<Arc<AntiIdleMicrotaskService>> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    pub fn register(&self, reg: MicrotaskRegistration) -> Result<(), String> {
        let mut guard = self
            .tasks
            .write()
            .map_err(|_| "lock poisoned при регистрации микрозадачи".to_string())?;
        if guard.iter().any(|t| t.id == reg.id) {
            return Err(format!("микрозадача {} уже зарегистрирована", reg.id));
        }
        info!(id = reg.id, "регистрация микрозадачи простоя");
        guard.push(Arc::new(TaskEntry::new(reg)));
        Ok(())
    }

    pub async fn drive(&self, idle_state: u32) -> usize {
        let snapshot = {
            let guard = match self.tasks.read() {
                Ok(g) => g.clone(),
                Err(_) => Vec::new(),
            };
            guard
        };
        let mut depth = 0usize;
        for task in snapshot {
            if !task.is_enabled() {
                continue;
            }
            if idle_state < task.min_idle_state {
                continue;
            }
            if task.is_running() {
                depth += 1;
                continue;
            }
            if task.is_ready().await {
                depth += 1;
                task.start();
            }
        }
        self.last_depth.store(depth, Ordering::Relaxed);
        depth
    }

    pub fn last_depth(&self) -> usize {
        self.last_depth.load(Ordering::Relaxed)
    }

    pub async fn snapshot(&self) -> Vec<MicrotaskSnapshot> {
        let tasks = {
            let guard = match self.tasks.read() {
                Ok(g) => g.clone(),
                Err(_) => Vec::new(),
            };
            guard
        };
        let mut out = Vec::with_capacity(tasks.len());
        for task in tasks {
            out.push(task.snapshot().await);
        }
        out
    }
}

pub fn register_microtask(reg: MicrotaskRegistration) -> Result<(), String> {
    AntiIdleMicrotaskService::instance().register(reg)
}

pub async fn drive_microtasks(idle_state: u32) -> usize {
    AntiIdleMicrotaskService::instance().drive(idle_state).await
}

pub fn microtask_depth() -> usize {
    AntiIdleMicrotaskService::instance().last_depth()
}

pub async fn microtask_snapshot() -> Vec<MicrotaskSnapshot> {
    AntiIdleMicrotaskService::instance().snapshot().await
}

struct TaskEntry {
    id: String,
    display_name: String,
    min_idle_state: u32,
    cooldown: Duration,
    enabled: TaskEnabled,
    runner: TaskRunner,
    last_start: Mutex<Option<Instant>>,
    running: AtomicBool,
}

impl TaskEntry {
    fn new(reg: MicrotaskRegistration) -> Self {
        Self {
            id: reg.id,
            display_name: reg.display_name,
            min_idle_state: reg.min_idle_state,
            cooldown: reg.cooldown,
            enabled: reg.enabled,
            runner: reg.runner,
            last_start: Mutex::new(None),
            running: AtomicBool::new(false),
        }
    }

    fn is_enabled(&self) -> bool {
        (self.enabled)()
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    async fn is_ready(&self) -> bool {
        let guard = self.last_start.lock().await;
        if let Some(last) = *guard {
            last.elapsed() >= self.cooldown
        } else {
            true
        }
    }

    fn start(self: Arc<Self>) {
        if self
            .running
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return;
        }
        tokio::spawn(async move {
            let started = Instant::now();
            {
                let mut lock = self.last_start.lock().await;
                *lock = Some(started);
            }
            metrics::counter!("auto_tasks_started", "task" => self.id.clone()).increment(1);
            debug!(task = %self.id, "запуск микрозадачи простоя");
            let result = (self.runner)().await;
            match result.status {
                MicrotaskStatus::Completed => {
                    metrics::counter!(
                        "auto_tasks_completed",
                        "task" => self.id.clone()
                    )
                    .increment(1);
                    info!(task = %self.id, msg = ?result.message, "микрозадача завершена");
                }
                MicrotaskStatus::Skipped => {
                    debug!(task = %self.id, msg = ?result.message, "микрозадача пропущена");
                }
                MicrotaskStatus::Failed => {
                    metrics::counter!(
                        "auto_tasks_blocked",
                        "task" => self.id.clone()
                    )
                    .increment(1);
                    warn!(task = %self.id, msg = ?result.message, "микрозадача завершилась ошибкой");
                }
            }
            self.running.store(false, Ordering::Release);
        });
    }

    async fn snapshot(&self) -> MicrotaskSnapshot {
        let remaining = if let Some(last) = *self.last_start.lock().await {
            let elapsed = last.elapsed();
            if elapsed >= self.cooldown {
                0
            } else {
                (self.cooldown - elapsed).as_secs()
            }
        } else {
            0
        };
        MicrotaskSnapshot {
            id: self.id.clone(),
            display_name: self.display_name.clone(),
            enabled: self.is_enabled(),
            running: self.is_running(),
            min_idle_state: self.min_idle_state,
            cooldown_seconds: self.cooldown.as_secs(),
            cooldown_remaining_seconds: remaining,
        }
    }
}

/* neira:meta
id: NEI-20270319-anti-idle-tests
intent: test
summary: Добавлены тесты сервиса микрозадач простоя: запуск и учёт порога простоя.
*/
#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    fn unique_id(prefix: &str) -> String {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let next = COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("{}.{}", prefix, next)
    }

    #[tokio::test]
    #[serial]
    async fn запускает_микрозадачу_при_готовности() {
        let flag = Arc::new(AtomicBool::new(true));
        let enabled_flag = flag.clone();
        let runs = Arc::new(AtomicUsize::new(0));
        let runs_clone = runs.clone();
        let reg = MicrotaskRegistration::new(
            unique_id("test.microtask"),
            "Тестовая микрозадача",
            2,
            Duration::from_millis(0),
            Arc::new(move || enabled_flag.load(Ordering::Relaxed)),
            Arc::new(move || {
                let counter = runs_clone.clone();
                Box::pin(async move {
                    counter.fetch_add(1, Ordering::Relaxed);
                    MicrotaskResult::completed(None)
                })
            }),
        );
        register_microtask(reg).expect("регистрация");
        let depth = drive_microtasks(3).await;
        tokio::time::sleep(Duration::from_millis(50)).await;
        flag.store(false, Ordering::Relaxed);
        assert_eq!(depth, 1, "должна запускаться одна задача");
        assert_eq!(runs.load(Ordering::Relaxed), 1, "микрозадача должна выполниться");
    }

    #[tokio::test]
    #[serial]
    async fn уважает_порог_простая() {
        let flag = Arc::new(AtomicBool::new(true));
        let enabled_flag = flag.clone();
        let runs = Arc::new(AtomicUsize::new(0));
        let runs_clone = runs.clone();
        let reg = MicrotaskRegistration::new(
            unique_id("test.microtask.idle"),
            "Микрозадача с порогом",
            3,
            Duration::from_secs(1),
            Arc::new(move || enabled_flag.load(Ordering::Relaxed)),
            Arc::new(move || {
                let counter = runs_clone.clone();
                Box::pin(async move {
                    counter.fetch_add(1, Ordering::Relaxed);
                    MicrotaskResult::completed(None)
                })
            }),
        );
        register_microtask(reg).expect("регистрация");
        let depth = drive_microtasks(1).await;
        tokio::time::sleep(Duration::from_millis(50)).await;
        flag.store(false, Ordering::Relaxed);
        assert_eq!(depth, 0, "очередь должна быть пустой");
        assert_eq!(runs.load(Ordering::Relaxed), 0, "не должна выполняться на низком простое");
    }
}
