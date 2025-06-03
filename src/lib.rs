use pyo3::prelude::*;
use harper_core;

/// Returns the version of harper-core.
#[pyfunction]
fn core_version() -> &'static str {
    harper_core::core_version()
}

/// A Python module implemented in Rust.
#[pymodule]
fn harper_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(core_version, m)?)?;
    Ok(())
}
