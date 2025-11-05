/* neira:meta
id: NEI-20260131-consciousness-module-v2
intent: feature
summary: |
  Модуль consciousness — Фаза 3 "пробуждения" Нейры (саморефлексия и автономное улучшение).
  
  Компоненты (текущие):
  - MetaCognition: анализ собственных мыслительных процессов, обнаружение bias, генерация improvement tasks
  
  Запланированные компоненты:
  - PersonalityEvolution: отслеживание изменений личности и ценностей
  - DailyGrowthReport: ежедневные отчёты о прогрессе
  - Auto-Improvement Loop: фоновая система выполнения задач самоулучшения
*/

pub mod metacognition;
pub mod auto_improvement;
pub mod personality_evolution;
pub mod daily_report;

pub use metacognition::{
    MetaCognitionEngine, ThoughtTrace, Decision, BiasDetection, BiasType,
    BiasSeverity, ImprovementTask, TaskType, TaskStatus, Priority,
    MetaCognitionStats,
};

pub use auto_improvement::AutoImprovementLoop;

pub use personality_evolution::{
    PersonalityEvolutionTracker, PersonalityTrait, PersonalitySnapshot,
    TraitEvolution, TraitSnapshot, CoreValue, PersonalityChangeAnalysis,
    TraitChange, EvolutionStats,
};

pub use daily_report::DailyGrowthReporter;
