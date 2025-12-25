//! Сохранение и загрузка позиций иконок

use super::layout::DesktopIconsLayout;
use std::fs;
use std::path::Path;

const COORDS_FILE: &str = "coords.kimi";

pub fn load_icon_positions(desktop_path: &Path) -> DesktopIconsLayout {
    let coords_file = desktop_path.join(COORDS_FILE);
    
    if !coords_file.exists() {
        return DesktopIconsLayout::default();
    }
    
    fs::read_to_string(&coords_file)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

pub fn save_icon_positions(desktop_path: &Path, layout: &DesktopIconsLayout) -> Result<(), String> {
    let coords_file = desktop_path.join(COORDS_FILE);
    let content = serde_json::to_string_pretty(layout).map_err(|e| e.to_string())?;
    
    fs::write(&coords_file, content)
        .map_err(|e| format!("Не удалось сохранить coords.kimi: {}", e))?;
    
    hide_file(&coords_file);
    Ok(())
}

#[cfg(windows)]
fn hide_file(path: &Path) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    
    let wide_path: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    unsafe {
        let _ = windows::Win32::Storage::FileSystem::SetFileAttributesW(
            windows::core::PCWSTR(wide_path.as_ptr()),
            windows::Win32::Storage::FileSystem::FILE_ATTRIBUTE_HIDDEN,
        );
    }
}

#[cfg(not(windows))]
fn hide_file(_path: &Path) {}
