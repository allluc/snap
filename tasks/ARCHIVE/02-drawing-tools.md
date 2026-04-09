# Task 2: Build Canvas Drawing Tools

## Objective

Implement the annotation drawing tools on the transparent overlay canvas. The user needs to circle things, draw arrows, freehand scribble, and add text labels.

## Context

Task 1 created a Tauri app with a transparent fullscreen overlay. This task replaces the test rectangle with a real drawing surface and toolbar.

## Steps

1. Replace the test content in `index.html` with:
   - A fullscreen `<canvas>` element, transparent background, covering the entire viewport
   - A minimal floating toolbar anchored to the top-center of the screen
   - Toolbar background: `rgba(30, 30, 30, 0.9)` with `border-radius: 8px` and `padding: 8px`

2. Implement the toolbar with these tools (use simple SVG icons or unicode symbols):
   - **Circle tool** (default): Click-drag to draw an ellipse. Shift constrains to circle.
   - **Arrow tool**: Click start point, drag to end point. Arrowhead on the end.
   - **Freehand tool**: Click-drag draws a smoothed path. Apply basic Catmull-Rom or simple point averaging for stroke stabilization.
   - **Text tool**: Click to place a text insertion point. Type to add label. Enter confirms. Simple, no rich text.
   - **Color picker**: 6 preset swatches (red `#FF3B30`, blue `#007AFF`, green `#34C759`, yellow `#FFCC00`, white `#FFFFFF`, black `#000000`). Red is default. No custom color dialog needed.
   - **Stroke width**: 3 presets (thin: 2px, medium: 4px, thick: 6px). Medium is default.
   - **Undo button**: Pops the last annotation from the stack and redraws.
   - **Clear button**: Removes all annotations.

3. Data model for annotations. Maintain an array of annotation objects in memory:

```javascript
const annotations = [];

// Circle example
{ type: "circle", cx: 340, cy: 220, rx: 45, ry: 45, color: "#FF3B30", strokeWidth: 4 }

// Arrow example
{ type: "arrow", fromX: 100, fromY: 200, toX: 300, toY: 250, color: "#FF3B30", strokeWidth: 4 }

// Freehand example
{ type: "freehand", points: [{x: 10, y: 20}, ...], color: "#FF3B30", strokeWidth: 4 }

// Text example
{ type: "text", x: 350, y: 310, content: "fix this", color: "#FF3B30", fontSize: 16 }
```

4. Rendering approach:
   - On every draw action, clear the canvas and re-render all annotations from the array
   - This makes undo trivial (pop from array, re-render)
   - Use `canvas.getContext('2d')` for all rendering
   - Arrow heads: filled triangle, 10px size, rotated to match arrow direction

5. Keyboard shortcuts:
   - `C` switches to circle tool
   - `A` switches to arrow tool
   - `F` switches to freehand tool
   - `T` switches to text tool
   - `Ctrl+Z` undo
   - `Escape` cancels current draw action (if mid-draw) or closes overlay (if idle)

6. Active tool should be visually highlighted in the toolbar (brighter background or underline).

## Verification

- All four tools draw correctly on the transparent canvas
- Annotations render on top of visible desktop content
- Color picker changes the color of subsequent annotations
- Stroke width changes the thickness of subsequent annotations
- Undo removes the last annotation
- Clear removes all annotations
- Keyboard shortcuts work
- Text tool allows typing and renders the label at click position

## Do Not Touch

- Do not implement save/export yet (Task 4)
- Do not capture the screen behind the overlay yet (Task 3)
- Do not add the sidecar JSON export yet (Task 4)
- Do not modify `tauri.conf.json` window settings from Task 1

## Notes

- The canvas is transparent, so the user draws directly "on top of" whatever they see on screen. The annotations float over the desktop.
- Text rendering: use `ctx.fillText()` with a slight dark shadow/outline (`ctx.strokeText()` in black first, then `ctx.fillText()` in the selected color) so text is readable on any background.
