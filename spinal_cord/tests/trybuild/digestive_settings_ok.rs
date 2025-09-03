// neira:meta
// id: NEI-20270420-trybuild-ok
// intent: test
// summary: Валидный DigestiveSettings компилируется.
use backend::digestive_pipeline::DigestiveSettings;
use std::path::PathBuf;

fn main() {
    let _cfg = DigestiveSettings { schema_path: PathBuf::from("schema.json") };
}
