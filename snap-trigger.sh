#!/bin/bash
# Trigger a new Snap overlay. Called by the desktop environment's global hotkey.
# If the tray app is running, signal it. Otherwise launch the overlay directly.

BINARY="$HOME/Desktop/snap/app/src-tauri/target/release/snap"
LOCK="/tmp/snap-overlay.lock"
LOG="$HOME/.snap/snap.log"

log() {
    mkdir -p "$HOME/.snap"
    echo "[$(date -u '+%Y-%m-%d %H:%M:%S')] [trigger] $1" >> "$LOG"
}

# Guard: don't open if overlay is already active
if [ -f "$LOCK" ]; then
    pid=$(cat "$LOCK" 2>/dev/null)
    if kill -0 "$pid" 2>/dev/null; then
        log "overlay already active (pid=$pid), ignoring"
        exit 0
    else
        rm -f "$LOCK"
    fi
fi

log "hotkey triggered — launching overlay"

# Launch the app in single-shot overlay mode
# The app will capture screen, show overlay, save, and exit
"$BINARY" --overlay-mode &
OVERLAY_PID=$!
echo "$OVERLAY_PID" > "$LOCK"

# Clean up lock when overlay exits
wait "$OVERLAY_PID" 2>/dev/null
rm -f "$LOCK"
log "overlay closed"
