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
# via a wrapper file
echo "🔨 Generating WASM bindings with wasm-bindgen (target: bundler)..."
cd "$BINDING_DIR"
mkdir -p pkg
$WASM_BINDGEN_CMD \
    --target bundler \
    --out-dir pkg \
    --out-name joy_generator_wasm \
    "$WASM_FILE"

# Create Cloudflare Workers-compatible wrapper file with explicit initWasm helper
echo "📝 Creating Cloudflare Workers-compatible wrapper..."
cat > pkg/joy_generator_wasm.js << 'WRAPPER_EOF'
// Cloudflare Workers compatibility wrapper
// Based on: https://developers.cloudflare.com/workers/languages/rust/#javascript-plumbing-wasm-bindgen
// and https://github.com/wg/cf-worker-wasm
//
// This wrapper handles WebAssembly.Module (Workers) vs raw module (Node.js/bundlers)
// Uses wasm-bindgen --target bundler which provides __wbg_set_wasm

import * as imports from "./joy_generator_wasm_bg.js";

let initPromise;

/**
 * Initialize the underlying WASM module.
 *
 * Safe to call multiple times; initialization only runs once and
 * subsequent calls await the same Promise.
 */
export function initWasm() {
  if (!initPromise) {
    initPromise = (async () => {
      if (typeof process !== "undefined" && process.release?.name === "node") {
        // Node.js environment - read WASM file from disk and instantiate
        const fs = await import("fs");
        const path = await import("path");
        const { fileURLToPath } = await import("url");
        const __filename = fileURLToPath(import.meta.url);
        const __dirname = path.dirname(__filename);
        const wasmPath = path.join(__dirname, "joy_generator_wasm_bg.wasm");
        const wasmBuffer = fs.readFileSync(wasmPath);
        const wasmInstance = await WebAssembly.instantiate(wasmBuffer, {
          "./joy_generator_wasm_bg.js": imports,
        });
        imports.__wbg_set_wasm(wasmInstance.instance.exports);
      } else {
        // Cloudflare Workers / other runtimes - load via fetch/import.meta.url
        const wasmUrl = new URL("./joy_generator_wasm_bg.wasm", import.meta.url);
        const response = await fetch(wasmUrl);
        const wasmBuffer = await response.arrayBuffer();
        const wasmInstance = await WebAssembly.instantiate(wasmBuffer, {
          "./joy_generator_wasm_bg.js": imports,
        });
        imports.__wbg_set_wasm(wasmInstance.instance.exports);
      }
    })();
  }
  return initPromise;
}

// Eagerly start initialization; callers (especially Workers) should still
// explicitly await initWasm() before invoking any exported functions.
initWasm().catch((err) => {
  console.error("Failed to initialize joy_generator_wasm", err);
});

// Re-export everything from the generated bindings
export * from "./joy_generator_wasm_bg.js";
WRAPPER_EOF

echo "✅ Created Cloudflare Workers-compatible wrapper with initWasm() helper"

echo "📝 Generating TypeScript definitions for wrapper..."
cat > pkg/joy_generator_wasm.d.ts << 'WRAPPER_TYPES'
/* tslint:disable */
/* eslint-disable */

export function generate_goodbye(
  language_code: string,
  template_args_json: string,
  use_emojis: boolean,
  timezone: string,
): string;

export function initWasm(): Promise<void>;
WRAPPER_TYPES

# Optimize WASM binary with wasm-opt (if available)
# wasm-opt is a binary tool (part of binaryen), not a library dependency
# Install via: cargo install wasm-opt --version 0.116.1
# Note: wasm-opt is optional but recommended for smaller binary sizes
if command -v wasm-opt >/dev/null 2>&1; then
    echo "⚡ Optimizing WASM binary with wasm-opt..."
    wasm-opt -Os pkg/joy_generator_wasm_bg.wasm -o pkg/joy_generator_wasm_bg.wasm
else
    echo "ℹ️  wasm-opt not found, skipping optimization"
    echo "   Install with: cargo install wasm-opt --version 0.116.1"
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

echo ""
echo "✅ TypeScript/WASM package built successfully"
echo "📦 Version: $VERSION"
echo "📁 Package directory: $BINDING_DIR/pkg"


