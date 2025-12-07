#!/bin/bash
# Setup git hooks from .githooks directory
set -e

REPO_ROOT="$(git rev-parse --show-toplevel)"
GIT_HOOKS_DIR="$REPO_ROOT/.git/hooks"
GITHOOKS_DIR="$REPO_ROOT/.githooks"

if [ ! -d "$GITHOOKS_DIR" ]; then
    echo "❌ Error: .githooks directory not found!"
    exit 1
fi

echo "🔧 Setting up git hooks..."
echo ""

# Create .git/hooks if it doesn't exist
mkdir -p "$GIT_HOOKS_DIR"

# Copy hooks from .githooks to .git/hooks
for hook in "$GITHOOKS_DIR"/*; do
    if [ -f "$hook" ] && [ -x "$hook" ]; then
        hook_name=$(basename "$hook")
        target="$GIT_HOOKS_DIR/$hook_name"
        
        echo "  Installing $hook_name..."
        cp "$hook" "$target"
        chmod +x "$target"
    fi
done

echo ""
echo "✅ Git hooks installed successfully!"
echo ""
echo "The following hooks are now active:"
ls -1 "$GIT_HOOKS_DIR" | grep -v sample || echo "  (none found)"
echo ""
echo "To test, try making a commit. The pre-commit hook will run automatically."

