use backend::analysis_node::AnalysisResult;
use serde_json::json;

#[test]
fn analysis_result_serializes_reasoning_chain_and_metrics() {
    let result = AnalysisResult::new("example", "output", vec!["step1".into(), "step2".into()]);
    let value = serde_json::to_value(&result).expect("serialize");
    assert_eq!(value["reasoning_chain"], json!(["step1", "step2"]));
    assert_eq!(value["quality_metrics"]["demand"], json!(2));
    assert_eq!(value["quality_metrics"]["credibility"], json!(1.0));
}
