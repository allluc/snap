# Task 9: CLAUDE.md Integration for Snap Awareness

## Objective

Add instructions to your project-level CLAUDE.md files so Claude Code automatically checks for visual annotations at the start of tasks and knows how to interpret them.

## Context

The MCP server is running. The tools work. But Claude Code won't use them unless it knows they exist and when to call them. This task wires the awareness into your development workflow.

## Steps

1. Add a Snap section to your global `~/.claude/CLAUDE.md` (or create it if it doesn't exist):

```markdown
## Snap -- Visual Annotations

You have access to the `snap` MCP tools. These let you see annotated screenshots the user has created.

### Tools available:
- `check_new_annotations()` -- call at the start of every task to see if there's visual feedback
- `get_latest_annotation()` -- returns the most recent annotated screenshot path + metadata
- `list_annotations(last_n)` -- returns recent annotations
- `get_annotation(filename)` -- returns a specific annotation
- `clear_inbox()` -- deletes processed annotations

### When to use:
- At the START of any UI/frontend task, call `check_new_annotations()` first
- If the user says "look at my annotation" or "check my snap" or "I marked something up", call `get_latest_annotation()`
- After getting annotation metadata, READ THE IMAGE at the `image_path` to see the annotated screenshot
- The sidecar JSON contains structured annotation data: positions, labels, colors, and source window context
- Use `source.window_title` to infer which page/component the user was looking at

### How to interpret annotations:
- Red circles/rectangles = "this area has a problem"
- Arrows = "this should move/connect there"
- Text labels = literal instructions ("fix this", "16px gap", "wrong color")
- Numbered markers = ordered list of issues (fix #1 first, then #2, etc.)
- Freehand marks = emphasis, "look at this general area"

### After addressing annotations:
- Tell the user what you fixed based on each annotation
- Reference the annotation numbers if numbered markers were used
- Call `clear_inbox()` only if the user confirms the fixes are good
```

2. For project-specific CLAUDE.md files (e.g., in your Adjective site repo or Girolamo repos), add a shorter version:

```markdown
## Visual Feedback
This project uses snap for visual annotations. Call `check_new_annotations()` at the start of UI tasks. See global CLAUDE.md for full instructions.
```

## Verification

1. Open Claude Code in any project
2. Say "start working on the homepage layout"
3. Claude Code should call `check_new_annotations()` before doing anything else
4. If there are annotations, it should read them and incorporate the visual feedback
5. If no annotations, it proceeds normally

## Do Not Touch

- Do not modify the MCP server code
- Do not modify the Tauri app
- This is purely documentation/configuration for Claude Code's behavior

## Notes

- The global CLAUDE.md approach means this works across ALL your projects without per-repo setup
- The interpretation guide ("red circles = problem area") is critical. Without it, Claude Code sees pixels but doesn't know the visual language.
