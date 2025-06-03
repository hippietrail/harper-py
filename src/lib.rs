use pyo3::prelude::*;
use harper_core;
use harper_core::Document;

/// Returns the version of harper-core.
#[pyfunction]
fn core_version() -> &'static str {
    harper_core::core_version()
}

/// Creates a new Document with pre-configured English language support.
///
/// # Arguments
/// * `text` - The text to create the document from (default: "Hello, world!")
///
/// This creates a document with a curated set of English grammar rules.
#[pyfunction]
#[pyo3(signature = (text = "Hello, world!"))]
fn create_english_document(text: &str) -> PyResult<String> {
    let doc = Document::new_plain_english_curated(text);
    Ok(doc.get_full_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn harper_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(core_version, m)?)?;
    m.add_function(wrap_pyfunction!(create_english_document, m)?)?;
    Ok(())
}
