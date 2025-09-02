/* neira:meta
id: NEI-20260514-preflight-tests
intent: test
summary: Проверяет валидацию StemCellRecord через preflight_check.
*/

use backend::factory::{StemCellRecord, StemCellState};
use backend::immune_system::preflight_check;
use chrono::Utc;

#[test]
fn preflight_check_valid_record() {
    let record = StemCellRecord {
        id: "b:tpl".to_string(),
        backend: "b".to_string(),
        template_id: "tpl".to_string(),
        state: StemCellState::Draft,
        created_at: Utc::now(),
    };
    assert!(preflight_check(&record).is_ok());
}

#[test]
fn preflight_check_invalid_record() {
    let record = StemCellRecord {
        id: String::new(),
        backend: String::new(),
        template_id: String::new(),
        state: StemCellState::Draft,
        created_at: Utc::now(),
    };
    assert!(preflight_check(&record).is_err());
}
