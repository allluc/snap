# Task 11: README, License, and Open-Source Packaging

## Objective

Package Snap for public release as an Adjective open-source project. README, license, install instructions, and a clean repo structure.

## Steps

1. Create the top-level repo structure:

```
snap/
  app/                  # Tauri annotation overlay (from Tasks 1-6)
  mcp-server/           # Python MCP server (from Task 7)
  README.md
  LICENSE
  Makefile
  .gitignore
```

2. Write `README.md`:

```markdown
# Snap

Visual annotation layer for agentic coding. Annotate your screen and feed marked-up screenshots directly into Claude Code (or any MCP-compatible agent).

## What it does

1. Press a hotkey from anywhere
2. Your screen freezes into an annotation overlay
3. Circle, arrow, highlight, and label the things you want fixed
4. Hit enter. The annotated screenshot + structured metadata drops into an inbox
5. Your coding agent reads the annotation via MCP and makes the fix

## Install

### Prerequisites

- Linux (X11). Wayland partial support.
- Rust toolchain
- Python 3.11+
- `scrot` (X11) or `grim` (Wayland): `sudo apt install scrot`
- `xdotool`: `sudo apt install xdotool`

### Build the annotation app

```bash
cd app
cargo tauri build
```

### Install the MCP server

```bash
cd mcp-server
pip install -e .
```

### Register with Claude Code

```bash
claude mcp add --transport stdio snap -- python /path/to/snap/mcp-server/server.py
```

### Auto-start (optional)

```bash
make install  # sets up systemd user service
```

## Usage

| Hotkey | Action |
|--------|--------|
| Ctrl+Shift+S | Open annotation overlay |
| C | Circle tool |
| R | Rectangle tool |
| A | Arrow tool |
| F | Freehand tool |
| T | Text tool |
| N | Numbered marker |
| D | Toggle dim layer |
| Ctrl+Z | Undo |
| Enter | Save and close |
| Escape | Cancel and close |

## MCP Tools

| Tool | Description |
|------|-------------|
| check_new_annotations() | Are there new annotations since last check? |
| get_latest_annotation() | Get most recent annotation path + metadata |
| list_annotations(last_n) | List recent annotations |
| get_annotation(filename) | Get specific annotation |
| clear_inbox() | Delete processed annotations |

## How agents interpret annotations

- Red circles/rectangles = problem area
- Arrows = move/connect direction
- Text labels = literal instructions
- Numbered markers = ordered issue list
- Freehand = emphasis

## Built by

[Adjective LLC](https://adjective.us) -- Infinite Intelligence
```

3. Add `LICENSE` file: MIT license, copyright Adjective LLC 2026.

4. Write `.gitignore`:

```
# Rust
app/target/
app/src-tauri/target/

# Python
mcp-server/__pycache__/
mcp-server/*.egg-info/
mcp-server/.venv/

# OS
.DS_Store
*.swp

# Snap runtime
.snap/
```

5. Top-level `Makefile`:

```makefile
.PHONY: build install start stop restart logs status clean

build:
	cd app && cargo tauri build
	cd mcp-server && pip install -e .

install: build
	mkdir -p ~/.config/systemd/user
	cp app/snap.service ~/.config/systemd/user/
	systemctl --user daemon-reload
	systemctl --user enable snap.service
	systemctl --user start snap.service
	@echo "Snap installed and running."
	@echo "Register with Claude Code:"
	@echo "  claude mcp add --transport stdio snap -- python $(PWD)/mcp-server/server.py"

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

clean:
	rm -rf app/target app/src-tauri/target
	rm -rf mcp-server/__pycache__ mcp-server/*.egg-info
```

6. Initialize git repo, make initial commit:

```bash
cd ~/projects/snap
git init
git add .
git commit -m "snap: visual annotation layer for agentic coding"
```

7. Create GitHub repo under the Adjective org (or personal account) and push.

## Verification

- `make build` completes without errors
- `make install` sets up the systemd service and it starts
- README renders correctly on GitHub
- A new user could follow the README and get it running

## Do Not Touch

- Do not modify any app or server code
- This is packaging and documentation only
