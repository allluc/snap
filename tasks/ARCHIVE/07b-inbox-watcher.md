# Task 7.5: Inbox Watcher with Notification Tool

## Objective

Add an MCP tool that tells Claude Code whether new annotations have arrived since it last checked. This enables a workflow where Claude Code periodically polls for new visual feedback without you explicitly telling it to look.

## Context

Task 7 built the core MCP server with read tools. But Claude Code has to be told "check my annotations." This task adds awareness so Claude Code can ask "is there anything new?" on its own.

## Steps

1. Add a state file that tracks the last-read timestamp:

```python
STATE_FILE = Path.home() / ".snap" / ".last_read"

def _get_last_read() -> float:
    """Return the timestamp of the last read, or 0 if never read."""
    if STATE_FILE.exists():
        return float(STATE_FILE.read_text().strip())
    return 0.0

def _mark_read():
    """Update the last-read timestamp to now."""
    STATE_FILE.parent.mkdir(parents=True, exist_ok=True)
    STATE_FILE.write_text(str(time.time()))
```

2. Add a new MCP tool to `server.py`:

```python
@mcp.tool
def check_new_annotations() -> dict:
    """Check if there are new screen annotations since the last time Claude Code looked. Returns the count of new annotations and their filenames. Call this periodically or at the start of a task to see if the user has provided visual feedback. Calling get_latest_annotation or list_annotations after this will mark them as read."""
    last_read = _get_last_read()
    files = _list_annotations_sorted()
    new_files = []
    for f in files:
        if f.stat().st_mtime > last_read:
            new_files.append(f.stem)
        else:
            break  # sorted newest first, so once we hit an old one, stop
    return {
        "new_count": len(new_files),
        "new_annotations": new_files,
        "has_new": len(new_files) > 0
    }
```

3. Update `get_latest_annotation` and `list_annotations` to call `_mark_read()` after returning data. This way, once Claude Code reads the annotations, they're marked as seen.

4. Add `time` to the imports in `server.py`.

## Verification

1. Create a test annotation in the inbox
2. Claude Code calls `check_new_annotations()` -- should return `has_new: true, new_count: 1`
3. Claude Code calls `get_latest_annotation()` -- returns the annotation and marks it read
4. Claude Code calls `check_new_annotations()` again -- should return `has_new: false, new_count: 0`
5. Create another test annotation
6. `check_new_annotations()` returns `has_new: true` again

## Do Not Touch

- Do not modify the existing tool signatures from Task 7
- Do not modify the Tauri app
- Only add to `server.py`, do not restructure it

## Notes

- This is the foundation for a future "auto-prompt" mode where Claude Code's CLAUDE.md includes an instruction like "at the start of every task, call check_new_annotations() to see if the user has visual feedback"
