use pyo3::prelude::*;

mod tag_processor;
mod memory_index;
mod knowledge_graph;
mod verification;
mod event_bus;

#[pyfunction]
fn ping() -> PyResult<&'static str> {
    Ok("pong")
}

#[pymodule]
fn neira_rust(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ping, m)?)?;
    m.add_class::<tag_processor::Tag>()?;
    m.add_function(wrap_pyfunction!(tag_processor::parse, m)?)?;
    m.add_function(wrap_pyfunction!(tag_processor::suggest_entities, m)?)?;
    m.add_class::<memory_index::MemoryIndex>()?;
    m.add_class::<knowledge_graph::KnowledgeGraph>()?;
    m.add_class::<verification::VerificationResult>()?;
    m.add_function(wrap_pyfunction!(verification::verify_claim, m)?)?;
    m.add_class::<event_bus::Event>()?;
    m.add_class::<event_bus::EventBus>()?;
    Ok(())
}

