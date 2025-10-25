/* neira:meta
id: NEI-20280501-120200-tone-state-tests
intent: chore
summary: Добавлены юнит-тесты контроллера тонов с проверкой метрик и событий.
*/

use std::sync::{Arc, Mutex};
use std::time::Duration;

use backend::event_bus::{Event, EventBus, Subscriber};
use backend::persona::tone_state::{
    ToneEventReason, ToneFeedback, ToneMood, ToneStateChanged, ToneStateController,
};

mod common;
use common::init_recorder;

struct ToneCollector {
    events: Arc<Mutex<Vec<ToneStateChanged>>>,
}

impl Subscriber for ToneCollector {
    fn on_event(&self, event: &dyn Event) {
        if let Some(changed) = event.as_any().downcast_ref::<ToneStateChanged>() {
            self.events.lock().unwrap().push(changed.clone());
        }
    }
}

#[tokio::test]
async fn tone_state_observation_changes_and_decays() {
    let metrics = init_recorder();
    let bus = EventBus::new();
    let events = Arc::new(Mutex::new(Vec::new()));
    bus.subscribe(Arc::new(ToneCollector {
        events: events.clone(),
    }));
    let controller = ToneStateController::new(bus, Duration::from_millis(50), 0.2);
    controller.spawn_decay_loop();

    let first = controller.apply_feedback(ToneFeedback::Observation { score: 0.9 });
    assert_eq!(first.mood, ToneMood::Supportive);
    assert!(first.intensity > 0.4);

    tokio::time::sleep(Duration::from_millis(140)).await;

    let decayed = controller.snapshot();
    assert!(decayed.intensity < first.intensity);

    let collected = events.lock().unwrap();
    assert!(
        collected
            .iter()
            .any(|evt| evt.reason == ToneEventReason::Observation),
        "observation event missing"
    );
    assert!(
        collected
            .iter()
            .any(|evt| evt.reason == ToneEventReason::Decay),
        "decay event missing"
    );
    drop(collected);

    let metrics_data = metrics.lock().unwrap();
    assert!(
        metrics_data
            .iter()
            .any(|(name, _)| name == "persona_tone_feedback_total"),
        "feedback counter not recorded"
    );
}
