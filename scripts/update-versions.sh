#!/bin/bash
# Update version in all package files
# Called by semantic-release during prepare step

set -e

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Error: Version not provided" >&2
    exit 1
fi

# Remove 'v' prefix if present
VERSION=${VERSION#v}

# Validate semantic versioning format
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Invalid version format. Expected semantic version (e.g., 0.1.0)" >&2
    exit 1
fi

echo "Updating version to $VERSION in all package files..."

# Update Cargo.toml (workspace)
if [ -f "Cargo.toml" ]; then
    sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
    echo "  ✅ Cargo.toml"
fi

# Update package.json
if [ -f "bindings/typescript/package.json" ]; then
    cd bindings/typescript
    # Use pnpm version command if available, otherwise sed
    if command -v pnpm &> /dev/null; then
        pnpm version "$VERSION" --no-git-tag-version --no-commit-hooks || \
        sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json
    else
        sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json
    fi
    cd ../..
    echo "  ✅ bindings/typescript/package.json"
fi

# Update pyproject.toml
if [ -f "bindings/python/pyproject.toml" ]; then
    sed -i "s/^version = \".*\"/version = \"$VERSION\"/" bindings/python/pyproject.toml
    echo "  ✅ bindings/python/pyproject.toml"
fi

echo "✅ All package files updated to version $VERSION"

