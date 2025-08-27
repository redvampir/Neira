use backend::analysis_node::AnalysisResult;
use serde_json::json;

#[test]
fn analysis_result_serializes_reasoning_chain_and_metrics() {
    let mut result = AnalysisResult::new("example", "output", vec!["step1".into()]);
    result.add_step("step2");
    let value = serde_json::to_value(&result).expect("serialize");

    assert_eq!(value["reasoning_chain"][0]["content"], json!("step1"));
    assert_eq!(value["reasoning_chain"][1]["content"], json!("step2"));
    assert_eq!(value["quality_metrics"]["demand"], json!(2));
    assert_eq!(value["quality_metrics"]["credibility"], json!(1.0));

    // опционально можно проверить, что timestamp присутствует:
    assert!(value["reasoning_chain"][0]["timestamp"].is_string());
    assert!(value["reasoning_chain"][1]["timestamp"].is_string());
}
