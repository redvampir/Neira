use backend::digestive_pipeline::DigestivePipeline;
use metrics_exporter_prometheus::PrometheusBuilder;
use serial_test::serial;

/* neira:meta
id: NEI-20261005-digestive-metrics-test
intent: test
summary: Проверяет запись метрик времени DigestivePipeline.
*/

#[test]
#[serial]
fn records_parse_and_validation_metrics() {
    let handle = PrometheusBuilder::new().install_recorder().unwrap();
    let json = r#"{"id":"1","result":"ok","metadata":{"schema":"s"}}"#;
    DigestivePipeline::ingest(json).expect("digest");
    let metrics = handle.render();
    assert!(metrics.contains("digestive_parse_duration_ms"));
    assert!(metrics.contains("digestive_validation_duration_ms"));
}
