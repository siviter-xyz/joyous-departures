#!/bin/bash
# Run the same linting checks as CI
set -e

echo "🔍 Running Rust linting checks (same as CI)..."
echo ""

# Format check
echo "📝 Checking code formatting..."
cargo fmt --check --all
echo "✅ Formatting OK"
echo ""

# Clippy with same flags as CI
echo "🔧 Running clippy (with -D warnings, same as CI)..."
cargo clippy --all-targets --all-features -- -D warnings
echo "✅ Clippy OK"
echo ""

echo "✅ All linting checks passed!"

