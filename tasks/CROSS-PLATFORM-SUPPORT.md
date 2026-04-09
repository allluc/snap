# Cross-Platform Support: macOS and Windows

## Context

The Snap app was built Linux-first (X11). This file contains tasks to make it work natively on macOS and Windows. The Tauri shell (window, canvas, toolbar, drawing tools, MCP server) is already cross-platform. The platform-specific parts are:

1. Screen capture (subprocess command differs per OS)
2. Window context capture (xdotool is Linux-only)
3. Auto-start mechanism (systemd is Linux-only)
4. File paths and conventions

Claude Code should detect the host platform and implement accordingly. The Rust code already has a platform detection pattern in `lib.rs`. Extend it.

---

## Task A: Cross-Platform Screen Capture

### What exists

`capture_screen()` in `app/src-tauri/src/lib.rs` currently shells out to `scrot` (Linux X11) or `grim` (Linux Wayland).

### What to add

Extend the function with platform branches:

**macOS:**
- Use `screencapture -x <path>` 
- The `-x` flag suppresses the screenshot sound
- For retina displays, the capture will be 2x resolution. This is fine. The canvas will scale it down via `drawImage`.
- `screencapture` is pre-installed on all macOS versions. No dependency needed.

**Windows:**
- Use PowerShell inline:
  ```
  powershell -Command "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.Screen]::PrimaryScreen | ForEach-Object { $bitmap = New-Object System.Drawing.Bitmap($_.Bounds.Width, $_.Bounds.Height); $graphics = [System.Drawing.Graphics]::FromImage($bitmap); $graphics.CopyFromScreen($_.Bounds.Location, [System.Drawing.Point]::Empty, $_.Bounds.Size); $bitmap.Save('<path>'); }"
  ```
- Alternative: use the `screenshots` Rust crate instead of subprocess. This is cleaner on Windows and avoids PowerShell complexity. Add `screenshots = "0.8"` to Cargo.toml and use it as the Windows backend. Can also be used as the macOS backend if preferred.
- If using the `screenshots` crate, it works on all three platforms and could replace the subprocess approach entirely. Claude Code should evaluate whether a unified Rust-native capture is simpler than three platform-specific subprocess calls.

### Implementation guidance

Use `std::env::consts::OS` for detection:

```rust
match std::env::consts::OS {
    "linux" => { /* existing scrot/grim logic */ },
    "macos" => { /* screencapture -x */ },
    "windows" => { /* screenshots crate or PowerShell */ },
    other => Err(format!("Unsupported platform: {}", other)),
}
```

### Verification

- On macOS: `cargo tauri dev`, overlay appears with captured screen behind it
- On Windows: same
- Captured image matches actual screen content
- No audible shutter sound on macOS

---

## Task B: Cross-Platform Window Context

### What exists

`get_active_window_context()` in `lib.rs` uses `xdotool` to get window title, class, and PID on Linux X11. Returns None on Wayland.

### What to add

**macOS:**
- Window title and app name via AppleScript (`osascript`):
  ```
  osascript -e 'tell application "System Events"
    set frontApp to first application process whose frontmost is true
    set appName to name of frontApp
    set windowTitle to ""
    try
      set windowTitle to name of front window of frontApp
    end try
    return appName & "||" & windowTitle
  end tell'
  ```
- Parse the output by splitting on `||`
- Map `window_class` to the app name (e.g., "Google Chrome", "Safari", "Firefox")
- Browser detection: check if app name contains "Chrome", "Safari", "Firefox", "Brave", "Arc", "Edge"
- PID: `osascript -e 'tell application "System Events" to get unix id of first application process whose frontmost is true'`

**Windows:**
- Use PowerShell to get the foreground window info:
  ```
  powershell -Command "Add-Type -TypeDefinition 'using System; using System.Runtime.InteropServices; public class WinAPI { [DllImport(\"user32.dll\")] public static extern IntPtr GetForegroundWindow(); [DllImport(\"user32.dll\", SetLastError=true, CharSet=CharSet.Auto)] public static extern int GetWindowText(IntPtr hWnd, System.Text.StringBuilder lpString, int nMaxCount); [DllImport(\"user32.dll\")] public static extern uint GetWindowThreadProcessId(IntPtr hWnd, out uint lpdwProcessId); }'; $hwnd = [WinAPI]::GetForegroundWindow(); $sb = New-Object System.Text.StringBuilder(256); [WinAPI]::GetWindowText($hwnd, $sb, 256); $pid = 0; [WinAPI]::GetWindowThreadProcessId($hwnd, [ref]$pid); Write-Output \"$($sb.ToString())||$pid\""
  ```
