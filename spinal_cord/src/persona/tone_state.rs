/* neira:meta
id: NEI-20280501-120030-tone-state-controller
intent: feature
summary: |-
  Контроллер tone_state переводит способность в stable: хранит текущее
  настроение, применяет обратную связь, публикует события и обновляет метрики.
*/

use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use chrono::Utc;
use serde::Serialize;
use tokio::time::sleep;

use crate::event_bus::{Event, EventBus};
use crate::hearing;

const EPSILON: f32 = 0.01;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ToneMood {
    Neutral,
    Supportive,
    Focused,
}

impl ToneMood {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Supportive => "supportive",
            Self::Focused => "focused",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ToneEventReason {
    Observation,
    Direct,
    Reset,
    Decay,
}

impl ToneEventReason {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Observation => "observation",
            Self::Direct => "direct",
            Self::Reset => "reset",
            Self::Decay => "decay",
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ToneSnapshot {
    pub mood: ToneMood,
    pub intensity: f32,
    pub confidence: f32,
    pub updated_ms: i64,
}

#[derive(Debug, Clone)]
pub enum ToneFeedback {
    Observation { score: f32 },
    Direct {
        mood: ToneMood,
        intensity: f32,
        confidence: f32,
        reason: Option<String>,
    },
    Reset { reason: Option<String> },
    Decay,
}

#[derive(Debug)]
struct ToneInner {
    mood: ToneMood,
    intensity: f32,
    confidence: f32,
    last_update: Instant,
    last_update_ms: i64,
}

pub struct ToneStateController {
    state: RwLock<ToneInner>,
    decay_period: Duration,
    observation_threshold: f32,
    event_bus: Arc<EventBus>,
}

impl ToneStateController {
    pub fn new(
        event_bus: Arc<EventBus>,
        decay_period: Duration,
        observation_threshold: f32,
    ) -> Arc<Self> {
        let period = if decay_period.is_zero() {
            Duration::from_secs(1)
        } else {
            decay_period
        };
        let threshold = observation_threshold.clamp(0.0, 1.0);
        let now_ms = Utc::now().timestamp_millis();
        Arc::new(Self {
            state: RwLock::new(ToneInner {
                mood: ToneMood::Neutral,
                intensity: 0.0,
                confidence: 0.0,
                last_update: Instant::now(),
                last_update_ms: now_ms,
            }),
            decay_period: period,
            observation_threshold: threshold,
            event_bus,
        })
    }

    pub fn spawn_decay_loop(self: &Arc<Self>) {
        let controller = Arc::clone(self);
        tokio::spawn(async move {
            loop {
                sleep(controller.decay_period).await;
                controller.apply_feedback(ToneFeedback::Decay);
            }
        });
    }

    pub fn snapshot(&self) -> ToneSnapshot {
        self.mutate_state(ToneEventReason::Decay, None, None, false, |_, _, _| false)
    }

    pub fn apply_feedback(&self, feedback: ToneFeedback) -> ToneSnapshot {
        match feedback {
            ToneFeedback::Observation { score } => {
                let bounded = score.clamp(-1.0, 1.0);
                metrics::histogram!("persona_tone_observation_score").record(bounded as f64);
                let magnitude = bounded.abs();
                self.mutate_state(
                    ToneEventReason::Observation,
                    None,
                    Some(bounded),
                    true,
                    |state, now, now_ms| {
                        if magnitude < self.observation_threshold {
                            state.last_update = now;
                            state.last_update_ms = now_ms;
                            return false;
                        }
                        let target = if bounded >= 0.0 {
                            ToneMood::Supportive
                        } else {
                            ToneMood::Focused
                        };
                        let new_intensity = ((state.intensity * 0.6) + (magnitude * 0.5))
                            .clamp(0.0, 1.0);
                        let new_confidence =
                            ((state.confidence * 0.5) + (magnitude * 0.5)).clamp(0.0, 1.0);
                        let changed = state.mood != target
                            || (state.intensity - new_intensity).abs() > EPSILON
                            || (state.confidence - new_confidence).abs() > EPSILON;
                        state.mood = if new_intensity < 0.05 {
                            ToneMood::Neutral
                        } else {
                            target
                        };
                        state.intensity = if state.mood == ToneMood::Neutral {
                            0.0
                        } else {
                            new_intensity
                        };
                        state.confidence = if state.mood == ToneMood::Neutral {
                            0.0
                        } else {
                            new_confidence
                        };
                        state.last_update = now;
                        state.last_update_ms = now_ms;
                        changed
                    },
                )
            }
            ToneFeedback::Direct {
                mood,
                intensity,
                confidence,
                reason,
            } => {
                self.mutate_state(
                    ToneEventReason::Direct,
                    reason,
                    None,
                    true,
                    |state, now, now_ms| {
                        let clamped_intensity = intensity.clamp(0.0, 1.0);
                        let clamped_confidence = confidence.clamp(0.0, 1.0);
                        let final_mood = if clamped_intensity < 0.05 {
                            ToneMood::Neutral
                        } else {
                            mood
                        };
                        let changed = state.mood != final_mood
                            || (state.intensity - clamped_intensity).abs() > EPSILON
                            || (state.confidence - clamped_confidence).abs() > EPSILON;
                        state.mood = final_mood;
                        state.intensity = if final_mood == ToneMood::Neutral {
                            0.0
                        } else {
                            clamped_intensity
                        };
                        state.confidence = if final_mood == ToneMood::Neutral {
                            0.0
                        } else {
                            clamped_confidence
                        };
                        state.last_update = now;
                        state.last_update_ms = now_ms;
                        changed
                    },
                )
            }
            ToneFeedback::Reset { reason } => self.mutate_state(
                ToneEventReason::Reset,
                reason,
                None,
                true,
                |state, now, now_ms| {
                    let changed = state.mood != ToneMood::Neutral
                        || state.intensity > EPSILON
                        || state.confidence > EPSILON;
                    state.mood = ToneMood::Neutral;
                    state.intensity = 0.0;
                    state.confidence = 0.0;
                    state.last_update = now;
                    state.last_update_ms = now_ms;
                    changed
                },
            ),
            ToneFeedback::Decay => self.mutate_state(
                ToneEventReason::Decay,
                None,
                None,
                true,
                |_, _, _| false,
            ),
        }
    }

