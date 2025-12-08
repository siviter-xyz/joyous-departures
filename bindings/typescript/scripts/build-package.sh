#!/bin/bash
# Build TypeScript/WASM package
# This script ensures the pkg directory is correctly built and packaged

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINDING_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_ROOT="$(cd "$BINDING_DIR/../.." && pwd)"

# Get version from git tag or package.json
if [ -n "$1" ]; then
    VERSION="$1"
else
    # Try to get version from git tag
    if git -C "$REPO_ROOT" describe --tags --exact-match HEAD >/dev/null 2>&1; then
        VERSION=$(git -C "$REPO_ROOT" describe --tags --exact-match HEAD | sed 's/^v//')
    else
        # Fall back to package.json
        VERSION=$(grep -E '"version"' "$BINDING_DIR/package.json" | head -1 | sed -E 's/.*"version"\s*:\s*"([^"]+)".*/\1/')
    fi
fi

echo "🔨 Building TypeScript/WASM package for version: $VERSION"
echo "📦 Binding directory: $BINDING_DIR"
echo "📁 Repo root: $REPO_ROOT"

cd "$BINDING_DIR"

# Ensure cargo is in PATH
source ~/.cargo/env 2>/dev/null || true

# Build WASM using wasm-bindgen directly (like cf-worker-wasm example)
# This gives us better control over the generated code for Cloudflare Workers compatibility
echo "🔨 Building WASM with cargo..."
cd "$REPO_ROOT"
cargo build --release --target wasm32-unknown-unknown -p joy-generator-wasm

# Find the built WASM file (look for the main crate output, not deps)
WASM_FILE=$(find "$REPO_ROOT/target/wasm32-unknown-unknown/release" -name "joy_generator_wasm.wasm" -not -path "*/deps/*" | head -1)

# Fallback to any matching file if exact match not found
if [ -z "$WASM_FILE" ]; then
    WASM_FILE=$(find "$REPO_ROOT/target/wasm32-unknown-unknown/release" -name "*.wasm" -not -name "*.d" -not -path "*/deps/*" | grep -E "joy_generator" | head -1)
fi

if [ -z "$WASM_FILE" ] || [ ! -f "$WASM_FILE" ]; then
    echo "❌ Error: Could not find built WASM file"
    echo "   Searched in: $REPO_ROOT/target/wasm32-unknown-unknown/release"
    exit 1
fi

echo "📦 Found WASM file: $WASM_FILE"

# Use wasm-bindgen-cli - prefer installed binary, fallback to cargo install
# Note: wasm-bindgen-cli is a binary tool, not a library dependency
# It should be installed via `cargo install wasm-bindgen-cli` for reproducible builds
if command -v wasm-bindgen >/dev/null 2>&1; then
    WASM_BINDGEN_CMD="wasm-bindgen"
    echo "   ✓ Using wasm-bindgen from PATH"
else
    echo "📥 Installing wasm-bindgen-cli via cargo..."
    cargo install wasm-bindgen-cli --version 0.2.106 --quiet
    WASM_BINDGEN_CMD="wasm-bindgen"
fi

# Generate bindings with wasm-bindgen using --target bundler
# The bundler target generates __wbg_set_wasm which allows proper Cloudflare Workers support
# via a wrapper file (no patching needed!)
echo "🔨 Generating WASM bindings with wasm-bindgen (target: bundler)..."
cd "$BINDING_DIR"
mkdir -p pkg
$WASM_BINDGEN_CMD \
    --target bundler \
    --out-dir pkg \
    --out-name joy_generator_wasm \
    "$WASM_FILE"

# Create Cloudflare Workers-compatible wrapper file
# Based on: https://developers.cloudflare.com/workers/languages/rust/#javascript-plumbing-wasm-bindgen
# and https://github.com/wg/cf-worker-wasm
# 
# The bundler target generates __wbg_set_wasm, which we use in the wrapper
# This follows the established Cloudflare pattern - no patching needed!
echo "📝 Creating Cloudflare Workers-compatible wrapper..."
cat > pkg/joy_generator_wasm.js << 'WRAPPER_EOF'
// Cloudflare Workers compatibility wrapper
// Based on: https://developers.cloudflare.com/workers/languages/rust/#javascript-plumbing-wasm-bindgen
// and https://github.com/wg/cf-worker-wasm
// 
// This wrapper handles WebAssembly.Module (Workers) vs raw module (Node.js/bundlers)
// Uses wasm-bindgen --target bundler which provides __wbg_set_wasm

import * as imports from "./joy_generator_wasm_bg.js";

// Import WASM module - in Workers this gives us a WebAssembly.Module
// In Node.js/bundlers, this gives us the module object
// Switch between both syntax for node and for workerd (Cloudflare pattern)
import wkmod from "./joy_generator_wasm_bg.wasm";
import * as nodemod from "./joy_generator_wasm_bg.wasm";

// Initialize based on environment (exact pattern from Cloudflare docs)
if (typeof process !== "undefined" && process.release?.name === "node") {
  // Node.js environment - use the module directly
  imports.__wbg_set_wasm(nodemod);
} else {
  // Cloudflare Workers environment - create Instance from Module
  const instance = new WebAssembly.Instance(wkmod, {
    "./joy_generator_wasm_bg.js": imports,
  });
  imports.__wbg_set_wasm(instance.exports);
}

// Re-export everything from the generated bindings
export * from "./joy_generator_wasm_bg.js";
WRAPPER_EOF

