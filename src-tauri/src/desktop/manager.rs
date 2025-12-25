//! Управление рабочими столами Kimi (CRUD операции)

use std::fs;

use crate::config::{
    count_files_in_dir, ensure_kimi_dir, get_desktop_folder_path, load_config, save_config,
    Desktop, DesktopConfig, CONFIG_LOCK, KIMI_DIR, MAX_DESKTOPS,
};
use super::icons::{
    get_current_icon_positions, load_icon_positions, restore_icon_positions, save_icon_positions,
};
use super::shell::set_desktop_path;

// ─────────────────────────────────────────────────────────────────────────────
// Tauri Commands
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_desktops() -> Result<Vec<Desktop>, String> {
    let _lock = CONFIG_LOCK.lock().map_err(|e| e.to_string())?;
    ensure_kimi_dir()?;
    
    let config = load_config();

    let desktops = config
        .desktops
        .iter()
        .map(|dc| {
            let path = get_desktop_folder_path(dc.id);
            Desktop {
                id: dc.id,
                name: dc.name.clone(),
                path: path.to_string_lossy().to_string(),
                is_active: dc.id == config.active_desktop_id,
                file_count: count_files_in_dir(&path),
            }
        })
        .collect();

    Ok(desktops)
}

#[tauri::command]
pub async fn create_desktop() -> Result<Desktop, String> {
    let _lock = CONFIG_LOCK.lock().map_err(|e| e.to_string())?;
    ensure_kimi_dir()?;
    
    let mut config = load_config();

    if config.desktops.len() >= MAX_DESKTOPS {
        return Err(format!("Максимум {} рабочих столов", MAX_DESKTOPS));
    }

    let new_id = config.desktops.iter().map(|d| d.id).max().unwrap_or(0) + 1;
    let desktop_path = get_desktop_folder_path(new_id);

    fs::create_dir_all(&desktop_path)
        .map_err(|e| format!("Не удалось создать папку: {}", e))?;

    let new_config = DesktopConfig {
        id: new_id,
        name: format!("Рабочий стол {}", new_id),
    };

    config.desktops.push(new_config.clone());
    save_config(&config)?;

    Ok(Desktop {
        id: new_id,
        name: new_config.name,
        path: desktop_path.to_string_lossy().to_string(),
        is_active: false,
        file_count: 0,
    })
}

#[tauri::command]
pub async fn switch_desktop(id: i32) -> Result<(), String> {
    let _lock = CONFIG_LOCK.lock().map_err(|e| e.to_string())?;
    let mut config = load_config();

    if !config.desktops.iter().any(|d| d.id == id) {
        return Err("Рабочий стол не найден".to_string());
    }

    let desktop_path = get_desktop_folder_path(id);
    if !desktop_path.exists() {
        return Err("Папка рабочего стола не существует".to_string());
    }

    // Сохраняем позиции текущего стола
    save_current_icons(&config);

    // Переключаем путь
    set_desktop_path(&desktop_path.to_string_lossy())?;

    // Восстанавливаем позиции нового стола
    restore_icons_for_desktop(&desktop_path);

    config.active_desktop_id = id;
    save_config(&config)?;

    Ok(())
}

#[tauri::command]
pub async fn delete_desktop(id: i32) -> Result<(), String> {
    let _lock = CONFIG_LOCK.lock().map_err(|e| e.to_string())?;
    let mut config = load_config();

    if config.desktops.len() <= 1 {
        return Err("Нельзя удалить последний рабочий стол".to_string());
    }

    if config.active_desktop_id == id {
        return Err("Нельзя удалить активный рабочий стол".to_string());
    }

    // Удаляем только из конфига, папка остаётся
    config.desktops.retain(|d| d.id != id);
    save_config(&config)?;

    Ok(())
}

#[tauri::command]
pub async fn restore_original_desktop() -> Result<(), String> {
    let _lock = CONFIG_LOCK.lock().map_err(|e| e.to_string())?;
    let mut config = load_config();

    if config.original_desktop_path.is_empty() {
        return Err("Оригинальный путь не сохранён".to_string());
    }

    set_desktop_path(&config.original_desktop_path)?;

    config.active_desktop_id = 0;
    save_config(&config)?;

    Ok(())
}

#[tauri::command]
pub async fn get_original_desktop_path() -> Result<String, String> {
    let config = load_config();
    Ok(config.original_desktop_path)
}

// ─────────────────────────────────────────────────────────────────────────────
// Startup validation
// ─────────────────────────────────────────────────────────────────────────────

pub fn validate_and_restore_if_needed() {
    #[cfg(windows)]
    {
        use super::registry::get_desktop_path_from_registry;
        use std::path::PathBuf;

        let Ok(current_path) = get_desktop_path_from_registry() else {
            return;
        };

        let path = PathBuf::from(&current_path);

        // Если путь указывает на Kimi, но папка не существует — восстанавливаем оригинал
        if current_path.starts_with(KIMI_DIR) && !path.exists() {
            let config = load_config();

            if !config.original_desktop_path.is_empty() {
                if set_desktop_path(&config.original_desktop_path).is_ok() {
                    let mut config = config;
                    config.active_desktop_id = 0;
                    let _ = save_config(&config);
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

fn save_current_icons(config: &crate::config::KimiConfig) {
    if config.active_desktop_id <= 0 {
        return;
    }

    let current_path = get_desktop_folder_path(config.active_desktop_id);
    
    if let Ok(layout) = get_current_icon_positions() {
        if !layout.icons.is_empty() {
            let _ = save_icon_positions(&current_path, &layout);
        }
    }
}

fn restore_icons_for_desktop(desktop_path: &std::path::PathBuf) {
    let layout = load_icon_positions(desktop_path);
    
    if !layout.icons.is_empty() {
        let _ = restore_icon_positions(&layout, desktop_path);
    }
}
