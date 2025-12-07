#!/bin/bash
# Get version from git tag or return default
# This script is used by build processes to determine version

set -e

# Try to get version from git tag
VERSION_TAG=$(git describe --tags --exact-match 2>/dev/null || git describe --tags 2>/dev/null || echo "")

if [ -n "$VERSION_TAG" ]; then
    # Extract version (remove 'v' prefix and any commit suffix)
    VERSION=$(echo "$VERSION_TAG" | sed 's/^v//' | sed 's/-.*$//')
    # Validate semantic versioning format
    if [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "$VERSION"
        exit 0
    fi
fi

# Fallback to version from package files or default
if [ -f "bindings/python/pyproject.toml" ]; then
    VERSION=$(grep -E '^version = ' bindings/python/pyproject.toml | sed 's/version = "\(.*\)"/\1/')
    if [ -n "$VERSION" ]; then
        echo "$VERSION"
        exit 0
    fi
fi

# Default fallback
echo "0.1.0"

