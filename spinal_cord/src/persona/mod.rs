/* neira:meta
id: NEI-20280501-120000-persona-module
intent: feature
summary: Экспортирован модуль эмоциональных состояний личности (tone state).
*/

pub mod tone_state;

pub use tone_state::{
    ToneEventReason, ToneFeedback, ToneMood, ToneSnapshot, ToneStateChanged, ToneStateController,
};