- Alternative: use the `windows` Rust crate for native Win32 API calls. Cleaner than PowerShell.
- Claude Code should pick whichever approach compiles cleanly and doesn't add heavy dependencies.

### Implementation guidance

Same pattern as Task A: branch on `std::env::consts::OS` inside `get_active_window_context()`.

Browser detection logic (checking window class/app name for browser identifiers) should be shared across platforms. Extract the browser name list into a helper function.

### Verification

- On macOS: open Safari to a URL, trigger Snap, save. Sidecar JSON shows Safari window title and app name.
- On Windows: open Chrome to a URL, trigger Snap, save. Sidecar JSON shows Chrome window title.
- Non-browser windows still capture title and PID correctly.

---

## Task C: Cross-Platform Auto-Start

### What exists

`snap.service` is a systemd user service (Linux only). `Makefile` has install/start/stop targets that use `systemctl`.

### What to add

**macOS: Launch Agent**

Create `com.adjective.snap.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.adjective.snap</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Applications/Snap.app/Contents/MacOS/snap</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <dict>
        <key>SuccessfulExit</key>
        <false/>
    </dict>
</dict>
</plist>
```

Install to `~/Library/LaunchAgents/com.adjective.snap.plist`.

Commands:
- Start: `launchctl load ~/Library/LaunchAgents/com.adjective.snap.plist`
- Stop: `launchctl unload ~/Library/LaunchAgents/com.adjective.snap.plist`
- Status: `launchctl list | grep snap`

**Windows: Startup Folder or Registry**

Simplest approach: place a shortcut to the Snap executable in the user's Startup folder:
`%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\`

Alternative: registry key at `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run`.

Claude Code should use whichever approach is simpler to implement from Rust/PowerShell.

**Makefile updates:**

Add platform-aware install/start/stop targets. Detect OS and dispatch:

```makefile
UNAME := $(shell uname -s)

install:
ifeq ($(UNAME), Linux)
	# existing systemd logic
endif
ifeq ($(UNAME), Darwin)
	cp com.adjective.snap.plist ~/Library/LaunchAgents/
	launchctl load ~/Library/LaunchAgents/com.adjective.snap.plist
endif
```

Windows users will likely use the installer that `cargo tauri build` produces, which can include auto-start configuration via the bundler settings in `tauri.conf.json`.

### Verification

- macOS: reboot, Snap runs automatically, hotkey works without manual launch
- Windows: reboot, same behavior
- Stop/start commands work on each platform

---

## Task D: Cross-Platform File Paths

### What exists

Runtime directory is `~/.snap/` with `inbox/`, `.last_read`, and `snap.log` inside it.

### What to change

`~/.snap/` works on Linux and macOS. On Windows, `~` expands to `C:\Users\<name>` which is fine, but the conventional location would be `%LOCALAPPDATA%\snap\` or `%APPDATA%\snap\`.

**Rust side:**
The `dirs` crate's `home_dir()` works cross-platform. Current code uses `dirs::home_dir().join(".snap")`. This resolves correctly on all platforms:
- Linux: `/home/user/.snap/`
- macOS: `/Users/user/.snap/`
- Windows: `C:\Users\user\.snap\`

This is acceptable for v1. If Claude Code wants to be more Windows-conventional, use `dirs::data_local_dir()` which gives `%LOCALAPPDATA%` on Windows, `~/.local/share` on Linux, `~/Library/Application Support` on macOS.

Decision: keep `~/.snap/` for simplicity and cross-platform consistency. It works everywhere.

**Python MCP server side:**
`Path.home() / ".snap"` works cross-platform in Python. No changes needed.

**Temp capture path:**
Currently `/tmp/snap-capture.png`. On Windows, use `std::env::temp_dir()` instead of hardcoding `/tmp/`:

```rust
let path = std::env::temp_dir().join("snap-capture.png");
```

This should be applied on all platforms for consistency.

### Verification

- On each platform, check that `~/.snap/inbox/` is created on first save
- Temp capture file lands in the correct temp directory
- MCP server reads from the correct inbox path

---

## Task E: macOS Permissions and Entitlements

### macOS-specific concern

Screen capture and accessibility (for window context) require explicit user permission on macOS.

**Screen Recording permission:**
- `screencapture` triggers the "Screen Recording" permission dialog on first use
- The app needs to be listed in System Settings > Privacy & Security > Screen Recording
- Tauri apps request this automatically when calling `screencapture`, but the user has to approve it

**Accessibility permission (for AppleScript window context):**
- AppleScript `tell application "System Events"` requires Accessibility permission
- System Settings > Privacy & Security > Accessibility
- The user will be prompted on first use

**Entitlements:**
Add to the Tauri macOS bundle configuration in `tauri.conf.json` under `bundle > macOS > entitlements`:

```
com.apple.security.temporary-exception.apple-events (for AppleScript)
```

Or create an `Entitlements.plist` in `src-tauri/`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.app-sandbox</key>
    <false/>
</dict>
</plist>
```

