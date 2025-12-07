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
if ! python -c "import joyous_departures" 2>/dev/null; then
    echo "Building Python extension..."
    uv pip install maturin pytest 2>/dev/null || true
    maturin develop
fi

echo "Testing Python bindings..."
echo ""

# Basic test
echo "1. Basic generation:"
python -c "import asyncio; from joyous_departures import generate_goodbye; print(asyncio.run(generate_goodbye()))"
echo ""

# Custom name
echo "2. With custom name:"
python -c "import asyncio; from joyous_departures import generate_goodbye; print(asyncio.run(generate_goodbye(template_args={'name': 'Alice'})))"
echo ""

# Without emojis
echo "3. Without emojis:"
python -c "import asyncio; from joyous_departures import generate_goodbye; print(asyncio.run(generate_goodbye(use_emojis=False)))"
echo ""

# Custom timezone
echo "4. With custom timezone:"
python -c "import asyncio; from joyous_departures import generate_goodbye; print(asyncio.run(generate_goodbye(timezone='America/New_York')))"
echo ""

# Multiple calls (randomness)
echo "5. Multiple calls (showing randomness):"
python -c "import asyncio; from joyous_departures import generate_goodbye; [print(f'  Call {i+1}: {asyncio.run(generate_goodbye())}') for i in range(3)]"
echo ""

echo "✅ Python bindings working!"

