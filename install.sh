#!/bin/bash

# Mac OS Mouse Scroll Reverser Installation Script
# This script builds, installs, and sets up the LaunchAgent service

set -e

# Configuration
INSTALL_DIR="$HOME/.scripts/mouse"
BINARY_NAME="mouse"
CONFIG_NAME="mouse.toml"
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

# Check if running on macOS
if [[ "$(uname)" != "Darwin" ]]; then
    print_error "This script is only for macOS"
    exit 1
fi

# Get script directory (project root)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

print_info "Building release binary..."
cargo build --release

print_info "Creating installation directory..."
mkdir -p "$INSTALL_DIR"
mkdir -p "$INSTALL_DIR/Logs"

print_info "Copying binary to $INSTALL_DIR..."
cp "$SCRIPT_DIR/target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

# Check if config file already exists
if [[ -f "$INSTALL_DIR/$CONFIG_NAME" ]]; then
    print_warn "Config file already exists at $INSTALL_DIR/$CONFIG_NAME"
    print_warn "Keeping existing config. To use new config, manually delete the old one."
else
    print_info "Creating default config file..."
    cat > "$INSTALL_DIR/$CONFIG_NAME" << 'EOF'
vertical_reverse = true
horizontal_reverse = false
scroll_sensitivity = 1
mouse_reverse = true
trackpad_reverse = false
EOF
fi

print_info "Creating LaunchAgent plist..."
cat > "$PLIST_PATH" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
    <dict>
        <key>Label</key>
        <string>com.science4ai.mouse</string>

        <key>ProgramArguments</key>
        <array>
            <string>$INSTALL_DIR/$BINARY_NAME</string>
        </array>

        <key>KeepAlive</key>
        <dict>
            <key>SuccessfulExit</key>
            <false />
        </dict>

        <key>RunAtLoad</key>
        <true />

        <key>WorkingDirectory</key>
        <string>$INSTALL_DIR</string>

        <key>StandardOutPath</key>
        <string>$INSTALL_DIR/Logs/mouse.out</string>

        <key>StandardErrorPath</key>
        <string>$INSTALL_DIR/Logs/mouse.err</string>
    </dict>
</plist>
EOF

# Check if service is already running
if launchctl list | grep -q "com.science4ai.mouse"; then
    print_warn "Service is already running. Stopping it first..."
    launchctl bootout gui/$(id -u) "$PLIST_PATH" 2>/dev/null || true
fi

print_info "Loading LaunchAgent..."
launchctl bootstrap gui/$(id -u) "$PLIST_PATH"

print_info "Installation complete!"
echo ""
echo "Binary:     $INSTALL_DIR/$BINARY_NAME"
echo "Config:     $INSTALL_DIR/$CONFIG_NAME"
echo "Logs:       $INSTALL_DIR/Logs/"
echo "Plist:      $PLIST_PATH"
echo ""
print_warn "Note: You may need to grant Accessibility permissions to the application."
print_warn "Go to System Preferences > Security & Privacy > Privacy > Accessibility"
print_warn "and add '$INSTALL_DIR/$BINARY_NAME' to the allowed applications."
echo ""
print_info "To check service status: launchctl list | grep mouse"
print_info "To stop service: launchctl bootout gui/$(id -u) $PLIST_PATH"
print_info "To restart service: launchctl bootout gui/$(id -u) $PLIST_PATH && launchctl bootstrap gui/$(id -u) $PLIST_PATH"
print_info "To view logs: tail -f $INSTALL_DIR/Logs/mouse.err"