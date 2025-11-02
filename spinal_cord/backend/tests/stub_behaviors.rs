use backend::{
    AnalysisCell, CellStatus, DiagnosticsCell, DigestivePipeline, MemoryCell, MetricsCollectorCell,
    MetricsRecord, PipelineError, SynapseHub,
};
use serde_json::json;

#[test]
fn digestive_pipeline_exposes_schema_path() {
    let pipeline = DigestivePipeline::new("schemas/training.json");
    assert_eq!(pipeline.schema_path(), "schemas/training.json");
}

#[test]
fn digestive_pipeline_parses_valid_json() -> Result<(), PipelineError> {
    let pipeline = DigestivePipeline::new("schemas/training.json");
    let parsed = pipeline.parse_json(r#"{ "mode": "train" }"#)?;
    assert_eq!(parsed.value["mode"], json!("train"));
    Ok(())
}

#[test]
fn digestive_pipeline_parse_error_mentions_schema() {
    let pipeline = DigestivePipeline::new("schemas/training.json");
    let err = match pipeline.parse_json("{ invalid json ") {
        Ok(_) => panic!("expected parse failure"),
        Err(err) => err,
    };
    let PipelineError::Parse(message) = err;
    assert!(
        message.contains("schemas/training.json"),
        "expected schema path in error, got: {message}"
    );
}

#[test]
fn diagnostics_cell_exposes_id() {
    let cell = DiagnosticsCell::new("diagnostics.alpha");
    assert_eq!(cell.id(), "diagnostics.alpha");
}

#[test]
fn metrics_collector_tracks_records() {
    let mut cell = MetricsCollectorCell::new("metrics.training");
    assert_eq!(cell.id(), "metrics.training");
    assert!(cell.records().is_empty());

    cell.push_record(MetricsRecord {
        timestamp: 42,
        value: 0.75,
    });

    let snapshot = cell.records();
    assert_eq!(snapshot.len(), 1);
    assert_eq!(snapshot[0].timestamp, 42);
    assert_eq!(snapshot[0].value, 0.75);
}

#[test]
fn analysis_cell_cycles_status() {
    let mut cell = AnalysisCell::new("analysis.cell");
    assert_eq!(cell.id(), "analysis.cell");
    assert!(matches!(cell.status(), CellStatus::Ready));

    cell.set_status(CellStatus::Processing);
    assert!(matches!(cell.status(), CellStatus::Processing));

    cell.set_status(CellStatus::Done);
    assert!(matches!(cell.status(), CellStatus::Done));
}

#[test]
fn memory_cell_stores_and_reads_payload() {
    let mut cell = MemoryCell::new("memory.cell");
    assert_eq!(cell.id(), "memory.cell");
    assert!(cell.data().is_none());

    cell.store(json!({ "result": "ok" }));
    let stored = cell.data().expect("value should be stored");
    assert_eq!(stored["result"], json!("ok"));
}

#[test]
fn synapse_hub_registers_connections() {
    let mut hub = SynapseHub::new();
    assert!(hub.connections_from("training").is_none());

    hub.connect("training", "metrics");
    hub.connect("training", "digestive");

    let downstream = hub
        .connections_from("training")
        .expect("connections should be registered");
    assert_eq!(
        downstream,
        &vec!["metrics".to_string(), "digestive".to_string()]
    );
}
