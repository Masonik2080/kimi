//! Управление позициями иконок рабочего стола

mod com;
mod layout;
mod storage;

pub use layout::DesktopIconsLayout;
pub use storage::{load_icon_positions, save_icon_positions};

#[cfg(windows)]
pub use com::{get_current_icon_positions, restore_icon_positions};

#[cfg(not(windows))]
pub fn get_current_icon_positions() -> Result<DesktopIconsLayout, String> {
    Err("Только для Windows".to_string())
}

#[cfg(not(windows))]
pub fn restore_icon_positions(
    _layout: &DesktopIconsLayout,
    _desktop_path: &std::path::PathBuf,
) -> Result<(), String> {
    Err("Только для Windows".to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
// Tauri Commands
// ─────────────────────────────────────────────────────────────────────────────

use crate::config::get_desktop_folder_path;

#[tauri::command]
pub async fn save_desktop_icons(desktop_id: i32) -> Result<(), String> {
    let layout = get_current_icon_positions()?;
    let path = get_desktop_folder_path(desktop_id);
    save_icon_positions(&path, &layout)?;
    Ok(())
}

#[tauri::command]
pub async fn restore_desktop_icons(desktop_id: i32) -> Result<(), String> {
    let path = get_desktop_folder_path(desktop_id);
    let layout = load_icon_positions(&path);
    
    if !layout.icons.is_empty() {
        restore_icon_positions(&layout, &path)?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_saved_icon_positions(desktop_id: i32) -> Result<DesktopIconsLayout, String> {
    let path = get_desktop_folder_path(desktop_id);
    Ok(load_icon_positions(&path))
}

#[tauri::command]
pub async fn force_save_current_layout(desktop_id: i32) -> Result<(), String> {
    std::thread::sleep(std::time::Duration::from_millis(200));
    save_desktop_icons(desktop_id).await
}

#[tauri::command]
pub async fn disable_desktop_auto_arrange() -> Result<(), String> {
    #[cfg(windows)]
    {
        com::disable_auto_arrange_public()
    }
    #[cfg(not(windows))]
    Err("Только для Windows".to_string())
}
