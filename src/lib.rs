use pyo3::prelude::*;

/// Returns a greeting message.
#[pyfunction]
fn hello_world() -> String {
    "Hello from Rust!".to_string()
}

/// A Python module implemented in Rust.
#[pymodule]
fn harper_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    Ok(())
}
