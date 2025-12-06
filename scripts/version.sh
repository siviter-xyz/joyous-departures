#!/bin/bash
# Version management script for joy-goodbye
# Extracts version from git tag and updates all package files

set -e

# Get version from git tag (format: v1.2.3)
VERSION_TAG=$(git describe --tags --exact-match 2>/dev/null || echo "")

if [ -z "$VERSION_TAG" ]; then
    echo "Error: No git tag found. Create a tag first: git tag v0.1.0" >&2
    exit 1
fi

# Extract version (remove 'v' prefix)
VERSION=${VERSION_TAG#v}

# Validate semantic versioning format
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Invalid version format. Expected semantic version (e.g., 0.1.0)" >&2
    exit 1
fi

echo "Updating version to $VERSION..."

# Update Cargo.toml (workspace)
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# Update package.json
if [ -f "bindings/typescript/package.json" ]; then
    cd bindings/typescript
    # Use npm/pnpm version command if available, otherwise sed
    if command -v pnpm &> /dev/null; then
        pnpm version "$VERSION" --no-git-tag-version --no-commit-hooks
    else
        sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json
    fi
    cd ../..
fi

# Update pyproject.toml
if [ -f "bindings/python/pyproject.toml" ]; then
    sed -i "s/^version = \".*\"/version = \"$VERSION\"/" bindings/python/pyproject.toml
fi

echo "✅ Version updated to $VERSION in all package files"
echo ""
echo "Files updated:"
echo "  - Cargo.toml"
echo "  - bindings/typescript/package.json"
echo "  - bindings/python/pyproject.toml"
