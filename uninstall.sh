#!/bin/bash

# Mac OS Mouse Scroll Reverser Uninstallation Script
# This script stops and removes the LaunchAgent service

set -e

# Configuration
INSTALL_DIR="$HOME/.scripts/mouse"
PLIST_NAME="com.science4ai.mouse.plist"
PLIST_PATH="$HOME/Library/LaunchAgents/$PLIST_NAME"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if service is running
if launchctl list | grep -q "com.science4ai.mouse"; then
    print_info "Stopping LaunchAgent service..."
    launchctl bootout gui/$(id -u) "$PLIST_PATH" 2>/dev/null || true
else
    print_warn "Service is not running"
fi

# Remove plist file
if [[ -f "$PLIST_PATH" ]]; then
    print_info "Removing plist file..."
    rm "$PLIST_PATH"
else
    print_warn "Plist file not found at $PLIST_PATH"
fi

# Ask about removing installation directory
echo ""
print_warn "Installation directory: $INSTALL_DIR"
read -p "Do you want to remove the installation directory? (y/N): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_info "Removing installation directory..."
    rm -rf "$INSTALL_DIR"
    print_info "Installation directory removed"
else
    print_info "Keeping installation directory. You can manually remove it later:"
    echo "  rm -rf $INSTALL_DIR"
fi

print_info "Uninstallation complete!"