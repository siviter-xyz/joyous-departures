#!/bin/bash
# Quick test script for TypeScript/WASM bindings

cd bindings/typescript

# Ensure WASM is built
if [ ! -d "pkg" ]; then
    echo "Building WASM package..."
    source ~/.cargo/env 2>/dev/null || true
    wasm-pack build --target web --out-dir pkg
fi

# Ensure dependencies are installed
if [ ! -d "node_modules" ]; then
    echo "Installing dependencies..."
    pnpm install
fi

echo "Testing TypeScript/WASM bindings..."
echo ""

# Try with tsx first (best option)
if command -v tsx &> /dev/null || pnpm exec tsx --version &> /dev/null; then
    pnpm exec tsx test-cli.mjs
elif command -v node &> /dev/null; then
    # Fallback: use vitest to run a simple test
    pnpm test --run 2>&1 | grep -A 2 "should generate a basic" | head -3 || {
        echo "Running via test suite..."
        pnpm test 2>&1 | tail -5
    }
else
    echo "Run tests with: cd bindings/typescript && pnpm test"
fi

