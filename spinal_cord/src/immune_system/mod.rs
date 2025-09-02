/* neira:meta
id: NEI-20250215-immune-module
intent: code
summary: Создан модуль immune_system с функцией observe.
*/

use crate::event_bus::{CellCreated, Event, OrganBuilt, Subscriber};
use crate::factory::StemCellRecord;
use jsonschema_valid::ValidationError;

pub fn observe(_record: &StemCellRecord) {
    metrics::counter!("immune_observations_total").increment(1);
}

/* neira:meta
id: NEI-20250720-immune-alert-handler
intent: code
summary: Добавлен обработчик alert для регистрации алертов иммунной системы.
*/
pub fn alert(severity: &str) {
    metrics::counter!("immune_alerts_total", "severity" => severity.to_string()).increment(1);
}

/* neira:meta
id: NEI-20251227-immune-subscriber
intent: code
summary: Подписчик immune_system на события CellCreated и OrganBuilt.
*/
pub struct ImmuneSystemSubscriber;

impl Subscriber for ImmuneSystemSubscriber {
    fn on_event(&self, event: &dyn Event) {
        if let Some(ev) = event.as_any().downcast_ref::<CellCreated>() {
            observe(&ev.record);
        } else if event.as_any().downcast_ref::<OrganBuilt>().is_some() {
            metrics::counter!("immune_organs_total").increment(1);
        }
    }
}

/* neira:meta
id: NEI-20260514-preflight-check
intent: code
summary: Добавлена заглушка preflight_check для валидации записей.
*/
pub fn preflight_check(record: &StemCellRecord) -> Result<(), ValidationError> {
    metrics::counter!("immune_preflight_checks_total").increment(1);
    if record.id.is_empty() || record.backend.is_empty() || record.template_id.is_empty() {
        return Err(ValidationError::new(
            "record fields must not be empty",
            None,
            None,
        ));
    }
    Ok(())
}
