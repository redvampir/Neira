use backend::memory_node::MemoryNode;

mod common;
use common::init_recorder;

#[test]
fn memory_node_records_preload_metric() {
    let data = init_recorder();
    let mem = MemoryNode::new();
    mem.preload_by_trigger(&["test".into()]);
    let records = data.lock().unwrap();
    assert!(
        records
            .iter()
            .any(|(n, _)| n == "memory_node_preload_duration_ms"),
        "no histogram recorded"
    );
    assert!(
        records
            .iter()
            .any(|(n, _)| n == "memory_node_preload_duration_ms_p95"),
        "no p95 histogram recorded"
    );
    assert!(
        records
            .iter()
            .any(|(n, _)| n == "memory_node_preload_duration_ms_p99"),
        "no p99 histogram recorded"
    );
}
