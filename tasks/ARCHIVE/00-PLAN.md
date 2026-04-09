# Snap: Visual Annotation Layer for Agentic Coding

## What This Is

A two-component system that lets you annotate your screen and feed marked-up screenshots directly into Claude Code via MCP.

**Component A:** Tauri desktop app. Global hotkey freezes screen into transparent overlay. Drawing tools (circle, arrow, freehand, text, color picker). Save exports annotated PNG + sidecar JSON to `~/.snap/inbox/`.

**Component B:** Python MCP server (fastmcp, stdio transport). Exposes tools that let Claude Code read annotations from the inbox. Registered with `claude mcp add`.

## Architecture

```
[You see a bug in browser]
        |
   Cmd+Shift+S (global hotkey)
        |
[Tauri overlay appears over frozen screen]
        |
   You circle the bug, type "fix this"
        |
   Hit Enter
        |
[Tauri saves to ~/.snap/inbox/]
   - snap-{timestamp}.png (annotated image)
   - snap-{timestamp}.json (metadata sidecar)
        |
[Back in terminal, tell Claude Code to look]
        |
[Claude Code calls get_latest_annotation() via MCP]
        |
[MCP server returns path to PNG + parsed metadata]
        |
[Claude Code reads image, sees annotations + context, makes fix]
```

## Sidecar JSON Schema

```json
{
  "timestamp": "2026-04-08T14:23:00Z",
  "source": {
    "window_title": "localhost:3000/dashboard - Brave",
    "url": "http://localhost:3000/dashboard",
    "display": "primary",
    "resolution": [1920, 1080]
  },
  "annotations": [
    {
      "type": "circle",
      "center": [340, 220],
      "radius": 45,
      "color": "#FF0000",
      "label": "fix this alignment"
    },
    {
      "type": "arrow",
      "from": [340, 265],
      "to": [340, 300],
      "color": "#FF0000",
      "label": null
    },
    {
      "type": "text",
      "position": [350, 310],
      "content": "should be 16px gap",
      "color": "#FF0000"
    }
  ],
  "image_path": "~/.snap/inbox/snap-1712345678.png"
}
```

## MCP Tools

| Tool | Purpose |
|------|---------|
| `get_latest_annotation()` | Returns path + metadata for most recent annotation |
| `list_annotations(last_n=5)` | Returns recent annotations |
| `get_annotation(filename)` | Returns specific annotation by name |
| `clear_inbox()` | Cleans up processed annotations |

## Tech Stack

- **Tauri 2.x** (Rust backend, web frontend)
- **Canvas API** for drawing overlay
- **Python 3.11+** with `fastmcp` for MCP server
- **xdotool / xdg / wmctrl** (Linux) for window context capture
- Target platform: **Linux (X11)** first (your dev machine), macOS later

## Task Breakdown

8 tasks, ordered by dependency:

1. Scaffold Tauri project with transparent fullscreen overlay
2. Build canvas drawing tools (circle, arrow, freehand, text, color picker)
3. Implement screen capture (grab the screen content behind the overlay)
4. Build save handler (PNG export + sidecar JSON generation)
5. Add window context capture (active window title, URL extraction)
6. Add global hotkey registration
7. Build the MCP server
8. Integration test: end-to-end annotation to Claude Code consumption

Each task is a separate markdown file with exact instructions for Claude Code.
