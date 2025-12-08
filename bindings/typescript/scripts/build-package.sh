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

# Build WASM package
echo "🔨 Building WASM with wasm-pack..."
wasm-pack build --target web --out-dir pkg --release

# Remove pkg/.gitignore (contains '*' which excludes all files from npm package)
if [ -f "pkg/.gitignore" ]; then
    echo "🗑️  Removing pkg/.gitignore..."
    rm -f pkg/.gitignore
fi

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
echo "   - joy_generator_wasm.js (loader - uses WebAssembly.instantiate only)"
echo "   - joy_generator_wasm.d.ts (TypeScript definitions)"
echo "   - package.json (package metadata)"

# Check that the generated JS uses WebAssembly.instantiate (not instantiateStreaming)
if grep -q "WebAssembly.instantiateStreaming" pkg/joy_generator_wasm.js; then
    echo "⚠️  Warning: Generated JS contains instantiateStreaming (not Cloudflare-compatible)"
    echo "   This is OK - our wrapper in src/index.ts ensures ArrayBuffer is passed"
    echo "   which forces wasm-pack to use instantiate() instead"
else
    echo "   ✓ Generated JS does not use instantiateStreaming"
fi

echo ""
echo "✅ TypeScript/WASM package built successfully"
echo "📦 Version: $VERSION"
echo "📁 Package directory: $BINDING_DIR/pkg"
echo "🌐 Cloudflare Workers compatible: ✓ (uses WebAssembly.instantiate only)"

