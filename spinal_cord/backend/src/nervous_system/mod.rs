/* neira:meta
id: NEI-20270318-120130-anti-idle-microtasks-export
intent: feature
summary: Экспортирован модуль anti_idle_microtasks для очереди микрозадач.
*/
/* neira:meta
id: NEI-20250215-ns-watch
intent: code
summary: Добавлен заглушечный watch для мониторинга записей фабрики.
*/
/* neira:meta
id: NEI-20240607-systemprobe-stop
intent: feature
summary: Трейт SystemProbe расширен методом stop для завершения фоновых циклов.
*/
use crate::event_bus::{CellCreated, Event, OrganBuilt, Subscriber};
use crate::factory::StemCellRecord;
use async_trait::async_trait;

/// Common interface for system-level probes that expose diagnostics and
/// metrics about the environment Neira runs in.
#[async_trait]
pub trait SystemProbe: Send + Sync {
    /// Start the probe. Implementations usually spawn a background loop and
    /// should not return under normal operation.
    async fn start(&mut self);

    /// Collect a single batch of metrics. The default implementation does
    /// nothing, allowing probes that operate only via `start` to leave it
    /// empty.
    fn collect(&mut self) {}

    /// Signal the probe to terminate its background loop and clean up.
    fn stop(&mut self);
}

pub fn watch(_record: &StemCellRecord) {
    metrics::counter!("nervous_watches_total").increment(1);
}

/* neira:meta
id: NEI-20251227-nervous-subscriber
intent: code
summary: Подписчик nervous_system на события CellCreated и OrganBuilt.
*/
pub struct NervousSystemSubscriber;

impl Subscriber for NervousSystemSubscriber {
    fn on_event(&self, event: &dyn Event) {
        if let Some(ev) = event.as_any().downcast_ref::<CellCreated>() {
            watch(&ev.record);
        } else if event.as_any().downcast_ref::<OrganBuilt>().is_some() {
            metrics::counter!("nervous_organs_total").increment(1);
        }
    }
}

pub mod anti_idle;
pub mod anti_idle_microtasks;
pub mod backpressure_probe;
pub mod base_path_resolver;
pub mod heartbeat;
pub mod host_metrics;
pub mod io_watcher;
pub mod loop_detector;
pub mod watchdog;
