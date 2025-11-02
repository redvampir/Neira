/* neira:meta
id: NEI-20280501-120000-persona-module
intent: feature
summary: Экспортирован модуль эмоциональных состояний личности (tone state).
*/
/* neira:meta
id: NEI-20280502-120100-persona-interaction-verbs
intent: feature
summary: Экспортирован детектор глаголов взаимодействия и событие наблюдения.
*/

pub mod interaction_verbs;
pub mod tone_state;

pub use interaction_verbs::{
    InteractionVerb, InteractionVerbActor, InteractionVerbDetector, InteractionVerbObserved,
};
pub use tone_state::{
    ToneEventReason, ToneFeedback, ToneMood, ToneSnapshot, ToneStateChanged, ToneStateController,
};
