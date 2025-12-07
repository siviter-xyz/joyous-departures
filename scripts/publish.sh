#!/bin/bash
# Publish packages to npm and PyPI

set -e

echo "Publishing to npm..."
cd bindings/typescript
pnpm publish
cd ../..

echo "Publishing to PyPI..."
cd bindings/python
maturin publish
cd ../..

echo "Publishing complete!"


