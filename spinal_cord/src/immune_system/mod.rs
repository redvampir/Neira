/* neira:meta
id: NEI-20250215-immune-module
intent: code
summary: Создан модуль immune_system с функцией observe.
*/

use crate::cell_template::load_schema_from;
use crate::config::env_flag;
use crate::event_bus::{
    CellCreated, Event, EventBus, LymphaticDecision, LymphaticDuplicateFound, OrganBuilt,
    Subscriber,
};
use crate::factory::StemCellRecord;
use jsonschema_valid::{Config, ValidationError};
use once_cell::sync::Lazy;
use serde_json::json;
use std::path::PathBuf;

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
pub mod lymphatic_filter;

pub struct ImmuneSystemSubscriber {
    bus: std::sync::Arc<EventBus>,
}

impl ImmuneSystemSubscriber {
    pub fn new(bus: std::sync::Arc<EventBus>) -> Self {
        Self { bus }
    }

    fn scan_and_emit(&self) {
        for report in lymphatic_filter::scan_workspace() {
            let decision = if report.similarity > 0.9 {
                LymphaticDecision::Remove
            } else {
                LymphaticDecision::Keep
            };
            metrics::counter!("lymphatic_duplicates_found_total").increment(1);
            if decision == LymphaticDecision::Remove {
                metrics::counter!("lymphatic_artifacts_removed_total").increment(1);
            }
            let ev = LymphaticDuplicateFound {
                gene_id: report.gene_id.clone(),
                location: report.file.clone(),
                similarity: report.similarity,
                decision,
            };
            self.bus.publish(&ev);
        }
    }
}

impl Subscriber for ImmuneSystemSubscriber {
    fn on_event(&self, event: &dyn Event) {
        if let Some(ev) = event.as_any().downcast_ref::<CellCreated>() {
            observe(&ev.record);
            if env_flag("LYMPHATIC_FILTER_ENABLED", true) {
                self.scan_and_emit();
            }
        } else if event.as_any().downcast_ref::<OrganBuilt>().is_some() {
            metrics::counter!("immune_organs_total").increment(1);
            if env_flag("LYMPHATIC_FILTER_ENABLED", true) {
                self.scan_and_emit();
            }
        }
    }
}

/* neira:meta
id: NEI-20260514-preflight-check
intent: code
summary: Добавлена заглушка preflight_check для валидации записей.
*/
#[allow(clippy::result_large_err)]
pub fn preflight_check(record: &StemCellRecord) -> Result<(), ValidationError> {
    metrics::counter!("immune_preflight_checks_total").increment(1);
    let cfg = STEM_CELL_SCHEMA
        .as_ref()
        .map_err(|e| ValidationError::new(e, None, None))?;
    let value = json!({
        "id": &record.id,
        "backend": &record.backend,
        "template_id": &record.template_id,
        "state": format!("{:?}", record.state),
        "created_at": record.created_at.to_rfc3339(),
    });
    if let Err(errors) = cfg.validate(&value) {
        let msg = errors
            .map(|err| err.to_string())
            .collect::<Vec<_>>()
            .join("; ");
        return Err(ValidationError::new(&msg, Some(&value), None));
    }
    Ok(())
}

static STEM_CELL_SCHEMA: Lazy<Result<Config<'static>, String>> = Lazy::new(|| {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../schemas/stem_cell_record.json");
    load_schema_from(&path)
});

/* neira:meta
id: NEI-20240513-immune-lint
intent: chore
summary: Подавлено предупреждение result_large_err для preflight_check.
*/
