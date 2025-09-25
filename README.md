# Mac OS Mouse Scroll Reverser

## Use
```bash
vim ~/Library/LaunchAgents/com.science4ai.mouse.plist


<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.science4ai.mouse</string>

    <key>ProgramArguments</key>
    <array>
        <string>/Users/ling/ws/rustWS/mouse/target/release/mouse</string>
    </array>

    <key>KeepAlive</key>
    <true/>

    <key>RunAtLoad</key>
    <true/>

    <key>WorkingDirectory</key>
    <string>/Users/ling/ws/rustWS/mouse/target/release/</string>

    <key>StandardOutPath</key>
    <string>/Users/ling/Library/Logs/mouse.out</string>

    <key>StandardErrorPath</key>
    <string>/Users/ling/Library/Logs/mouse.err</string>
</dict>
</plist>


launchctl load ~/Library/LaunchAgents/com.science4ai.mouse.plist

```