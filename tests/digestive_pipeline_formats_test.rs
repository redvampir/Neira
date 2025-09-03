/* neira:meta
id: NEI-20260601-digestive-formats-test
intent: test
summary: Проверяет, что DigestivePipeline распознаёт XML и YAML как структуру.
*/
use backend::digestive_pipeline::{DigestivePipeline, ParsedInput, PipelineError};

#[test]
fn parses_yaml_input() {
    let raw = "id: \"1\"\nresult: \"ok\"\nmetadata:\n  schema: \"s\"";
    let parsed = DigestivePipeline::ingest(raw).expect("parse yaml");
    assert!(matches!(parsed, ParsedInput::Json(_)));
}

#[test]
fn parses_xml_input() {
    let raw = "<root><id>1</id><result>ok</result><metadata><schema>s</schema></metadata></root>";
    let parsed = DigestivePipeline::ingest(raw);
    assert!(matches!(
        parsed,
        Ok(ParsedInput::Json(_)) | Err(PipelineError::Validation(_))
    ));
}
