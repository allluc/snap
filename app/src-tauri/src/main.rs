// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_wayland = env::var("WAYLAND_DISPLAY").is_ok();

    // On Wayland or with --overlay flag: single-shot overlay mode
    // On X11 without flags: tray app with global hotkey
    if is_wayland || args.contains(&"--overlay-mode".to_string()) {
        run_overlay_mode();
    } else {
        run_tray_mode();
    }
}

/// Single-shot mode: open overlay window, user annotates, save, exit.
/// Used on Wayland where global hotkeys require the DE to trigger us.
fn run_overlay_mode() {
    snap_lib::log_event("snap starting (overlay mode)");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(snap_lib::invoke_handler())
        .setup(|app| {
            snap_lib::log_event("overlay mode ready");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running snap");
}

/// Tray mode: background app with global hotkey (X11 only).
fn run_tray_mode() {
    use std::sync::atomic::Ordering;
    use tauri::{
        image::Image,
        menu::{Menu, MenuItem},
        tray::TrayIconBuilder,
        Manager,
    };
    use tauri_plugin_global_shortcut::{
        Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
    };

    snap_lib::log_event("snap starting (tray mode, X11)");

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(snap_lib::invoke_handler())
        .setup(|app| {
            // ---- System tray ----
            let quit = MenuItem::with_id(app, "quit", "Quit Snap", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;

            let icon = {
                let png_bytes = include_bytes!("../icons/icon.png");
                let decoder = png::Decoder::new(std::io::Cursor::new(png_bytes));
                match decoder.read_info() {
                    Ok(mut reader) => {
                        let mut buf = vec![0u8; reader.output_buffer_size()];
                        if let Ok(info) = reader.next_frame(&mut buf) {
                            buf.truncate(info.buffer_size());
                            Image::new_owned(buf, info.width, info.height)
                        } else {
                            Image::new_owned(vec![255u8, 255, 255, 200], 1, 1)
                        }
                    }
                    Err(_) => Image::new_owned(vec![255u8, 255, 255, 200], 1, 1),
                }
            };

            TrayIconBuilder::new()
                .icon(icon)
                .tooltip("Snap — Ctrl+Shift+S to annotate")
                .menu(&menu)
                .on_menu_event(|app, event| {
                    if event.id() == "quit" {
                        snap_lib::log_event("quit from tray");
                        app.exit(0);
                    }
                })
                .build(app)?;

            // ---- Global shortcut: Ctrl+Shift+S ----
            let shortcut = Shortcut::new(
                Some(Modifiers::CONTROL | Modifiers::SHIFT),
                Code::KeyS,
            );

            let app_handle = app.handle().clone();

            app.global_shortcut().on_shortcut(
                shortcut,
                move |_app, _shortcut, event| {
                    if event.state != ShortcutState::Pressed {
                        return;
                    }

                    if snap_lib::OVERLAY_ACTIVE.load(Ordering::SeqCst) {
                        return;
                    }
                    snap_lib::OVERLAY_ACTIVE.store(true, Ordering::SeqCst);

                    snap_lib::log_event("hotkey triggered");

                    if let Some(window) = app_handle.get_webview_window("overlay") {
                        let _ = window.destroy();
                    }

                    let handle = app_handle.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(50));
                        let handle_inner = handle.clone();
                        let _ = handle.run_on_main_thread(move || {
                            match tauri::WebviewWindowBuilder::new(
                                &handle_inner,
                                "overlay",
                                tauri::WebviewUrl::App("index.html".into()),
                            )
                            .title("Snap")
                            .visible(false)
                            .transparent(true)
                            .decorations(false)
                            .fullscreen(true)
                            .always_on_top(true)
                            .skip_taskbar(true)
                            .resizable(false)
                            .build()
                            {
                                Ok(_) => snap_lib::log_event("overlay window created"),
                                Err(e) => {
                                    snap_lib::log_event(&format!(
                                        "failed to create overlay: {}",
                                        e
                                    ));
                                    snap_lib::OVERLAY_ACTIVE.store(false, Ordering::SeqCst);
                                }
                            }
                        });
                    });
                },
            )?;

            snap_lib::log_event("global shortcut registered: Ctrl+Shift+S");

            // Destroy default window — start headless
            if let Some(w) = app.get_webview_window("overlay") {
                let _ = w.destroy();
            }

            snap_lib::log_event("snap ready — waiting for hotkey");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running snap");
}
