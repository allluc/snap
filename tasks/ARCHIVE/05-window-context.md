# Task 5: Add Window Context Capture

## Objective

Before the overlay appears, capture metadata about what the user was looking at: the active window title and (if it's a browser) the URL. This context gets written into the sidecar JSON so Claude Code can map visual annotations to source files.

## Context

Task 4 left `source.window_title` and `source.url` as null in the sidecar JSON. This task fills them in.

## Steps

1. In the Rust backend, create a new Tauri command:

```rust
#[tauri::command]
fn get_active_window_context() -> Result<WindowContext, String> {
    // Must be called BEFORE the Tauri overlay window is shown
}

#[derive(serde::Serialize)]
struct WindowContext {
    window_title: Option<String>,
    url: Option<String>,
    window_class: Option<String>,
    pid: Option<u32>,
}
```

2. Implementation for Linux X11:
   - Use `xdotool getactivewindow getwindowname` to get the window title
   - Use `xdotool getactivewindow getwindowclassname` to get the window class (e.g., "Brave-browser", "firefox")
   - Use `xdotool getactivewindow getwindowpid` to get the PID

3. URL extraction from browser windows:
   - If the window class indicates a browser (brave, firefox, chromium, google-chrome), attempt to extract the URL from the window title
   - Most browsers put the URL or page title in the window title: "Page Title - Brave" or "localhost:3000/dashboard - Brave"
   - Parse the title: split on " - " or " — ", take everything before the last separator as the page title
   - For actual URL extraction: use `xdotool` to get the window ID, then use `xdg-open` or accessibility tools. However, this is fragile. For v1, just capture the window title as-is and let Claude Code infer the URL from it. Do NOT attempt clipboard manipulation or accessibility API scraping.

4. Timing: this command must run BEFORE the overlay window is shown. Modify the app launch sequence from Task 3:
   - Step 1: `get_active_window_context()` -- capture who was focused
   - Step 2: `capture_screen()` -- capture what was on screen
   - Step 3: Load image, show overlay
   - Store the context in a global JS variable for use at save time

5. Update the save handler from Task 4 to include the context:

```javascript
const metadata = {
  timestamp: new Date().toISOString(),
  source: {
    window_title: windowContext.window_title,
    url: windowContext.url,
    window_class: windowContext.window_class,
    pid: windowContext.pid,
    display: "primary",
    resolution: [window.screen.width, window.screen.height]
  },
  annotations: [ /* ... */ ]
};
```

## Verification

- Open a browser to `localhost:3000/something`
- Launch the overlay
- Draw an annotation and save
- Check the sidecar JSON: `source.window_title` should contain the browser window title
- Check `source.window_class` is populated (e.g., "Brave-browser")
- Repeat with a non-browser window (terminal, VS Code). Title should still capture, URL will be null.

## Do Not Touch

- Do not modify drawing tools from Task 2
- Do not modify screen capture from Task 3
- Do not modify the save file format from Task 4 (only populate the null fields)
- Do not add global hotkey yet (Task 6)

## Dependencies

- `xdotool`: `sudo apt install xdotool`

## Edge Cases

- No active window (empty desktop): return all fields as None, do not crash
- Wayland: `xdotool` does not work on Wayland. For v1, return None for all fields on Wayland and log a warning. Wayland context capture is a future enhancement (would need `swaymsg` or similar).
