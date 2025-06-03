# harper-py

Python bindings for the Harper grammar checker library.

## Status

Early development - Currently the bare minimum functionality.

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/harper-py.git
cd harper-py

# Set up a virtual environment (recommended)
python -m venv venv
source venv/bin/activate  # On Windows use `venv\Scripts\activate`

# Install the package in development mode
pip install -e .
```

## Usage

```python
import harper_py

# Get the version of the underlying harper-core library
version = harper_py.core_version()
print(f"Harper Core Version: {version}")

# Create a new English document
document_text = harper_py.create_english_document()
print(f"Document content: {document_text}")
```

# Currently available functions:

# - `core_version()`: Returns the version of the harper-core library
# - `create_english_document(text: str = "Hello, world!")`: Creates a new document with pre-configured English language support.
  - `text`: Optional input text (default: "Hello, world!")
  - Returns: The processed document content as a string

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
```

## License

MIT