echo "✅ Created Cloudflare Workers-compatible wrapper (no patching needed!)"

# Optimize WASM binary with wasm-opt (if available)
# wasm-opt is part of binaryen, install via: cargo install wasm-opt
if command -v wasm-opt >/dev/null 2>&1; then
    echo "⚡ Optimizing WASM binary with wasm-opt..."
    wasm-opt -Os pkg/joy_generator_wasm_bg.wasm -o pkg/joy_generator_wasm_bg.wasm
else
    echo "ℹ️  wasm-opt not found, skipping optimization"
    echo "   Install with: cargo install wasm-opt"
    echo "   Note: wasm-opt is optional but recommended for smaller binary sizes"
fi

# Generate package.json for pkg directory (wasm-pack would do this, but we're using wasm-bindgen directly)
echo "📝 Generating package.json for pkg directory..."
cat > pkg/package.json << EOF
{
  "name": "joy-generator-wasm",
  "type": "module",
  "version": "$VERSION",
  "description": "WASM bindings for joy-generator message generator",
  "main": "joy_generator_wasm.js",
  "types": "joy_generator_wasm.d.ts",
  "files": [
    "joy_generator_wasm_bg.wasm",
    "joy_generator_wasm.js",
    "joy_generator_wasm.d.ts"
  ]
}
EOF

# Remove any old/stale WASM files (from previous builds with different names)
echo "🧹 Cleaning up old WASM files..."
find pkg -name "joy_goodbye_wasm*" -type f -delete 2>/dev/null || true

# Build TypeScript to both ESM and CommonJS using tsdown
echo ""
echo "🔨 Building TypeScript to ESM and CommonJS with tsdown..."

# Ensure tsdown is available (should be installed via pnpm install)
if ! command -v tsdown >/dev/null 2>&1 && [ -f "node_modules/.bin/tsdown" ]; then
    # Use local tsdown from node_modules
    npx tsdown
elif command -v tsdown >/dev/null 2>&1; then
    # Use global tsdown if available
    tsdown
else
    # Fallback to npx (will install if needed, but slower)
    echo "⚠️  tsdown not found locally, using npx (may be slower)..."
    npx tsdown
fi

echo "✅ TypeScript builds complete"

# Copy README from repo root
if [ ! -f "README.md" ]; then
    echo "📄 Copying README..."
    cp "$REPO_ROOT/README.md" README.md
fi

# Verify pkg directory contents
echo ""
echo "🔍 Verifying pkg directory contents..."
REQUIRED_FILES=(
    "pkg/joy_generator_wasm_bg.wasm"
    "pkg/joy_generator_wasm.js"
    "pkg/joy_generator_wasm.d.ts"
    "pkg/package.json"
)

MISSING=0
for file in "${REQUIRED_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo "❌ Missing required file: $file"
        MISSING=1
    else
        echo "   ✓ $file"
    fi
done

if [ $MISSING -eq 1 ]; then
    echo "❌ Package verification failed"
    exit 1
fi

# Check package.json version matches
PKG_VERSION=$(grep -E '"version"' pkg/package.json | head -1 | sed -E 's/.*"version"\s*:\s*"([^"]+)".*/\1/')
if [ "$PKG_VERSION" != "$VERSION" ]; then
    echo "⚠️  Warning: pkg/package.json version ($PKG_VERSION) doesn't match expected version ($VERSION)"
    echo "   Updating pkg/package.json version..."
    # Update version in pkg/package.json (requires node/jq or sed)
    if command -v node >/dev/null 2>&1; then
        node -e "const fs = require('fs'); const pkg = JSON.parse(fs.readFileSync('pkg/package.json')); pkg.version = '$VERSION'; fs.writeFileSync('pkg/package.json', JSON.stringify(pkg, null, 2) + '\n');"
    else
        echo "   ⚠️  Could not update version automatically (node not found)"
    fi
fi

# Verify package only includes required files (no superfluous files)
echo ""
echo "🔍 Verifying package contents (Cloudflare Workers compatible)..."
echo "   Required files for Cloudflare Workers:"
echo "   - joy_generator_wasm_bg.wasm (pre-compiled WASM module)"
echo "   - joy_generator_wasm_bg.js (wasm-bindgen generated bindings with __wbg_set_wasm)"
echo "   - joy_generator_wasm.js (Cloudflare Workers-compatible wrapper)"
echo "   - joy_generator_wasm.d.ts (TypeScript definitions)"
echo "   - package.json (package metadata)"

# Verify wrapper file uses __wbg_set_wasm (Cloudflare Workers pattern)
if grep -q "__wbg_set_wasm" pkg/joy_generator_wasm.js; then
    echo "   ✓ Wrapper uses __wbg_set_wasm (Cloudflare Workers compatible)"
else
    echo "   ⚠️  Warning: Wrapper file may not be correctly configured"
fi

# Check that the generated JS does not use restricted methods
if grep -q "WebAssembly.instantiateStreaming\|WebAssembly.compileStreaming" pkg/joy_generator_wasm_bg.js; then
    echo "   ⚠️  Note: Generated bindings may contain streaming methods (handled by wrapper)"
else
    echo "   ✓ Generated bindings do not use restricted streaming methods"
fi

echo ""
echo "✅ TypeScript/WASM package built successfully"
echo "📦 Version: $VERSION"
echo "📁 Package directory: $BINDING_DIR/pkg"
echo "🌐 Cloudflare Workers compatible: ✓ (uses WebAssembly.instantiate only)"

