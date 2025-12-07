#!/bin/bash
# Publish packages to npm and PyPI
# Requires: npm login (or Trusted Publishing), PyPI API token

set -e

echo "📦 Publishing packages..."

# Build and publish npm package
echo "📦 Building and publishing to npm..."
cd bindings/typescript
# Build WASM package first (prepublishOnly will also run, but explicit is better)
npm run build
npm publish --access public
cd ../..

# Build and publish Python package
echo "🐍 Building and publishing to PyPI..."
cd bindings/python
source ~/.cargo/env || true
# Create venv and install dependencies
uv venv --python 3.13 || python3 -m venv .venv
source .venv/bin/activate
uv pip install maturin twine || pip install maturin twine
# Copy README for maturin
cp ../../README.md README.md
# Build package
maturin build --release
# Publish (requires PYPI_API_TOKEN env var or ~/.pypirc)
if [ -z "$PYPI_API_TOKEN" ]; then
  echo "⚠️  PYPI_API_TOKEN not set. Using twine with ~/.pypirc or prompt"
  twine upload target/wheels/*.whl
else
  twine upload target/wheels/*.whl --username __token__ --password "$PYPI_API_TOKEN"
fi
cd ../..

echo "✅ All packages published!"
