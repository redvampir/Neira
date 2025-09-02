/* neira:meta
id: NEI-20250215-immune-module
intent: code
summary: Создан модуль immune_system с функцией observe.
*/

use crate::factory::StemCellRecord;

pub fn observe(_record: &StemCellRecord) {
    metrics::counter!("immune_observations_total").increment(1);
}
