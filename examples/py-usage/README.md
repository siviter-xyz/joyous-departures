# Python Usage Example

This example demonstrates how to use `joyous-departures` in a Python project.

## Setup

```bash
# Create a virtual environment
uv venv

# Activate the virtual environment
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install dependencies
uv pip install -e .

# Run the example
python main.py
```

## What it demonstrates

- Basic message generation
- Custom template variables (name, location)
- Emoji control
- Timezone support
- Random message generation

**Note:** This example uses the published PyPI package. If the package isn't published yet, you can install from the local bindings:

```bash
cd ../../bindings/python
maturin develop
cd ../../examples/py-usage
python main.py
```

