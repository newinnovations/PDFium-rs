#!/bin/bash

# Script to install git hooks from the repository
# Run this script from the repository root

set -e

HOOKS_DIR="scripts/hooks"
GIT_HOOKS_DIR=".git/hooks"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "Installing git hooks..."

# Check if we're in a git repository
if [[ ! -d ".git" ]]; then
    echo "Error: Not in a git repository root"
    exit 1
fi

# Check if hooks directory exists
if [[ ! -d "$HOOKS_DIR" ]]; then
    echo "Error: $HOOKS_DIR directory not found"
    exit 1
fi

# Create .git/hooks directory if it doesn't exist
mkdir -p "$GIT_HOOKS_DIR"

# Install each hook
for hook in "$HOOKS_DIR"/*; do
    if [[ -f "$hook" ]]; then
        hook_name=$(basename "$hook")
        
        # Make the hook executable
        chmod +x "$hook"
        
        # Create symlink to the hook in .git/hooks
        ln -sf "../../$HOOKS_DIR/$hook_name" "$GIT_HOOKS_DIR/$hook_name"
        
        echo -e "${GREEN}✓ Installed $hook_name hook${NC}"
    fi
done

echo -e "${GREEN}Git hooks installation complete!${NC}"
echo -e "${YELLOW}Hooks are now active and will run on commits.${NC}"
