#!/bin/bash
# Run all test suites

set -e

# Ensure cargo is in PATH
source ~/.cargo/env 2>/dev/null || true

echo "Running Rust tests..."
cargo test

echo "Running Python E2E tests..."
cd bindings/python
if [ ! -d ".venv" ]; then
    echo "Creating Python virtual environment..."
    uv venv --python 3.13
fi
source .venv/bin/activate
if ! python -c "import joy_goodbye" 2>/dev/null; then
    echo "Building Python extension..."
    uv pip install maturin pytest
    maturin develop
fi
pytest tests/test_e2e.py
cd ../..

echo "Running TypeScript E2E tests..."
cd bindings/typescript
if [ ! -d "pkg" ]; then
    echo "Building WASM package..."
    source ~/.cargo/env 2>/dev/null || true
    wasm-pack build --target web --out-dir pkg
fi
if [ ! -d "node_modules" ]; then
    pnpm install
fi
pnpm test
cd ../..

echo "All tests passed!"

