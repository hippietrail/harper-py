# harper-py

Python bindings for the Harper grammar checker library.

## Status

Basic functionality implemented - Document creation, linting, and suggestions are working.

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/harper-py.git
cd harper-py

# Set up a virtual environment (recommended)
python -m venv venv
source venv/bin/activate  # On Windows use `venv\Scripts\activate`

# Install the package in development mode
maturin develop
```

## Usage

### Command Line Interface

The easiest way to use harper-py is through the CLI:

```bash
# Basic text processing
python cli.py "Your text here"

# Check for grammar and style issues
python cli.py --lint "I can't find the .exe for this program."
```

Example output:
```
Found 3 issue(s):

1. Capitalization (31) : This sentence does not start with a capital letter
   Line 1: I can't find the .exe for this program.
           ^
   Suggestions:
     - Replace with: "Exe"

2. Formatting (63) : Unnecessary space at the end of the sentence.
   Line 1: I can't find the .exe for this program.
                                                      ^
   Suggestions:
     - Remove error

3. Spelling (63) : Did you mean to spell `exe` this way?
   Line 1: I can't find the .exe for this program.
                    ^^^
   Suggestions:
     - Replace with: "eye"
     - Replace with: "ere"
     - Replace with: "eve"
```

### Python API

```python
import harper_py

# Get the version of the underlying harper-core library
version = harper_py.core_version()
print(f"Harper Core Version: {version}")

# Create a new English document
doc = harper_py.create_english_document("This is a test sentence.")
print(f"Document text: {doc.get_text()}")
print(f"Token count: {doc.token_count()}")

# Check for linting issues
lint_group = harper_py.create_curated_lint_group()
lints = doc.get_lints(lint_group)

for lint in lints:
    print(f"Issue: {lint.message()}")
    print(f"Kind: {lint.kind()}")
    print(f"Priority: {lint.priority()}")
    print(f"Position: {lint.start()}-{lint.end()}")
    print("Suggestions:")
    for suggestion in lint.suggestions():
        print(f"  - {suggestion.text()}")
```

## API Reference

### Functions
- `core_version() -> str`: Returns the version of the harper-core library
- `create_english_document(text: str) -> PyDocument`: Creates a new document with pre-configured English language support.
- `create_curated_lint_group() -> PyLintGroup`: Creates a lint group with curated rules.

### PyDocument Class
- `PyDocument(text: str)`: Create a new document with the given text
- `get_text() -> str`: Get the full text of the document
- `token_count() -> int`: Get the number of tokens in the document
- `get_lints(lint_group: PyLintGroup) -> list[PyLint]`: Get linting issues for the document

### PyLint Class
- `message() -> str`: Get the lint message
- `kind() -> str`: Get the type of lint (e.g., "Spelling", "Capitalization")
- `priority() -> int`: Get the priority level (lower numbers = higher priority)
- `start() -> int`: Get the start position of the issue in the text
- `end() -> int`: Get the end position of the issue in the text
- `suggestions() -> list[PySuggestion]`: Get suggested fixes

### PySuggestion Class
- `text() -> str`: Get the suggestion as a readable string (e.g., "Replace with: 'word'")

## Development

This project uses [maturin](https://github.com/PyO3/maturin) for building Python extensions in Rust.

### Prerequisites

- Rust (install via [rustup](https://rustup.rs/))
- Python 3.7+
- maturin (`pip install maturin`)

### Building and Testing

```bash
# Build and install in development mode
maturin develop

# Run tests
python test.py

# Test the CLI
python cli.py --lint "Test text with issues."
```

### Project Structure

- `src/lib.rs`: Rust implementation with PyO3 bindings
- `cli.py`: Command-line interface for testing and usage
- `test.py`: Basic tests for the Python API
- `Cargo.toml`: Rust dependencies and build configuration
- `pyproject.toml`: Python package configuration

## License

MIT
