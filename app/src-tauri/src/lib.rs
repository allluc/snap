use base64::Engine;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

pub static OVERLAY_ACTIVE: AtomicBool = AtomicBool::new(false);

// ----- Screen Capture -----

#[tauri::command]
fn capture_screen() -> Result<String, String> {
    let path = "/tmp/snap-capture.png";

    // Clean up previous capture
    let _ = fs::remove_file(path);

    // Try capture methods in order of preference
    let methods: Vec<(&str, Vec<&str>)> = if std::env::var("WAYLAND_DISPLAY").is_ok() {
        vec![
            // GNOME Wayland: gnome-screenshot is the most reliable
            ("gnome-screenshot", vec!["--file", path]),
            // wlroots-based compositors (Sway, Hyprland, etc.)
            ("grim", vec![path]),
            // Fallback: scrot might work via XWayland
            ("scrot", vec!["--overwrite", path]),
        ]
    } else {
        vec![
            ("scrot", vec!["--overwrite", path]),
            ("gnome-screenshot", vec!["--file", path]),
        ]
    };

    let mut last_error = String::from("No screenshot tool found");

    for (tool, args) in &methods {
        match Command::new(tool).args(args).output() {
            Ok(output) if output.status.success() => {
                match fs::metadata(path) {
                    Ok(meta) if meta.len() > 0 => {
                        log_event(&format!("screen captured via {}", tool));
                        return Ok(path.to_string());
                    }
                    _ => {
                        last_error = format!("{} produced empty file", tool);
                        continue;
                    }
                }
            }
            Ok(output) => {
                last_error = format!(
                    "{} failed: {}",
                    tool,
                    String::from_utf8_lossy(&output.stderr)
                );
                continue;
            }
            Err(_) => continue, // tool not installed, try next
        }
    }

    Err(format!(
        "Screen capture failed: {}. Install one of: sudo apt install gnome-screenshot grim scrot",
        last_error
    ))
}

// ----- Window Context -----

#[derive(Serialize)]
struct WindowContext {
    window_title: Option<String>,
    url: Option<String>,
    window_class: Option<String>,
    pid: Option<u32>,
}

#[tauri::command]
fn get_active_window_context() -> Result<WindowContext, String> {
    // Only works on X11
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        return Ok(WindowContext {
            window_title: None,
            url: None,
            window_class: None,
            pid: None,
        });
    }

    let title = run_xdotool(&["getactivewindow", "getwindowname"]);
    let class = run_xdotool(&["getactivewindow", "getwindowclassname"]);
    let pid_str = run_xdotool(&["getactivewindow", "getwindowpid"]);
    let pid = pid_str.as_ref().and_then(|s| s.trim().parse::<u32>().ok());

    // Try to infer URL from browser window titles
    let url = title.as_ref().and_then(|t| {
        let is_browser = class
            .as_ref()
            .map(|c| {
                let lower = c.to_lowercase();
                lower.contains("brave")
                    || lower.contains("firefox")
                    || lower.contains("chrom")
                    || lower.contains("webkit")
            })
            .unwrap_or(false);

        if is_browser {
            Some(t.clone())
        } else {
            None
        }
    });

    Ok(WindowContext {
        window_title: title,
        url,
        window_class: class,
        pid,
    })
}

fn run_xdotool(args: &[&str]) -> Option<String> {
    Command::new("xdotool")
        .args(args)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
}

// ----- Save Annotation -----

#[tauri::command]
fn save_annotation(metadata_json: String, image_base64: Option<String>) -> Result<String, String> {
    let inbox = inbox_dir()?;

    let timestamp = chrono::Utc::now().format("%Y%m%d-%H%M%S%.3f").to_string();
    let timestamp = timestamp.replace('.', "-");
    let png_name = format!("snap-{}.png", timestamp);
    let json_name = format!("snap-{}.json", timestamp);

    let png_path = inbox.join(&png_name);
    let json_path = inbox.join(&json_name);

    // If base64 image data provided, decode and write it
    // Otherwise, copy the screen capture file directly
    if let Some(b64) = image_base64 {
        let image_bytes = base64::engine::general_purpose::STANDARD
            .decode(&b64)
            .map_err(|e| format!("Base64 decode failed: {}", e))?;
        fs::write(&png_path, &image_bytes).map_err(|e| format!("Failed to write PNG: {}", e))?;
    } else {
        // Copy the raw screen capture — much faster, no IPC overhead
        let capture = PathBuf::from("/tmp/snap-capture.png");
        if capture.exists() {
            fs::copy(&capture, &png_path).map_err(|e| format!("Failed to copy capture: {}", e))?;
        } else {
            return Err("No capture file found".to_string());
        }
    }

    // Inject image filename into metadata
    let mut metadata: serde_json::Value =
        serde_json::from_str(&metadata_json).map_err(|e| format!("Invalid JSON: {}", e))?;
    metadata["image_filename"] = serde_json::json!(png_name);

    let metadata_str =
        serde_json::to_string_pretty(&metadata).map_err(|e| format!("JSON serialize: {}", e))?;

    // Write JSON
    fs::write(&json_path, &metadata_str)
        .map_err(|e| format!("Failed to write metadata: {}", e))?;

    log_event(&format!("Saved annotation: {}", png_name));

    Ok(png_path.to_string_lossy().to_string())
}

// ----- Read capture as base64 (avoids tainted canvas) -----

#[tauri::command]
fn read_capture_base64() -> Result<String, String> {
    let path = "/tmp/snap-capture.png";
    let bytes = fs::read(path).map_err(|e| format!("Failed to read capture: {}", e))?;
    Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
}

// ----- Overlay lifecycle -----

#[tauri::command]
fn mark_overlay_closed() {
    OVERLAY_ACTIVE.store(false, Ordering::SeqCst);
    log_event("overlay closed");
}

// ----- Helpers -----

fn inbox_dir() -> Result<PathBuf, String> {
    let dir = dirs::home_dir()
        .ok_or("Could not determine home directory")?
        .join(".snap")
        .join("inbox");
    fs::create_dir_all(&dir).map_err(|e| format!("Failed to create inbox: {}", e))?;
    Ok(dir)
}

pub fn log_event(msg: &str) {
    let log_dir = dirs::home_dir()
        .map(|h| h.join(".snap"))
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let _ = fs::create_dir_all(&log_dir);
    let log_path = log_dir.join("snap.log");

    // Rotate if over 1MB
    if let Ok(meta) = fs::metadata(&log_path) {
        if meta.len() > 1_000_000 {
            let backup = log_dir.join("snap.log.old");
            let _ = fs::rename(&log_path, &backup);
        }
    }

    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let line = format!("[{}] {}\n", timestamp, msg);
    let _ = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut f| {
            use std::io::Write;
            f.write_all(line.as_bytes())
        });
}

// ----- Public: generate the invoke handler -----

pub fn invoke_handler() -> impl Fn(tauri::ipc::Invoke) -> bool {
    tauri::generate_handler![
        capture_screen,
        get_active_window_context,
        save_annotation,
        read_capture_base64,
        mark_overlay_closed,
    ]
}
