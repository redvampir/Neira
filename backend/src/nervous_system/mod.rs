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
}

pub mod base_path_resolver;
pub mod host_metrics;
pub mod io_watcher;
