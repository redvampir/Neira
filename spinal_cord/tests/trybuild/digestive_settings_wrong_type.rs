// neira:meta
// id: NEI-20270420-trybuild-wrong-type
// intent: test
// summary: Неверный тип поля schema_path вызывает ошибку компиляции.
use backend::digestive_pipeline::DigestiveSettings;

fn main() {
    let _cfg = DigestiveSettings { schema_path: 123 };
}
