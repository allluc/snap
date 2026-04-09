# Task 2.5: Rectangle Tool + Toolbar UX Polish

## Objective

Add a rectangle/box selection tool and polish the toolbar so it doesn't interfere with annotation work.

## Context

Task 2 built circle, arrow, freehand, and text tools. This adds the most-requested missing tool (rectangle for highlighting regions) and fixes UX issues that emerge once you start actually using the overlay.

## Steps

1. Add **Rectangle tool** to the toolbar:
   - Click-drag to draw a rectangle outline (no fill, or very low opacity fill like 10%)
   - Shift constrains to square
   - Keyboard shortcut: `R`
   - Data model:
   ```javascript
   { type: "rect", x: 100, y: 200, width: 300, height: 150, color: "#FF3B30", strokeWidth: 4 }
   ```

2. Add **Numbered marker tool** to the toolbar:
   - Click to place a circled number (auto-incrementing: 1, 2, 3...)
   - Rendered as a filled circle with the number centered inside, white text on colored background
   - Useful for "fix these 3 things" annotations where order matters
   - Keyboard shortcut: `N`
   - Data model:
   ```javascript
   { type: "marker", x: 340, y: 220, number: 1, color: "#FF3B30", radius: 16 }
   ```
   - Counter resets when annotations are cleared

3. Make the toolbar **draggable** so it doesn't cover the area you want to annotate:
   - Add a small drag handle (grip dots or a bar) on the left side of the toolbar
   - Mousedown on the handle + drag repositions the toolbar
   - Toolbar stays within viewport bounds

4. Add a **dim layer** toggle button on the toolbar:
   - When active, draws a semi-transparent dark overlay (`rgba(0,0,0,0.3)`) over the entire screen BEHIND the annotations
   - Makes annotations more visible against busy backgrounds
   - Toggle on/off with `D` key
   - Default: off

5. Update the sidecar JSON schema in Task 4's save handler to include the new annotation types:
   ```javascript
   // Rectangle
   { type: "rect", position: [x, y], size: [width, height], color: "#FF3B30", label: null }
   // Marker
   { type: "marker", position: [x, y], number: 1, color: "#FF3B30" }
   ```

## Verification

- Rectangle tool draws outlined boxes on the canvas
- Shift constrains rectangle to square
- Numbered markers auto-increment correctly
- Toolbar can be dragged to any position
- Dim layer toggle works and annotations remain fully visible on top
- All keyboard shortcuts work without conflict
- Undo works for rectangles and markers

## Do Not Touch

- Do not modify screen capture logic (Task 3)
- Do not modify save handler file format (just add new annotation type mappings)
- Do not modify the Tauri window configuration
