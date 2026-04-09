# Task 8: Integration Test -- End-to-End Annotation to Claude Code

## Objective

Verify the full loop works: hotkey to overlay to annotation to save to MCP to Claude Code reading the image and metadata.

## Prerequisites

- Tasks 1-7 are complete and individually verified
- Tauri app is built and running (tray mode, listening for hotkey)
- MCP server is registered with Claude Code (`claude mcp list` shows `snap` connected)

## Test Sequence

### Test 1: Basic annotation flow

1. Open a browser to any page with visible UI elements (your Adjective site or Girolamo)
2. Press the global hotkey (`Ctrl+Shift+S`)
3. Verify: overlay appears with a static screenshot of the browser
4. Draw a red circle around a UI element
5. Add a text label: "wrong color"
6. Press Enter to save
7. Verify: overlay closes
8. Check `~/.snap/inbox/` for new PNG + JSON pair
9. Open Claude Code
10. Say: "check my latest snap annotation"
11. Verify: Claude Code calls `get_latest_annotation`, gets the metadata
12. Say: "look at the image at [path from metadata]"
13. Verify: Claude Code reads the PNG and describes what it sees including the red circle and text

### Test 2: Window context accuracy

1. Open a browser to `http://localhost:3000/dashboard`
2. Press hotkey, draw any annotation, save
3. Check the sidecar JSON
4. Verify: `source.window_title` contains something like "Dashboard - localhost:3000 - Brave"
5. Verify: `source.window_class` is populated

### Test 3: Multiple annotations

1. Take 3 screenshots with annotations in sequence
2. In Claude Code: "list my recent snap annotations"
3. Verify: returns 3 entries, newest first
4. "Get the annotation called snap-XXXXXXXX-XXXXXX" (use a specific filename)
5. Verify: returns the correct one

### Test 4: Cleanup

1. In Claude Code: "clear the snap inbox"
2. Verify: `~/.snap/inbox/` is empty
3. "Get latest annotation" should return the no-annotations error

### Test 5: Rapid fire

1. Hotkey, circle something, save. Immediately hotkey again, arrow something, save.
2. Verify: two distinct PNG+JSON pairs with different timestamps
3. No file collisions, no corruption

## Known Issues to Watch For

- **Timing**: if the screen capture command is slow, the overlay might briefly appear in the screenshot. The capture must complete before the window is shown.
- **Canvas export quality**: `toDataURL('image/png')` on a fullscreen canvas produces a large file. If files are >10MB, consider adding quality reduction or resolution capping in a future task.
- **Wayland**: window context (Task 5) returns null on Wayland. The rest should still work.

## Success Criteria

All 5 tests pass. The workflow from "I see a bug" to "Claude Code sees my annotated screenshot" takes under 10 seconds.

## After This Task

The tool is usable for your daily workflow. Future enhancements (not tasks):
- Rectangle selection tool
- Numbered annotations (circle with "1", "2", etc.)
- Auto-prompt: Claude Code watches inbox and proactively asks about new annotations
- macOS support (screencapture instead of scrot, different window context APIs)
- Publish as open-source Adjective project
