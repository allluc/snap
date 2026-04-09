# Task 6.5: Auto-Start on Login + Systemd Service

## Objective

Make Snap start automatically when you log in so the hotkey is always available without manually launching the app.

## Context

Task 6 set up the global hotkey via a tray app. But if the app isn't running, the hotkey does nothing. Since you want to "walk away" and have this just work, it needs to auto-start.

## Steps

1. Create a systemd user service file:

```ini
# ~/.config/systemd/user/snap.service
[Unit]
Description=Snap Screen Annotation Tool
After=graphical-session.target

[Service]
Type=simple
ExecStart=%h/projects/snap/app/target/release/snap
Restart=on-failure
RestartSec=3
Environment=DISPLAY=:0

[Install]
WantedBy=default.target
```

2. Build the Tauri app in release mode first:

```bash
cd ~/projects/snap/app
cargo tauri build
```

The binary lands in `target/release/snap` (or whatever the binary name is from `tauri.conf.json`).

3. Enable and start the service:

```bash
systemctl --user daemon-reload
systemctl --user enable snap.service
systemctl --user start snap.service
```

4. Verify the service is running:

```bash
systemctl --user status snap.service
```

5. Add convenience commands to a Makefile or shell aliases:

```makefile
# ~/projects/snap/Makefile

start:
	systemctl --user start snap.service

stop:
	systemctl --user stop snap.service

restart:
	systemctl --user restart snap.service

logs:
	journalctl --user -u snap.service -f

status:
	systemctl --user status snap.service

install:
	cargo tauri build
	cp ~/.config/systemd/user/snap.service ~/.config/systemd/user/snap.service
	systemctl --user daemon-reload
	systemctl --user enable snap.service
	systemctl --user start snap.service
```

## Verification

- Log out and log back in (or reboot)
- Without manually starting anything, press the global hotkey
- Overlay should appear
- `systemctl --user status snap.service` shows active (running)

## Do Not Touch

- Do not modify the Tauri app code
- Do not modify the MCP server
- This is purely deployment/service configuration

## Notes

- If using Wayland, you may need `Environment=WAYLAND_DISPLAY=wayland-0` instead of `DISPLAY=:0`
- The service restarts on failure with a 3-second delay so a crash doesn't leave you without the hotkey
