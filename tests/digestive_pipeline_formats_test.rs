/* neira:meta
id: NEI-20260601-digestive-formats-test
intent: test
summary: Проверяет, что DigestivePipeline распознаёт XML и YAML как структуру.
*/
/* neira:meta
id: NEI-20260920-digestive-log-test
intent: test
summary: Проверяет запись лога при ошибке валидации.
*/
use backend::digestive_pipeline::{DigestivePipeline, ParsedInput, PipelineError};
use std::sync::{Arc, Mutex};

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

#[test]
fn logs_validation_failure() {
    struct BufWriter {
        buf: Arc<Mutex<Vec<u8>>>,
    }
    impl std::io::Write for BufWriter {
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            self.buf.lock().unwrap().extend_from_slice(data);
            Ok(data.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let buf = Arc::new(Mutex::new(Vec::new()));
    let subscriber = tracing_subscriber::fmt()
        .with_writer({
            let buf = buf.clone();
            move || BufWriter { buf: buf.clone() }
        })
        .with_ansi(false)
        .with_target(false)
        .finish();
    let _guard = tracing::subscriber::set_default(subscriber);

    let err = DigestivePipeline::ingest("{}").unwrap_err();
    assert!(matches!(err, PipelineError::Validation(_)));

    let contents = String::from_utf8(buf.lock().unwrap().clone()).expect("utf8");
    assert!(
        contents.contains("validation failed"),
        "log missing: {contents}"
    );
}
