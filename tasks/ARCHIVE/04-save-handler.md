# Task 4: Build Save Handler (PNG Export + Sidecar JSON)

## Objective

When the user finishes annotating and hits the save button (or Enter), export two files to `~/.snap/inbox/`: the composited PNG and a sidecar JSON with structured annotation metadata.

## Context

Tasks 1-3 gave us a Tauri overlay with drawing tools and a captured screen background. This task adds the export pipeline.

## Steps

1. Add a **Save button** to the toolbar (checkmark icon or "Done" label). Also bind `Enter` key to save.

2. On save, the frontend does two things:

**A. Export the PNG:**
- Call `canvas.toDataURL('image/png')` to get the composited image (screen capture + annotations)
- Convert the base64 data to a Uint8Array
- Send to Rust backend via a Tauri command to write the file

```rust
#[tauri::command]
fn save_annotation(image_data: Vec<u8>, metadata_json: String) -> Result<String, String> {
    let inbox = dirs::home_dir()
        .ok_or("No home dir")?
        .join(".snap")
        .join("inbox");
    std::fs::create_dir_all(&inbox).map_err(|e| e.to_string())?;

    let timestamp = chrono::Utc::now().format("%Y%m%d-%H%M%S").to_string();
    let png_name = format!("snap-{}.png", timestamp);
    let json_name = format!("snap-{}.json", timestamp);

    let png_path = inbox.join(&png_name);
    let json_path = inbox.join(&json_name);

    std::fs::write(&png_path, image_data).map_err(|e| e.to_string())?;
    std::fs::write(&json_path, metadata_json).map_err(|e| e.to_string())?;

    Ok(png_path.to_string_lossy().to_string())
}
```

**B. Build the sidecar JSON:**
- Serialize the annotations array from the frontend
- Include source context (placeholder for now, Task 5 adds real window context)

```javascript
const metadata = {
  timestamp: new Date().toISOString(),
  source: {
    window_title: null,  // Task 5 fills this
    url: null,           // Task 5 fills this
    display: "primary",
    resolution: [window.screen.width, window.screen.height]
  },
  annotations: annotations.map(a => {
    // Map internal annotation objects to the sidecar schema
    switch (a.type) {
      case "circle":
        return { type: "circle", center: [a.cx, a.cy], radius: [a.rx, a.ry], color: a.color, label: null };
      case "arrow":
        return { type: "arrow", from: [a.fromX, a.fromY], to: [a.toX, a.toY], color: a.color, label: null };
      case "freehand":
        return { type: "freehand", points: a.points, color: a.color, label: null };
      case "text":
        return { type: "text", position: [a.x, a.y], content: a.content, color: a.color };
    }
  }),
  image_filename: null  // Set after we know the filename
};
```

3. After save completes:
   - Close the overlay window (`appWindow.close()`)
   - The user is back to their terminal/desktop immediately

4. Create the `~/.snap/inbox/` directory on first run if it does not exist (handle in Rust command).

5. Add `chrono` and `dirs` to Cargo.toml dependencies:
   ```toml
   [dependencies]
   chrono = "0.4"
   dirs = "5"
   ```

## File Output Example

After saving, `~/.snap/inbox/` contains:
```
snap-20260408-142300.png   (composited screenshot + annotations)
snap-20260408-142300.json  (structured metadata)
```

## Verification

- Draw some annotations, hit Enter or click Save
- Check `~/.snap/inbox/` for matching PNG and JSON files
- PNG shows the captured screen with annotations baked in
- JSON contains the correct annotation data with types, positions, colors
- Overlay closes immediately after save
- Saving with zero annotations still works (exports clean screenshot + empty annotations array)

## Do Not Touch

- Do not modify drawing tools from Task 2
- Do not modify screen capture from Task 3
- Do not add window context capture yet (Task 5)
- Do not wire up global hotkey yet (Task 6)
