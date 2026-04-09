# Task 10: Error Handling and Edge Case Hardening

## Objective

Harden the Tauri app and MCP server against the edge cases that will crash things when you're not watching.

## Steps

### Tauri App Hardening

1. **Screen capture failure recovery:**
   - If `scrot` or `grim` fails (not installed, display error, permission denied), show a brief error toast on the overlay and close after 3 seconds
   - Do not leave a blank overlay hanging with no way out
   - Log the error to `~/.snap/error.log` with timestamp

2. **Canvas export failure:**
   - If `canvas.toDataURL()` fails (unlikely but possible on very large displays), catch the error
   - Fallback: save just the sidecar JSON with a note that the image export failed
   - Do not silently lose annotations

3. **Disk space / write failure:**
   - If writing to `~/.snap/inbox/` fails, show error toast, do not close overlay so the user can try again or escape
   - Check that the inbox directory is writable on startup

4. **Rapid hotkey spam:**
   - If the user presses the hotkey while the overlay is already open, ignore it
   - Guard against multiple overlay windows spawning simultaneously
   - Use a simple boolean lock: `is_overlay_active`

5. **Display resolution mismatch:**
   - If the screen resolution changes between capture and overlay display (e.g., plugging in a monitor), the capture dimensions won't match the canvas
   - Detect this: compare captured image dimensions to current `window.screen` dimensions
   - If mismatch, recapture or show a warning

### MCP Server Hardening

6. **Corrupt JSON files:**
   - If a sidecar JSON is malformed, skip it and move to the next one
   - Return a warning in the response: `"warning": "skipped corrupt file: snap-XXXX.json"`
   - Do not crash the entire tool call

7. **Missing PNG for a JSON:**
   - Already handled (`image_path: null`) but add a `"warning": "image file missing"` field

8. **Empty inbox:**
   - `get_latest_annotation()` already returns an error dict
   - `list_annotations()` should return an empty array, not an error
   - `clear_inbox()` on empty inbox should return `{"deleted_json": 0, "deleted_png": 0}`, not an error

9. **File permissions:**
   - If the inbox directory exists but isn't readable, return a clear error message
   - Do not throw an unhandled exception

10. **Large inbox:**
    - If someone never clears the inbox and it grows to hundreds of files, `list_annotations(last_n=5)` should still be fast
    - The glob + sort approach is fine for hundreds of files. Add a note if inbox exceeds 100 files: `"note": "inbox has 147 files, consider clearing old annotations"`

### Logging

11. Add a simple log file at `~/.snap/snap.log`:
    - Tauri app logs: capture start/end, save success/failure, hotkey triggered
    - MCP server logs: tool calls with timestamps
    - Rotate or truncate at 1MB
    - Use append mode, not overwrite

## Verification

- Force-kill `scrot` mid-capture (or uninstall it temporarily). App should show error and close gracefully.
- Create a malformed JSON in the inbox. MCP tools should skip it, not crash.
- Press hotkey 10 times rapidly. Only one overlay should appear.
- Fill inbox with 50+ test files. `list_annotations(5)` returns in under 1 second.
- Check `~/.snap/snap.log` exists and contains entries.

## Do Not Touch

- Do not change the annotation tools or drawing behavior
- Do not change the sidecar JSON schema
- Do not change the MCP tool signatures (only add defensive handling inside them)
