/* neira:meta
id: NEI-20270318-120080-training-module
intent: feature
summary: |-
  Добавлен модуль training: экспортируется оркестратор автоматизированных
  циклов обучения и вспомогательная логика.
*/
pub mod curriculum;
pub mod metrics;
pub mod orchestrator;
pub mod scheduler;
pub mod types;
