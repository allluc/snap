# Task 3: Implement Screen Capture Behind the Overlay

## Objective

When the user triggers the overlay, capture the screen content BEFORE the overlay renders. This captured image becomes the background of the annotation, so the exported PNG shows the screen + annotations composited together.

## Context

Right now the overlay is transparent and the user sees through it to the desktop. But when we export (Task 4), we need the actual screen pixels. We need to capture the screen at overlay launch, use it as the canvas background, and composite annotations on top.

## Steps

1. In the Rust backend (`src-tauri/src/lib.rs` or `src-tauri/src/main.rs`), create a Tauri command:

```rust
#[tauri::command]
fn capture_screen() -> Result<String, String> {
    // Use scrot (X11) or grim (Wayland) to capture the screen
    // Save to a temp file
    // Return the file path
}
```

Platform detection:
- Check `WAYLAND_DISPLAY` env var. If set, use `grim /tmp/snap-capture.png`
- Otherwise use `scrot /tmp/snap-capture.png`
- The capture must happen BEFORE the Tauri window becomes visible

2. Modify the app launch sequence:
   - On app start (or when overlay is triggered), call `capture_screen` from the frontend via `invoke`
   - Wait for the capture to complete
   - Load the captured image as the canvas background using `ctx.drawImage()`
   - THEN make the window visible (use Tauri's window `show()` API, start with window hidden)

3. Update `tauri.conf.json`:
   - Set `visible: false` on the window config (start hidden)
   - The frontend calls `capture_screen`, loads the image, then calls `appWindow.show()`

4. The canvas rendering pipeline becomes:
   - Clear canvas
   - Draw captured screen image as background (full canvas size)
   - Draw all annotations on top
   - This means the overlay is no longer "transparent" in the see-through sense. It shows the captured screenshot with annotations composited on top.

5. Handle the brief delay:
   - The capture + load takes a moment. Show nothing until ready (window stays hidden).
   - Once the image is loaded and drawn, show the window. The transition should feel instant.

## Verification

- Launch the app. There should be a brief moment where nothing happens, then the overlay appears showing a static screenshot of your desktop.
- The screenshot matches exactly what was on screen before the overlay appeared.
- Drawing tools from Task 2 still work on top of the captured image.
- Moving windows on the desktop underneath does NOT change what the overlay shows (it's a static capture, not a live view).

## Do Not Touch

- Do not modify the drawing tools from Task 2
- Do not implement the export/save handler yet (Task 4)
- Do not add global hotkey yet (Task 6)

## Dependencies

- Linux X11: `sudo apt install scrot`
- Linux Wayland: `sudo apt install grim`

## Edge Cases

- Multi-monitor: `scrot` captures all monitors by default. For v1, capture the primary monitor only. Use `scrot --focused` or specify display geometry if needed.
- If `scrot` or `grim` is not installed, the command should return a clear error message, not crash.
