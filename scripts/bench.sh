#!/bin/bash
# Run benchmarks and generate report

set -e

echo "Running Rust benchmarks..."
cargo bench

echo "Running Python benchmarks..."
cd bindings/python
uv run pytest tests/ -k benchmark
cd ../..

echo "Running TypeScript benchmarks..."
cd bindings/typescript
pnpm test -- --bench
cd ../..

echo "Benchmarks complete!"


