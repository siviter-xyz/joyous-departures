#!/bin/bash
# Build all packages (Rust workspace, WASM, Python)

set -e

# Ensure cargo is in PATH
source ~/.cargo/env 2>/dev/null || true

echo "Building Rust workspace..."
cargo build --release

echo "Building WASM package..."
cd bindings/typescript
source ~/.cargo/env 2>/dev/null || true
wasm-pack build --target web --out-dir pkg
cd ../..

echo "Building Python package..."
cd bindings/python
if [ ! -d ".venv" ]; then
    uv venv --python 3.13
fi
source .venv/bin/activate
uv pip install maturin 2>/dev/null || true
maturin build --release
cd ../..

echo "Build complete!"

