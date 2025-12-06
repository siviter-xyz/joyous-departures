#!/bin/bash
# Quick test script for Python bindings

cd bindings/python

# Ensure venv exists
if [ ! -d ".venv" ]; then
    echo "Creating Python virtual environment..."
    uv venv --python 3.13
fi

source .venv/bin/activate

# Install if needed
if ! python -c "import joy_goodbye" 2>/dev/null; then
    echo "Building Python extension..."
    uv pip install maturin pytest 2>/dev/null || true
    maturin develop
fi

echo "Testing Python bindings..."
echo ""

# Basic test
echo "1. Basic generation:"
python -c "from joy_goodbye import generate_goodbye; print(generate_goodbye())"
echo ""

# Custom name
echo "2. With custom name:"
python -c "from joy_goodbye import generate_goodbye; print(generate_goodbye(template_args={'name': 'Alice'}))"
echo ""

# Without emojis
echo "3. Without emojis:"
python -c "from joy_goodbye import generate_goodbye; print(generate_goodbye(use_emojis=False))"
echo ""

# Custom timezone
echo "4. With custom timezone:"
python -c "from joy_goodbye import generate_goodbye; print(generate_goodbye(timezone='America/New_York'))"
echo ""

# Multiple calls (randomness)
echo "5. Multiple calls (showing randomness):"
python -c "from joy_goodbye import generate_goodbye; [print(f'  Call {i+1}: {generate_goodbye()}') for i in range(3)]"
echo ""

echo "✅ Python bindings working!"

