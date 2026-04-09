# Task 1: Scaffold Tauri Project with Transparent Fullscreen Overlay

## Objective

Create a new Tauri 2.x project that opens a transparent, frameless, fullscreen window. This is the foundation for the screen annotation overlay.

## Steps

1. Initialize a new Tauri 2.x project in `~/projects/snap/app/`
   - Use vanilla HTML/CSS/JS for the frontend (no React, no framework)
   - Name: `snap`
   - Bundle identifier: `com.adjective.snap`

2. Configure `tauri.conf.json` window properties:
   - `transparent: true`
   - `decorations: false`
   - `fullscreen: true`
   - `alwaysOnTop: true`
   - `skipTaskbar: true`
   - `resizable: false`

3. Set the HTML body and root element to:
   - `background: transparent`
   - Full viewport width/height
   - No margins, no padding
   - Cursor: crosshair

4. Add a simple visual test: draw a semi-transparent red rectangle in the center of the screen to confirm transparency works. You should see your desktop through the window with the red rectangle floating on top.

5. Add an escape key listener that calls `appWindow.close()` to dismiss the overlay.

## Verification

- `cargo tauri dev` opens a fullscreen transparent overlay
- Desktop content is visible behind the overlay
- Red test rectangle renders on top
- Pressing Escape closes the window
- No title bar, no window frame, no taskbar entry

## Do Not Touch

- Do not add any drawing tools yet (Task 2)
- Do not add screen capture yet (Task 3)
- Do not add hotkey registration yet (Task 6)

## Dependencies

- Rust toolchain (rustup)
- Node.js 18+
- Tauri CLI 2.x: `cargo install tauri-cli --version "^2"`
- Linux: `sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev`
