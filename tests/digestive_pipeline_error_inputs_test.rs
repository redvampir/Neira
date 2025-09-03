/* neira:meta
id: NEI-20261215-digestive-invalid-tests
intent: test
summary: Проверяет ошибки DigestivePipeline при некорректных JSON, YAML, XML и схеме.
*/
use backend::digestive_pipeline::{DigestivePipeline, PipelineError};
use serial_test::serial;
use std::sync::{Arc, Mutex};
use tempfile::tempdir;

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

fn init_logger(buf: Arc<Mutex<Vec<u8>>>) -> tracing::subscriber::DefaultGuard {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_writer({
            let buf = buf.clone();
            move || BufWriter { buf: buf.clone() }
        })
        .with_ansi(false)
        .with_target(false)
        .finish();
    tracing::subscriber::set_default(subscriber)
}

#[test]
#[serial]
fn invalid_json_error_logged() {
    let buf = Arc::new(Mutex::new(Vec::new()));
    let _guard = init_logger(buf.clone());

    let err = DigestivePipeline::ingest("{\"result\":\"ok\",\"metadata\":{\"schema\":\"s\"}}")
        .expect_err("json validation");
    assert!(matches!(err, PipelineError::Validation(_)));

    let contents = String::from_utf8(buf.lock().unwrap().clone()).expect("utf8");
    assert!(
        contents.contains("validation failed"),
        "log missing: {contents}"
    );
}

#[test]
#[serial]
fn invalid_yaml_error_logged() {
    let buf = Arc::new(Mutex::new(Vec::new()));
    let _guard = init_logger(buf.clone());

    let raw = "result: \"ok\"\nmetadata:\n  schema: \"s\"";
    let err = DigestivePipeline::ingest(raw).expect_err("yaml validation");
    assert!(matches!(err, PipelineError::Validation(_)));

    let contents = String::from_utf8(buf.lock().unwrap().clone()).expect("utf8");
    assert!(
        contents.contains("validation failed"),
        "log missing: {contents}"
    );
}

#[test]
#[serial]
fn invalid_xml_error_logged() {
    let buf = Arc::new(Mutex::new(Vec::new()));
    let _guard = init_logger(buf.clone());

    let raw = "<root><result>ok</result><metadata><schema>s</schema></metadata></root>";
    let err = DigestivePipeline::ingest(raw).expect_err("xml validation");
    assert!(matches!(err, PipelineError::Validation(_)));

    let contents = String::from_utf8(buf.lock().unwrap().clone()).expect("utf8");
    assert!(
        contents.contains("validation failed"),
        "log missing: {contents}"
    );
}

#[test]
#[serial]
fn invalid_schema_error_logged() {
    DigestivePipeline::reset_cache();
    let dir = tempdir().expect("dir");
    let schema_path = dir.path().join("schema.json");
    std::fs::write(&schema_path, "not json").expect("schema file");

    let cfg_path = dir.path().join("digestive.toml");
    std::fs::write(
        &cfg_path,
        format!("schema_path = \"{}\"", schema_path.display()),
    )
    .expect("config file");
    std::env::set_var("DIGESTIVE_CONFIG", &cfg_path);

    let buf = Arc::new(Mutex::new(Vec::new()));
    let _guard = init_logger(buf.clone());

    let raw = "{\"id\":\"1\",\"result\":\"ok\",\"metadata\":{\"schema\":\"s\"}}";
    let err = DigestivePipeline::ingest(raw).expect_err("schema load");
    assert!(matches!(err, PipelineError::Schema(_)));

    let contents = String::from_utf8(buf.lock().unwrap().clone()).expect("utf8");
    assert!(
        contents.contains("invalid schema"),
        "log missing: {contents}"
    );

    std::env::remove_var("DIGESTIVE_CONFIG");
    DigestivePipeline::reset_cache();
}