    fn mutate_state<F>(
        &self,
        reason: ToneEventReason,
        note: Option<String>,
        score: Option<f32>,
        count_feedback: bool,
        update: F,
    ) -> ToneSnapshot
    where
        F: FnOnce(&mut ToneInner, Instant, i64) -> bool,
    {
        let now = Instant::now();
        let now_ms = Utc::now().timestamp_millis();
        let mut state = self.state.write().unwrap();
        let previous = ToneSnapshot::from(&*state);
        let mut changed = self.apply_decay_locked(&mut state, now, now_ms);
        changed |= update(&mut state, now, now_ms);
        let snapshot = ToneSnapshot::from(&*state);
        drop(state);
        self.record_metrics(reason, &snapshot, count_feedback && changed);
        if changed {
            self.publish_event(previous, snapshot.clone(), reason, note, score);
        }
        snapshot
    }

    fn apply_decay_locked(
        &self,
        state: &mut ToneInner,
        now: Instant,
        now_ms: i64,
    ) -> bool {
        if state.mood == ToneMood::Neutral {
            state.last_update = now;
            state.last_update_ms = now_ms;
            return false;
        }
        let elapsed = now.saturating_duration_since(state.last_update);
        if elapsed < self.decay_period {
            return false;
        }
        let periods = (elapsed.as_secs_f32() / self.decay_period.as_secs_f32()).floor() as i32;
        if periods <= 0 {
            state.last_update = now;
            state.last_update_ms = now_ms;
            return false;
        }
        let factor = 0.5_f32.powi(periods);
        state.intensity *= factor;
        state.confidence *= factor;
        if state.intensity < 0.05 {
            state.mood = ToneMood::Neutral;
            state.intensity = 0.0;
            state.confidence = 0.0;
        }
        state.last_update = now;
        state.last_update_ms = now_ms;
        true
    }

    fn record_metrics(&self, reason: ToneEventReason, snapshot: &ToneSnapshot, count: bool) {
        metrics::gauge!("persona_tone_intensity").set(snapshot.intensity as f64);
        metrics::gauge!("persona_tone_confidence").set(snapshot.confidence as f64);
        if count {
            metrics::counter!(
                "persona_tone_feedback_total",
                "reason" => reason.as_str()
            )
            .increment(1);
        }
    }

    fn publish_event(
        &self,
        previous: ToneSnapshot,
        current: ToneSnapshot,
        reason: ToneEventReason,
        note: Option<String>,
        score: Option<f32>,
    ) {
        metrics::counter!(
            "persona_tone_transitions_total",
            "from" => previous.mood.as_str(),
            "to" => current.mood.as_str(),
            "reason" => reason.as_str()
        )
        .increment(1);
        hearing::info(&format!(
            "tone_state изменён: from={} to={} intensity={:.2} reason={}",
            previous.mood.as_str(),
            current.mood.as_str(),
            current.intensity,
            reason.as_str()
        ));
        self.event_bus.publish(&ToneStateChanged {
            previous,
            current,
            reason,
            note,
            score,
        });
    }
}

impl From<&ToneInner> for ToneSnapshot {
    fn from(state: &ToneInner) -> Self {
        ToneSnapshot {
            mood: state.mood,
            intensity: state.intensity,
            confidence: state.confidence,
            updated_ms: state.last_update_ms,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToneStateChanged {
    pub previous: ToneSnapshot,
    pub current: ToneSnapshot,
    pub reason: ToneEventReason,
    pub note: Option<String>,
    pub score: Option<f32>,
}

impl Event for ToneStateChanged {
    fn name(&self) -> &str {
        "persona.tone_state.changed"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn data(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "previous": {
                "mood": self.previous.mood.as_str(),
                "intensity": self.previous.intensity,
                "confidence": self.previous.confidence,
                "updated_ms": self.previous.updated_ms,
            },
            "current": {
                "mood": self.current.mood.as_str(),
                "intensity": self.current.intensity,
                "confidence": self.current.confidence,
                "updated_ms": self.current.updated_ms,
            },
            "reason": self.reason.as_str(),
            "note": self.note,
            "score": self.score,
        }))
    }
}
