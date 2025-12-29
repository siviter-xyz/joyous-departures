#!/bin/bash
# Build Python wheel using maturin's native mixed project support
# This script uses maturin's proper configuration - no manual hacks needed!

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINDING_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_ROOT="$(cd "$BINDING_DIR/../.." && pwd)"

# Get version from git tag or pyproject.toml
if [ -n "$1" ]; then
    VERSION="$1"
else
    # Try to get version from git tag
    if git -C "$REPO_ROOT" describe --tags --exact-match HEAD >/dev/null 2>&1; then
        VERSION=$(git -C "$REPO_ROOT" describe --tags --exact-match HEAD | sed 's/^v//')
    else
        # Fall back to pyproject.toml
        VERSION=$(grep -E '^version\s*=' "$BINDING_DIR/pyproject.toml" | head -1 | sed -E 's/.*version\s*=\s*"([^"]+)".*/\1/')
    fi
fi

echo "🔨 Building Python wheel for version: $VERSION"
echo "📦 Binding directory: $BINDING_DIR"
echo "📁 Repo root: $REPO_ROOT"

cd "$BINDING_DIR"

# Ensure cargo is in PATH
source ~/.cargo/env 2>/dev/null || true

# Setup virtual environment
if [ ! -d ".venv" ]; then
    echo "📦 Creating virtual environment..."
    uv venv --python 3.13
fi

source .venv/bin/activate

# Install build dependencies
echo "📥 Installing build dependencies..."
uv pip install -q maturin

# Copy README to package directory (required by maturin)
if [ ! -f "README.md" ]; then
    echo "📄 Copying README..."
    cp "$REPO_ROOT/README.md" README.md
fi

# Build wheel with maturin (now properly configured as mixed project)
echo "🔨 Building wheel with maturin (mixed project)..."
maturin build --release

# Find the built wheel
WHEEL=$(find "$REPO_ROOT/target/wheels" -name "joyous_departures-${VERSION}-*.whl" 2>/dev/null | head -1)
if [ -z "$WHEEL" ]; then
    WHEEL=$(find "$BINDING_DIR/target/wheels" -name "joyous_departures-${VERSION}-*.whl" 2>/dev/null | head -1)
fi

if [ -z "$WHEEL" ]; then
    echo "❌ ERROR: Wheel not found for version $VERSION"
    echo "Searched in:"
    echo "  - $REPO_ROOT/target/wheels/"
    echo "  - $BINDING_DIR/target/wheels/"
    exit 1
fi

echo "✅ Found wheel: $WHEEL"

# Verify the wheel contents
echo ""
echo "🔍 Verifying wheel contents..."
python3 << PYTHON_VERIFY
import zipfile
import sys

wheel_path = "$WHEEL"
required_files = [
    "joyous_departures/__init__.py",
    "joyous_departures/_joy_generator.cpython-313-x86_64-linux-gnu.so",
]

with zipfile.ZipFile(wheel_path, 'r') as z:
    files = z.namelist()
    
    # Check for required files (allow for different Python versions/architectures)
    missing = []
    for req_file in required_files:
        # Check if any file matches the pattern
        if req_file.endswith('.so'):
            # For .so files, check if any matches the pattern
            so_files = [f for f in files if 'joyous_departures/_joy_generator' in f and f.endswith('.so')]
            if not so_files:
                missing.append(req_file)
        else:
            if req_file not in files:
                missing.append(req_file)
    
    if missing:
        print(f"❌ Missing required files: {missing}")
        print(f"   Available files: {[f for f in files if 'joyous_departures' in f]}")
        sys.exit(1)
    
    # Check RECORD file
    dist_info = [f for f in files if f.endswith('.dist-info/RECORD')]
    if not dist_info:
        print("❌ RECORD file not found")
        sys.exit(1)
    
    # Verify RECORD includes all files
    record_content = z.read(dist_info[0]).decode('utf-8')
    record_files = [line.split(',')[0].lstrip('./') for line in record_content.strip().split('\n') if line]
    
    # Check that joyous_departures is in RECORD
    if not any(f.startswith('joyous_departures/') for f in record_files):
        print("❌ joyous_departures package not in RECORD file")
        print(f"   RECORD entries: {[f for f in record_files if 'joyous' in f]}")
        sys.exit(1)
    
    print("✅ Wheel verification passed")
    print(f"   - joyous_departures package: ✓")
    print(f"   - Rust extension module: ✓")
    print(f"   - RECORD file: ✓")
PYTHON_VERIFY

if [ $? -ne 0 ]; then
    echo "❌ Wheel verification failed"
    exit 1
fi

echo ""
echo "✅ Wheel built successfully: $WHEEL"
echo "📦 Version: $VERSION"
echo ""
echo "📋 Wheel contents:"
python3 -m zipfile -l "$WHEEL" | grep -E "(joyous_departures|RECORD)" | head -10
