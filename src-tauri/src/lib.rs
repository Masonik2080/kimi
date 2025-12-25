//! Kimi - менеджер виртуальных рабочих столов

mod autostart;
mod config;
mod desktop;
mod hotkeys;
mod taskbar;
mod tray;
mod virtual_desktop;
mod workspace;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    desktop::validate_and_restore_if_needed();
    taskbar::init_taskbar_filter();

    let hotkey_running = Arc::new(AtomicBool::new(true));
    let hotkey_running_clone = hotkey_running.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // При попытке запустить второй экземпляр — показываем главное окно
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .setup(move |app| {
            tray::setup_system_tray(app.handle())?;
            hotkeys::start_hotkey_listener(app.handle().clone(), hotkey_running_clone.clone());
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            // Desktop management
            desktop::manager::get_desktops,
            desktop::manager::create_desktop,
            desktop::manager::switch_desktop,
            desktop::manager::delete_desktop,
            desktop::manager::restore_original_desktop,
            desktop::manager::get_original_desktop_path,
            // Icon positions
            desktop::icons::save_desktop_icons,
            desktop::icons::restore_desktop_icons,
            desktop::icons::get_saved_icon_positions,
            desktop::icons::force_save_current_layout,
            desktop::icons::disable_desktop_auto_arrange,
            // Virtual desktops
            virtual_desktop::api::get_virtual_desktop_count,
            virtual_desktop::api::get_current_virtual_desktop,
            virtual_desktop::api::switch_virtual_desktop,
            virtual_desktop::keyboard::create_virtual_desktop,
            virtual_desktop::keyboard::remove_current_virtual_desktop,
            virtual_desktop::keyboard::switch_desktop_left,
            virtual_desktop::keyboard::switch_desktop_right,
            virtual_desktop::api::move_window_to_desktop,
            virtual_desktop::api::is_window_on_current_desktop,
            virtual_desktop::api::get_window_desktop_index,
            virtual_desktop::api::get_all_windows,
            virtual_desktop::api::get_windows_on_current_desktop,
            // Taskbar
            taskbar::enable_taskbar_filter,
            taskbar::disable_taskbar_filter,
            taskbar::get_taskbar_filter_status,
            // Workspace
            workspace::switch_workspace,
            workspace::link_to_virtual_desktop,
            workspace::unlink_from_virtual_desktop,
            workspace::get_workspace_mappings,
            // Tray
            tray::show_main_window,
            tray::exit_app,
            // Hotkeys
            hotkeys::get_hotkey_settings,
            hotkeys::set_hotkey_settings,
            hotkeys::toggle_hotkeys,
            // Autostart
            autostart::get_autostart_enabled,
            autostart::set_autostart_enabled,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
