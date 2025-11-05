/* neira:meta
id: NEI-20251103-dialogue-module
intent: feature
summary: |
  Модуль dialogue — эволюционирующая диалоговая система с памятью и саморазвитием.
  Интеграция с SemanticMemory для обучения на каждом диалоге.
*/

pub mod evolving_dialogue;

pub use evolving_dialogue::{
    EvolvingDialogue, UserInput, DialogueResponse, GrowthStats,
    GrowthTracker, ReflectionEntry, SelfAssessment, SkillLevel,
};
