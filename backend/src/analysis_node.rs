use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NodeStatus {
    Draft,
    Active,
    Deprecated,
    Error,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct QualityMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credibility: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recency_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demand: Option<u32>,
}

impl QualityMetrics {
    pub fn compute(reasoning_chain: &[String]) -> Self {
        let credibility = if reasoning_chain.is_empty() { 0.0 } else { 1.0 };
        let demand = reasoning_chain.len() as u32;
        QualityMetrics {
            credibility: Some(credibility),
            recency_days: Some(0),
            demand: Some(demand),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalysisMetadata {
    pub schema: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AnalysisResult {
    pub id: String,
    pub output: String,
    pub status: NodeStatus,
    pub quality_metrics: QualityMetrics,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reasoning_chain: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncertainty_score: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<String>,
    pub metadata: AnalysisMetadata,
}

impl AnalysisResult {
    pub fn new(id: impl Into<String>, output: impl Into<String>, reasoning_chain: Vec<String>) -> Self {
        let quality_metrics = QualityMetrics::compute(&reasoning_chain);
        let uncertainty_score = quality_metrics.credibility.map(|c| 1.0 - c);
        AnalysisResult {
            id: id.into(),
            output: output.into(),
            status: NodeStatus::Active,
            quality_metrics,
            reasoning_chain,
            uncertainty_score,
            explanation: None,
            links: vec![],
            metadata: AnalysisMetadata {
                schema: "1.0.0".into(),
            },
        }
    }
}

pub trait AnalysisNode {
    fn id(&self) -> &str;
    fn analysis_type(&self) -> &str;
    fn status(&self) -> NodeStatus;
    fn links(&self) -> &[String];
    fn confidence_threshold(&self) -> f32;
    fn analyze(&self, input: &str) -> AnalysisResult;
    fn explain(&self) -> String;
}
