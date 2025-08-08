use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct VerificationResult {
    #[pyo3(get)]
    pub claim: String,
    #[pyo3(get)]
    pub verdict: Option<bool>,
    #[pyo3(get)]
    pub confidence: f32,
    #[pyo3(get)]
    pub sources: Vec<String>,
    #[pyo3(get)]
    pub clarifying_questions: Vec<String>,
    #[pyo3(get)]
    pub disclaimer: Option<String>,
}

#[pyfunction]
pub fn verify_claim(claim: &str, context: Vec<&str>) -> VerificationResult {
    let ctx = context.join(" ").to_lowercase();
    let lc_claim = claim.to_lowercase();
    let verdict = if claim.is_empty() { None } else { Some(ctx.contains(&lc_claim)) };
    let confidence = match verdict {
        Some(true) => 1.0,
        Some(false) => 0.0,
        None => 0.0,
    };
    VerificationResult {
        claim: claim.to_string(),
        verdict,
        confidence,
        sources: if verdict.is_some() { vec!["context".to_string()] } else { Vec::new() },
        clarifying_questions: Vec::new(),
        disclaimer: None,
    }
}

