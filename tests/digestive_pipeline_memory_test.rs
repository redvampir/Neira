/* neira:meta
id: NEI-20261124-digestive-memory-test
intent: test
summary: Проверяет, что DigestivePipeline сохраняет распарсенный ввод в MemoryCell.
*/
use backend::digestive_pipeline::{DigestivePipeline, ParsedInput};
use backend::memory_cell::MemoryCell;
use serde_json::json;
use std::sync::Arc;

#[test]
fn stores_parsed_input_in_memory() {
    let memory = Arc::new(MemoryCell::new());
    DigestivePipeline::set_memory(memory.clone());
    let raw = r#"{"id":"1","result":"ok","metadata":{"schema":"s"}}"#;
    let parsed = DigestivePipeline::ingest(raw).expect("parse");
    assert_eq!(
        parsed,
        ParsedInput::Json(json!({"id":"1","result":"ok","metadata":{"schema":"s"}}))
    );
    assert_eq!(memory.parsed_inputs(), vec![parsed]);
}
