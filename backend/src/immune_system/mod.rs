/* neira:meta
id: NEI-20250215-immune-module
intent: code
summary: Создан модуль immune_system с функцией observe.
*/

use crate::factory::StemCellRecord;
use jsonschema_valid::ValidationError;

pub fn observe(_record: &StemCellRecord) {
    metrics::counter!("immune_observations_total").increment(1);
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
