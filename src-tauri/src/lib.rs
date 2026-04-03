use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    window::Color,
    Emitter, Manager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            // Force transparency: set background + resize workaround for DMG builds
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.set_background_color(Some(Color(0, 0, 0, 0)));
                // Resize trick: change size by 1px and back to force WebKit re-render
                // This fixes the known macOS bug where transparency is lost in bundled builds
                let size = win.outer_size().unwrap_or(tauri::PhysicalSize::new(340, 340));
                let _ = win.set_size(tauri::PhysicalSize::new(size.width + 1, size.height + 1));
                let _ = win.set_size(size);
            }

            // --- Build variant submenu ---
            let v_classic = MenuItem::with_id(app, "variant_classic", "Classic (Silver)", true, None::<&str>)?;
            let v_white = MenuItem::with_id(app, "variant_white", "White", true, None::<&str>)?;
            let v_black = MenuItem::with_id(app, "variant_black", "Black", true, None::<&str>)?;
            let v_dark = MenuItem::with_id(app, "variant_dark", "Dark", true, None::<&str>)?;
            let v_red = MenuItem::with_id(app, "variant_red", "Red", true, None::<&str>)?;
            let v_gold = MenuItem::with_id(app, "variant_gold", "Gold", true, None::<&str>)?;

            let variant_menu = Submenu::with_items(
                app,
                "Color Variant",
                true,
                &[&v_classic, &v_white, &v_black, &v_dark, &v_red, &v_gold],
            )?;

            // --- Build shadow submenu ---
            let s_light = MenuItem::with_id(app, "shadow_light", "Light", true, None::<&str>)?;
            let s_medium = MenuItem::with_id(app, "shadow_medium", "Medium", true, None::<&str>)?;
            let s_heavy = MenuItem::with_id(app, "shadow_heavy", "Heavy", true, None::<&str>)?;
            let s_none = MenuItem::with_id(app, "shadow_none", "None", true, None::<&str>)?;

            let shadow_menu = Submenu::with_items(
                app,
                "Shadow",
                true,
                &[&s_light, &s_medium, &s_heavy, &s_none],
            )?;

            // --- Window layer controls ---
            let bring_front = MenuItem::with_id(app, "bring_front", "Bring to Front", true, None::<&str>)?;
            let send_back = MenuItem::with_id(app, "send_back", "Send to Desktop", true, None::<&str>)?;

            let sep1 = PredefinedMenuItem::separator(app)?;
            let sep2 = PredefinedMenuItem::separator(app)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[
                    &variant_menu,
                    &shadow_menu,
                    &sep1,
                    &bring_front,
                    &send_back,
                    &sep2,
                    &quit,
                ],
            )?;

            let _tray = TrayIconBuilder::with_id("main")
                .tooltip("SBB Clock Widget")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| {
                    let id = event.id().as_ref();
                    if id == "quit" {
                        app.exit(0);
                    } else if id == "bring_front" {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.set_always_on_bottom(false);
                            let _ = win.set_always_on_top(true);
                            let _ = win.set_focus();
                            // Reset after a moment so it doesn't stay on top forever
                            let win_clone = win.clone();
                            std::thread::spawn(move || {
                                std::thread::sleep(std::time::Duration::from_secs(2));
                                let _ = win_clone.set_always_on_top(false);
                            });
                        }
                    } else if id == "send_back" {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.set_always_on_top(false);
                            let _ = win.set_always_on_bottom(true);
                        }
                    } else if let Some(variant) = id.strip_prefix("variant_") {
                        let _ = app.emit("set-variant", variant.to_string());
                    } else if let Some(shadow) = id.strip_prefix("shadow_") {
                        let _ = app.emit("set-shadow", shadow.to_string());
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