Disabling the sandbox is the simplest path for a developer tool that needs screen capture and accessibility. Not appropriate for App Store distribution, but fine for direct download / Homebrew.

### What to document

Add a section to README.md under Install > macOS:

```
### macOS permissions

On first launch, macOS will ask for:
- **Screen Recording** permission (required for screen capture)
- **Accessibility** permission (required for detecting the active window)

Grant both in System Settings > Privacy & Security. You may need to restart Snap after granting permissions.
```

### Verification

- Fresh macOS install: launch Snap, permission dialogs appear
- After granting permissions: capture and window context work correctly
- Without permissions: app shows a clear error, does not crash

---

## Task F: Windows-Specific Considerations

### Transparent window on Windows

Tauri transparent windows on Windows require the WebView2 runtime (pre-installed on Windows 10 21H2+ and Windows 11). Older Windows 10 builds may need the WebView2 bootstrapper bundled with the installer.

Tauri's bundler handles this automatically when using `cargo tauri build`. No code changes needed, but test on a clean Windows VM if possible.

### Global hotkey on Windows

`tauri-plugin-global-shortcut` works on Windows. No changes needed.

`Ctrl+Shift+S` may conflict with "Save As" in some Windows apps. Same consideration as Linux. Document the conflict and note how to rebind.

### Windows installer

`cargo tauri build` on Windows produces an MSI installer and/or NSIS installer. The bundler config in `tauri.conf.json` controls this. Default settings are fine for v1.

### Verification

- `cargo tauri build` on Windows produces a working installer
- Installing and running shows the tray icon
- Global hotkey works from any application
- Screen capture, annotation, save, and MCP server all work

---

## Execution Order

These tasks can run in any order since they're all modifying platform branches in the same files. However, the recommended sequence:

1. **Task D** first (file paths) -- smallest change, unblocks everything else
2. **Task A** (screen capture) -- core functionality
3. **Task B** (window context) -- nice to have, not blocking
4. **Task C** (auto-start) -- polish
5. **Task E** (macOS permissions) -- macOS only, do when testing on Mac
6. **Task F** (Windows considerations) -- Windows only, do when testing on Windows

If Claude Code is running on macOS, do A+B+D+E in one session. If on Windows, do A+B+D+F in one session. Task C (auto-start) is a separate session on each platform.

## Notes for Claude Code

- The existing Linux code must continue to work. Do not break it while adding platform branches.
- Use `#[cfg(target_os = "...")]` for compile-time platform branching where appropriate, or runtime `std::env::consts::OS` checks for subprocess-based logic. Either approach is fine. Pick whichever is cleaner for each specific case.
- If the `screenshots` Rust crate works well on all three platforms, it can replace all three subprocess capture methods. Test it first on the current platform before ripping out the subprocess code.
- The MCP server (Python) should work on all platforms without changes beyond the temp path fix. Verify it does.