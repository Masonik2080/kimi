//! System Tray

use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

// ─────────────────────────────────────────────────────────────────────────────
// Setup
// ─────────────────────────────────────────────────────────────────────────────

pub fn setup_system_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let _tray = TrayIconBuilder::with_id("main")
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("Kimi")
        .on_tray_icon_event(handle_tray_event)
        .build(app)?;

    setup_tray_window_blur(app);
    Ok(())
}

fn handle_tray_event(tray: &tauri::tray::TrayIcon, event: TrayIconEvent) {
    let TrayIconEvent::Click { button, button_state: MouseButtonState::Up, position, .. } = event else {
        return;
    };

    let app = tray.app_handle();

    // Скрываем меню если оно открыто
    if let Some(tray_window) = app.get_webview_window("tray-menu") {
        if tray_window.is_visible().unwrap_or(false) {
            let _ = tray_window.hide();
            return;
        }
    }

    match button {
        MouseButton::Left => show_main(app),
        MouseButton::Right => show_tray_menu(app, position),
        _ => {}
    }
}

fn show_main(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn show_tray_menu(app: &AppHandle, position: tauri::PhysicalPosition<f64>) {
    if let Some(tray_window) = app.get_webview_window("tray-menu") {
        let x = position.x as i32 - 100;
        let y = position.y as i32 - 250;

        let _ = tray_window.set_position(tauri::Position::Physical(
            tauri::PhysicalPosition { x, y },
        ));
        let _ = tray_window.show();
        let _ = tray_window.set_focus();
    }
}

fn setup_tray_window_blur(app: &AppHandle) {
    let app_handle = app.clone();
    
    if let Some(tray_window) = app.get_webview_window("tray-menu") {
        tray_window.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(false) = event {
                if let Some(w) = app_handle.get_webview_window("tray-menu") {
                    let _ = w.hide();
                }
            }
        });
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tauri Commands
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn show_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn exit_app(app: AppHandle) {
    app.exit(0);
}
