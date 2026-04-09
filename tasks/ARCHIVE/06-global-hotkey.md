# Task 6: Global Hotkey Registration

## Objective

Register a system-wide hotkey (Ctrl+Shift+S) that launches the Snap overlay from anywhere, without the app window needing to be focused or visible.

## Context

Tasks 1-5 built the overlay app that captures, annotates, and saves. But right now you have to manually run `cargo tauri dev` or launch the binary to open it. This task makes it launch with a hotkey from anywhere on the desktop.

## Approach

There are two ways to do this. Use **Option A** for v1 as it's simpler:

### Option A: Background tray app (recommended for v1)

Convert the Tauri app into a system tray application that runs in the background. The tray icon is optional but the process stays alive. On hotkey press, it opens the overlay window.

1. Add the Tauri tray plugin and global shortcut plugin:

```toml
# src-tauri/Cargo.toml
[dependencies]
tauri-plugin-global-shortcut = "2"
```

2. In the Tauri setup, register the global shortcut:

```rust
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let shortcut = Shortcut::new(
                Some(Modifiers::CONTROL | Modifiers::SHIFT),
                Code::KeyS
            );

            app.global_shortcut().on_shortcut(shortcut, |app, _shortcut, _event| {
                // Create and show a new overlay window
                // OR if the window exists but is hidden, show it
                // The window creation triggers the capture_screen + get_active_window_context flow
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

3. Modify `tauri.conf.json`:
   - Keep the window hidden on startup (`visible: false`)
   - The app starts with no visible window, just the hotkey listener
   - On hotkey, create/show the overlay window
   - On save or escape, hide/destroy the window but keep the app process running

4. The main window should NOT appear on launch. The app starts silently, registers the hotkey, and waits.

5. Add a tray icon so the user can quit the app:
   ```rust
   // Simple tray with just a Quit option
   ```

### Option B: Standalone daemon + launcher (future)

A separate lightweight process that listens for the hotkey and spawns the Tauri app as a child process. More complex, save for later.

## Verification

- Build and run the app: it starts with no visible window
- Press `Ctrl+Shift+S` from any application (browser, terminal, file manager)
- The overlay appears with the screen capture
- Draw annotations, save, overlay closes
- Press `Ctrl+Shift+S` again. Overlay appears again (new capture).
- Right-click tray icon, select Quit. Process exits.

## Do Not Touch

- Do not modify drawing tools from Task 2
- Do not modify screen capture from Task 3
- Do not modify save handler from Task 4
- Do not modify window context from Task 5

## Conflict Note

`Ctrl+Shift+S` may conflict with "Save As" in some applications. If this is a problem during testing, change to `Ctrl+Shift+X` or `Super+Shift+S`. The exact binding is easy to change later. Pick one that doesn't conflict with your common apps.

## Dependencies

- `tauri-plugin-global-shortcut` v2
