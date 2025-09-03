// neira:meta
// id: NEI-20270420-trybuild-unknown-field
// intent: test
// summary: Лишнее поле в DigestiveSettings вызывает ошибку компиляции.
use backend::digestive_pipeline::DigestiveSettings;
use std::path::PathBuf;

fn main() {
    let _cfg = DigestiveSettings { schema_path: PathBuf::from("schema.json"), extra: 1 };
}
