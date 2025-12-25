//! Переключение рабочих пространств (файлы + виртуальный стол)

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use crate::config::{get_desktop_folder_path, load_config, save_config, CONFIG_LOCK};
use crate::desktop::icons::{
    get_current_icon_positions, load_icon_positions, restore_icon_positions, save_icon_positions,
};
use crate::desktop::set_desktop_path;

#[cfg(windows)]
use crate::virtual_desktop::ensure_virtual_desktops_exist;
#[cfg(windows)]
use winvd::go_to_desktop;

// ─────────────────────────────────────────────────────────────────────────────
// Tauri Commands
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn switch_workspace(kimi_desktop_id: i32) -> Result<(), String> {
    let _lock = CONFIG_LOCK.lock().map_err(|e| e.to_string())?;
    let mut config = load_config();

    // Сохраняем иконки текущего стола
    save_icons_for_current(&config, kimi_desktop_id);

    // Находим индекс целевого стола
    let kimi_index = config
        .desktops
        .iter()
        .position(|d| d.id == kimi_desktop_id)
        .ok_or("Рабочий стол Kimi не найден")?;

    // Переключаем путь
    let desktop_path = get_desktop_folder_path(kimi_desktop_id);
    if !desktop_path.exists() {
        return Err("Папка рабочего стола не существует".to_string());
    }

    set_desktop_path(&desktop_path.to_string_lossy())?;

    // Переключаем виртуальный стол Windows
    #[cfg(windows)]
    {
        ensure_virtual_desktops_exist(kimi_index + 1)?;
        let _ = go_to_desktop(kimi_index);
    }

    config.active_desktop_id = kimi_desktop_id;
    save_config(&config)?;

    // Восстанавливаем иконки нового стола
    thread::sleep(Duration::from_millis(300));
    restore_icons_for_desktop(&desktop_path);

    Ok(())
}

#[tauri::command]
pub async fn link_to_virtual_desktop(
    kimi_desktop_id: i32,
    virtual_desktop_index: String,
) -> Result<(), String> {
    let _lock = CONFIG_LOCK.lock().map_err(|e| e.to_string())?;
    let mut config = load_config();

    if !config.desktops.iter().any(|d| d.id == kimi_desktop_id) {
        return Err("Рабочий стол Kimi не найден".to_string());
    }

    config.virtual_desktop_mapping.insert(kimi_desktop_id, virtual_desktop_index);
    save_config(&config)?;

    Ok(())
}

#[tauri::command]
pub async fn unlink_from_virtual_desktop(kimi_desktop_id: i32) -> Result<(), String> {
    let _lock = CONFIG_LOCK.lock().map_err(|e| e.to_string())?;
    let mut config = load_config();
    
    config.virtual_desktop_mapping.remove(&kimi_desktop_id);
    save_config(&config)?;

    Ok(())
}

#[tauri::command]
pub async fn get_workspace_mappings() -> Result<HashMap<i32, String>, String> {
    let config = load_config();
    Ok(config.virtual_desktop_mapping)
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

fn save_icons_for_current(config: &crate::config::KimiConfig, new_id: i32) {
    let old_id = config.active_desktop_id;
    
    if old_id <= 0 || old_id == new_id {
        return;
    }

    let old_path = get_desktop_folder_path(old_id);
    
    if let Ok(layout) = get_current_icon_positions() {
        if !layout.icons.is_empty() {
            let _ = save_icon_positions(&old_path, &layout);
        }
    }
}

fn restore_icons_for_desktop(desktop_path: &std::path::PathBuf) {
    let layout = load_icon_positions(desktop_path);
    
    if !layout.icons.is_empty() {
        let _ = restore_icon_positions(&layout, desktop_path);
    }
}
