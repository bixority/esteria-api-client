use pyo3::prelude::*;

/// Simple add function
#[allow(clippy::unnecessary_wraps)]
#[pyfunction]
fn add(a: i32, b: i32) -> PyResult<i32> {
    Ok(a + b)
}

/// Module definition

#[pymodule]
fn esteria_api_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
