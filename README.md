# Harper Python Bindings

Python bindings for the `harper-core` Rust library, providing access to Harper's core functionality from Python.

## Prerequisites

- Python 3.8 or higher
- Rust toolchain (install via [rustup](https://rustup.rs/))
- `maturin` (Python package, will be installed automatically if not present)

## Installation

1. Clone this repository
2. Install in development mode:
   ```bash
   pip install -e .
   ```

## Usage

```python
import harper_py

# Create a new document
doc = harper_py.get_document("Hello, world!")
print(doc)  # Prints the document representation
```

## Development

To build the extension:

```bash
maturin develop  # or 'maturin develop --release' for optimized build
```

## License

MIT
