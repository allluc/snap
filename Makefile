.PHONY: build install start stop restart logs status clean dev deps setup-mcp

# Install system dependencies (Ubuntu/Debian)
deps:
	sudo apt install -y scrot xdotool pkg-config libwebkit2gtk-4.1-dev \
		build-essential libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev

build:
	cd app && npm install && cargo tauri build
	cd mcp-server && uv venv .venv && uv pip install -e .

dev:
	cd app && cargo tauri dev

install: build
	mkdir -p ~/.config/systemd/user
	cp snap.service ~/.config/systemd/user/
	sed -i "s|%h/Desktop/snap|$(PWD)|g" ~/.config/systemd/user/snap.service
	systemctl --user daemon-reload
	systemctl --user enable snap.service
	systemctl --user start snap.service
	@echo ""
	@echo "Snap installed and running."
	@echo "Register with Claude Code:"
	@echo "  claude mcp add --transport stdio snap -- $(PWD)/mcp-server/.venv/bin/python $(PWD)/mcp-server/server.py"

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

setup-mcp:
	./setup-mcp.sh

clean:
	rm -rf app/src-tauri/target app/node_modules
	rm -rf mcp-server/__pycache__ mcp-server/*.egg-info
