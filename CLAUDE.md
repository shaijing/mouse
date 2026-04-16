# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build          # Debug build
cargo build --release  # Release build (required for deployment as LaunchAgent)
cargo test           # Run tests
cargo run            # Run in debug mode
```

## Architecture

This is a macOS mouse scroll reverser tool that intercepts scroll events via Core Graphics event taps and reverses their direction based on user configuration.

**Two main modules:**

- `src/main.rs` - Event tap implementation using `CGEventTap` to intercept `ScrollWheel` events at HID level. Distinguishes between mouse (non-continuous) and trackpad (continuous) events via `SCROLL_WHEEL_EVENT_IS_CONTINUOUS` field. Three callback functions handle different device combinations: `mouse_reverse`, `trackpad_reverse`, `mouse_trackpad_reverse`.

- `src/config.rs` - TOML configuration parsing via serde. Config file (`mouse.toml`) must be in the same directory as the executableœ.

**Configuration options:**
- `vertical_reverse`, `horizontal_reverse` - scroll direction inversion
- `scroll_sensitivity` - multiplier for scroll delta values
- `mouse_reverse`, `trackpad_reverse` - which devices to apply reversal to

**Deployment:** The tool runs as a macOS LaunchAgent (see README.md for plist setup). It requires accessibility permissions to intercept HID events.