pub mod autopilot;
pub mod cell_template;
pub mod server;
pub mod training;

pub use autopilot::AutoPilot;
pub use server::MetricsServer;
pub use training::metrics::LearningMetrics;

pub fn greet() -> &'static str {
    "Hello, Neira!"
}
