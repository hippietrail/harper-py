use pyo3::prelude::*;
use harper_core::Document;
use harper_core::linting::{LintGroup, Linter, Lint, Suggestion, LintKind};
use harper_core::Dialect;
use harper_core::FstDictionary;
use std::sync::Arc;
use parking_lot::Mutex;

/// A minimal Python wrapper around the Harper Document type.
#[pyclass]
struct PyDocument {
    doc: Arc<Mutex<Document>>,
}

/// A Python wrapper around the Harper LintGroup type.
#[pyclass]
struct PyLintGroup {
    lint_group: LintGroup,
}

/// A Python wrapper around the Harper Lint type.
#[pyclass]
struct PyLint {
    lint: Lint,
}

/// A Python wrapper around the Harper Suggestion type.
#[pyclass]
struct PySuggestion {
    suggestion: Suggestion,
}

#[pymethods]
impl PyDocument {
    /// Create a new document with the given text
    #[new]
    fn new(text: &str) -> Self {
        PyDocument {
            doc: Arc::new(Mutex::new(Document::new_plain_english_curated(text))),
        }
    }

    /// Get the full text of the document
    fn get_text(&self) -> String {
        let doc = self.doc.lock();
        doc.get_full_string()
    }

    /// Get the number of tokens in the document
    fn token_count(&self) -> usize {
        let doc = self.doc.lock();
        println!("DEBUG: token_count() called");
        0
    }

    /// Get linting issues from the document using a lint group
    fn get_lints(&self, lint_group: &mut PyLintGroup) -> Vec<PyLint> {
        let doc = self.doc.lock();
        println!("DEBUG: Document created with text: '{}'", doc.get_full_string());
        println!("DEBUG: Getting lints using lint group...");
        
        let lints = lint_group.lint_group.lint(&doc);
        println!("DEBUG: Got {} lints", lints.len());
        
        lints.into_iter().map(|lint| PyLint { lint }).collect()
    }
}

#[pymethods]
impl PyLintGroup {
    /// Create a new curated lint group for American English
    #[new]
    fn new() -> Self {
        let dictionary = FstDictionary::curated();
        let lint_group = LintGroup::new_curated(Arc::new(dictionary), harper_core::Dialect::American);
        PyLintGroup { lint_group }
    }
}

#[pymethods]
impl PySuggestion {
    /// Get the text of this suggestion as a readable string
    fn text(&self) -> String {
        format!("{}", self.suggestion)
    }

    /// Get the suggestion type (ReplaceWith, InsertAfter, Remove)
    fn suggestion_type(&self) -> String {
        match &self.suggestion {
            Suggestion::ReplaceWith(_) => "ReplaceWith".to_string(),
            Suggestion::InsertAfter(_) => "InsertAfter".to_string(),
            Suggestion::Remove => "Remove".to_string(),
        }
    }
}

#[pymethods]
impl PyLint {
    /// Get the message for this lint
    fn message(&self) -> String {
        self.lint.message.to_string()
    }

    /// Get the start position of this lint
    fn start(&self) -> usize {
        self.lint.span.start
    }

    /// Get the end position of this lint
    fn end(&self) -> usize {
        self.lint.span.end
    }

    /// Get the suggestions for this lint
    fn suggestions(&self) -> Vec<PySuggestion> {
        self.lint.suggestions.iter().map(|s| PySuggestion { suggestion: s.clone() }).collect()
    }

    /// Get the lint kind
    fn kind(&self) -> String {
        format!("{:?}", self.lint.lint_kind)
    }

    /// Get the priority
    fn priority(&self) -> u8 {
        self.lint.priority
    }
}

/// Returns the version of harper-core
#[pyfunction]
fn core_version() -> &'static str {
    harper_core::core_version()
}

/// Creates a new Document with pre-configured English language support.
#[pyfunction]
fn create_english_document(text: &str) -> PyDocument {
    PyDocument::new(text)
}

/// Creates a new curated lint group for American English.
#[pyfunction]
fn create_curated_lint_group() -> PyLintGroup {
    PyLintGroup::new()
}

/// A Python module implemented in Rust.
#[pymodule]
fn harper_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(core_version, m)?)?;
    m.add_function(wrap_pyfunction!(create_english_document, m)?)?;
    m.add_function(wrap_pyfunction!(create_curated_lint_group, m)?)?;
    m.add_class::<PyDocument>()?;
    m.add_class::<PyLintGroup>()?;
    m.add_class::<PyLint>()?;
    m.add_class::<PySuggestion>()?;
    Ok(())
}
