#!/bin/bash
# Extract version from git tag and update all package files

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 1.0.0"
    exit 1
fi

VERSION="$1"

echo "Updating version to $VERSION..."

# Update workspace Cargo.toml
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# Update package.json
sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" bindings/typescript/package.json

# Update pyproject.toml
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" bindings/python/pyproject.toml

echo "Version updated to $VERSION"

