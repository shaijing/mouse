# Mac OS Mouse Scroll Reverser

A macOS mouse scroll reverser tool that intercepts scroll events via Core Graphics event taps and reverses their direction based on user configuration.

## Installation

```bash
./install.sh
```

The script will:
- Build the release binary
- Install to `~/.scripts/mouse/`
- Create default config file
- Set up LaunchAgent service

**Note:** You need to grant Accessibility permissions after installation:
- Go to System Preferences > Security & Privacy > Privacy > Accessibility
- Add `~/.scripts/mouse/mouse` to the allowed applications

## Uninstallation

```bash
./uninstall.sh
```

## Configuration

Edit `~/.scripts/mouse/mouse.toml`:

```toml
vertical_reverse = true      # Reverse vertical scroll
horizontal_reverse = false   # Reverse horizontal scroll
scroll_sensitivity = 1       # Scroll speed multiplier
mouse_reverse = true         # Apply to mouse
trackpad_reverse = false     # Apply to trackpad
```

## Service Management

```bash
# Check status
launchctl list | grep mouse

# Stop service
launchctl bootout gui/$(id -u) ~/Library/LaunchAgents/com.science4ai.mouse.plist

# Start service
launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/com.science4ai.mouse.plist

# View logs
tail -f ~/.scripts/mouse/Logs/mouse.err
```

## Build from Source

```bash
cargo build --release
```