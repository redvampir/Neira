use pyo3::prelude::*;

#[pyfunction]
fn ping() -> PyResult<&'static str> {
    Ok("pong")
}

#[pymodule]
fn neira_rust(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ping, m)?)?;
    Ok(())
}
