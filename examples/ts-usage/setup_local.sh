#!/bin/bash
# Setup script to install local package for testing
# Usage: ./setup_local.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXAMPLES_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_ROOT="$(cd "$EXAMPLES_DIR/.." && pwd)"

# Check for local package path from .env.local
ENV_FILE="$EXAMPLES_DIR/.env.local"
LOCAL_PACKAGE_PATH=""

if [ -f "$ENV_FILE" ]; then
    while IFS='=' read -r key value; do
        # Skip comments and empty lines
        [[ "$key" =~ ^#.*$ ]] && continue
        [[ -z "$key" ]] && continue
        
        if [ "$key" = "TYPESCRIPT_PACKAGE_PATH" ]; then
            # Remove quotes
            LOCAL_PACKAGE_PATH=$(echo "$value" | sed 's/^["'\'']//; s/["'\'']$//')
            break
        fi
    done < "$ENV_FILE"
fi

# Also check environment variable
if [ -z "$LOCAL_PACKAGE_PATH" ]; then
    LOCAL_PACKAGE_PATH="$TYPESCRIPT_PACKAGE_PATH"
fi

# Resolve relative paths
if [ -n "$LOCAL_PACKAGE_PATH" ]; then
    if [[ ! "$LOCAL_PACKAGE_PATH" = /* ]]; then
        LOCAL_PACKAGE_PATH="$REPO_ROOT/$LOCAL_PACKAGE_PATH"
    fi
    
    if [ -d "$LOCAL_PACKAGE_PATH" ]; then
        echo "📦 Installing local package from: $LOCAL_PACKAGE_PATH"
        npm install "$LOCAL_PACKAGE_PATH"
        echo "✅ Local package installed"
    else
        echo "⚠️  Local package path not found: $LOCAL_PACKAGE_PATH"
        echo "   Using published package instead"
    fi
else
    echo "ℹ️  No local package path found, using published package"
    echo "   Set TYPESCRIPT_PACKAGE_PATH in examples/.env.local to use local builds"
fi

