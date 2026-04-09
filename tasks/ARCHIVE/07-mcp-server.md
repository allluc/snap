# Task 7: Build the MCP Server

## Objective

Create a Python MCP server using `fastmcp` that exposes the annotation inbox to Claude Code. Stdio transport. Claude Code calls these tools to read annotated screenshots.

## Location

`~/projects/snap/mcp-server/`

## Steps

1. Create the project structure:

```
mcp-server/
  server.py
  pyproject.toml
  README.md
```

2. `pyproject.toml`:

```toml
[project]
name = "snap-server"
version = "0.1.0"
requires-python = ">=3.11"
dependencies = ["fastmcp>=2.0.0"]

[project.scripts]
snap = "server:main"
```

3. `server.py` -- the entire MCP server:

```python
import json
import os
from pathlib import Path
from fastmcp import FastMCP

mcp = FastMCP(name="snap")

INBOX = Path.home() / ".snap" / "inbox"


def _load_annotation(json_path: Path) -> dict:
    """Load a sidecar JSON and attach the image path."""
    with open(json_path) as f:
        data = json.load(f)
    png_path = json_path.with_suffix(".png")
    data["image_path"] = str(png_path) if png_path.exists() else None
    data["filename"] = json_path.stem
    return data


def _list_annotations_sorted() -> list[Path]:
    """Return all sidecar JSON files sorted newest first."""
    if not INBOX.exists():
        return []
    jsons = sorted(INBOX.glob("snap-*.json"), reverse=True)
    return jsons


@mcp.tool
def get_latest_annotation() -> dict:
    """Get the most recent screen annotation. Returns the image path and structured metadata including annotation positions, labels, colors, and source window context. Claude Code should read the image at the returned image_path to see the annotated screenshot."""
    files = _list_annotations_sorted()
    if not files:
        return {"error": "No annotations in inbox"}
    return _load_annotation(files[0])


@mcp.tool
def list_annotations(last_n: int = 5) -> list[dict]:
    """List recent screen annotations. Returns metadata for the N most recent annotations, newest first. Each entry includes the image_path that Claude Code can read to see the annotated screenshot."""
    files = _list_annotations_sorted()[:last_n]
    return [_load_annotation(f) for f in files]


@mcp.tool
def get_annotation(filename: str) -> dict:
    """Get a specific annotation by filename (without extension). Example: get_annotation('snap-20260408-142300')"""
    json_path = INBOX / f"{filename}.json"
    if not json_path.exists():
        return {"error": f"Annotation {filename} not found"}
    return _load_annotation(json_path)


@mcp.tool
def clear_inbox() -> dict:
    """Delete all processed annotations from the inbox. Use after Claude Code has addressed all pending annotations."""
    files = _list_annotations_sorted()
    png_count = 0
    json_count = 0
    for json_path in files:
        png_path = json_path.with_suffix(".png")
        if png_path.exists():
            png_path.unlink()
            png_count += 1
        json_path.unlink()
        json_count += 1
    return {"deleted_json": json_count, "deleted_png": png_count}


def main():
    mcp.run(transport="stdio")


if __name__ == "__main__":
    main()
```

4. Register with Claude Code:

```bash
claude mcp add --transport stdio snap -- python ~/projects/snap/mcp-server/server.py
```

Or if using uv:

```bash
claude mcp add --transport stdio snap -- uv run --directory ~/projects/snap/mcp-server python server.py
```

## Verification

1. Create a fake annotation for testing:

```bash
mkdir -p ~/.snap/inbox
echo '{"timestamp":"2026-04-08T14:00:00Z","source":{"window_title":"test"},"annotations":[]}' > ~/.snap/inbox/snap-20260408-140000.json
touch ~/.snap/inbox/snap-20260408-140000.png
```

2. Register the MCP server with Claude Code
3. Open Claude Code and run: "use the snap tools to get the latest annotation"
4. Claude Code should call `get_latest_annotation` and return the test data
5. Run: "list all annotations" -- should call `list_annotations`
6. Run: "clear the snap inbox" -- should call `clear_inbox` and delete the test files

## Do Not Touch

- Do not modify the Tauri app (Tasks 1-6)
- Do not change the sidecar JSON schema from Task 4
- This server is read-only against the inbox. It never writes annotations, only reads and deletes.
