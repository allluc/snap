#!/bin/bash
# setup-mcp.sh — Register the Snap MCP server with your AI coding tools.
# Run this after `make build` to connect Snap to your agents.

set -e

SNAP_DIR="$(cd "$(dirname "$0")" && pwd)"
PYTHON="$SNAP_DIR/mcp-server/.venv/bin/python"
SERVER="$SNAP_DIR/mcp-server/server.py"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo ""
echo -e "${BLUE}Snap MCP Setup${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Verify the server works
if [ ! -f "$PYTHON" ]; then
    echo -e "${YELLOW}Python venv not found. Run 'make build' first.${NC}"
    exit 1
fi

if ! "$PYTHON" -c "from server import mcp" 2>/dev/null; then
    echo -e "${YELLOW}MCP server failed to import. Run 'cd mcp-server && uv pip install -e .' first.${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} MCP server verified"
echo ""

# ── Claude Code ──────────────────────────────────────────────
setup_claude_code() {
    if command -v claude &>/dev/null; then
        claude mcp remove snap 2>/dev/null || true
        claude mcp add --transport stdio snap -- "$PYTHON" "$SERVER"
        echo -e "${GREEN}✓${NC} Claude Code — registered"
    else
        echo -e "${YELLOW}⊘${NC} Claude Code — not installed, skipping"
    fi
}

# ── Claude Desktop ───────────────────────────────────────────
setup_claude_desktop() {
    local config=""
    # Linux
    if [ -d "$HOME/.config/Claude" ]; then
        config="$HOME/.config/Claude/claude_desktop_config.json"
    fi
    # macOS
    if [ -d "$HOME/Library/Application Support/Claude" ]; then
        config="$HOME/Library/Application Support/Claude/claude_desktop_config.json"
    fi

    if [ -z "$config" ]; then
        echo -e "${YELLOW}⊘${NC} Claude Desktop — not installed, skipping"
        return
    fi

    # Read existing config or start fresh
    if [ -f "$config" ]; then
        existing=$(cat "$config")
    else
        existing='{}'
    fi

    # Merge snap server into the config using Python
    "$PYTHON" -c "
import json, sys

config = json.loads('''$existing''') if '''$existing'''.strip() else {}
if 'mcpServers' not in config:
    config['mcpServers'] = {}

config['mcpServers']['snap'] = {
    'command': '$PYTHON',
    'args': ['$SERVER']
}

with open('$config', 'w') as f:
    json.dump(config, f, indent=2)
"
    echo -e "${GREEN}✓${NC} Claude Desktop — configured at $config"
}

# ── Cursor ───────────────────────────────────────────────────
setup_cursor() {
    local config="$HOME/.cursor/mcp.json"
    local dir="$HOME/.cursor"

    if [ ! -d "$dir" ]; then
        echo -e "${YELLOW}⊘${NC} Cursor — not installed, skipping"
        return
    fi

    if [ -f "$config" ]; then
        existing=$(cat "$config")
    else
        existing='{}'
    fi

    "$PYTHON" -c "
import json

config = json.loads('''$existing''') if '''$existing'''.strip() else {}
if 'mcpServers' not in config:
    config['mcpServers'] = {}

config['mcpServers']['snap'] = {
    'command': '$PYTHON',
    'args': ['$SERVER']
}

with open('$config', 'w') as f:
    json.dump(config, f, indent=2)
"
    echo -e "${GREEN}✓${NC} Cursor — configured at $config"
}

# ── Windsurf ─────────────────────────────────────────────────
setup_windsurf() {
    local config="$HOME/.codeium/windsurf/mcp_config.json"
    local dir="$HOME/.codeium/windsurf"

    if [ ! -d "$dir" ]; then
        echo -e "${YELLOW}⊘${NC} Windsurf — not installed, skipping"
        return
    fi

    if [ -f "$config" ]; then
        existing=$(cat "$config")
    else
        existing='{}'
    fi

    "$PYTHON" -c "
import json

config = json.loads('''$existing''') if '''$existing'''.strip() else {}
if 'mcpServers' not in config:
    config['mcpServers'] = {}

config['mcpServers']['snap'] = {
    'command': '$PYTHON',
    'args': ['$SERVER']
}

with open('$config', 'w') as f:
    json.dump(config, f, indent=2)
"
    echo -e "${GREEN}✓${NC} Windsurf — configured at $config"
}

# ── Run setup ────────────────────────────────────────────────
echo "Detecting installed tools..."
echo ""

setup_claude_code
setup_claude_desktop
setup_cursor
setup_windsurf

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Done. Take a snap (Ctrl+Shift+S) and ask your agent:"
echo ""
echo -e "  ${BLUE}\"check my latest snap annotation\"${NC}"
echo ""
echo "Manual registration for other MCP clients:"
echo ""
echo "  command: $PYTHON"
echo "  args:    $SERVER"
echo ""
