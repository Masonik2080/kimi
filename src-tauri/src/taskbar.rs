//! Управление фильтром таскбара для виртуальных столов

// ─────────────────────────────────────────────────────────────────────────────
// Tauri Commands
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn enable_taskbar_filter() -> Result<(), String> {
    platform::set_filter(true)
}

#[tauri::command]
pub async fn disable_taskbar_filter() -> Result<(), String> {
    platform::set_filter(false)
}

#[tauri::command]
pub async fn get_taskbar_filter_status() -> Result<bool, String> {
    platform::get_filter_status()
}

// ─────────────────────────────────────────────────────────────────────────────
// Startup helper
// ─────────────────────────────────────────────────────────────────────────────

pub fn init_taskbar_filter() {
    #[cfg(windows)]
    {
        if let Err(e) = platform::set_filter(true) {
            eprintln!("Не удалось настроить фильтр таскбара: {}", e);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Platform-specific implementation
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(windows)]
mod platform {
    use windows::Win32::Foundation::{LPARAM, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{SendMessageW, HWND_BROADCAST, WM_SETTINGCHANGE};
    use winreg::enums::*;
    use winreg::RegKey;

    const REG_PATH: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced";
    const REG_KEY: &str = "VirtualDesktopTaskbarFilter";

    pub fn set_filter(show_current_only: bool) -> Result<(), String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        
        let key = hkcu
            .open_subkey_with_flags(REG_PATH, KEY_SET_VALUE | KEY_READ)
            .map_err(|e| format!("Не удалось открыть реестр: {}", e))?;

        let value: u32 = if show_current_only { 1 } else { 0 };
        
        key.set_value(REG_KEY, &value)
            .map_err(|e| format!("Не удалось установить значение: {}", e))?;

        broadcast_change();
        Ok(())
    }

    pub fn get_filter_status() -> Result<bool, String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        
        let key = hkcu
            .open_subkey(REG_PATH)
            .map_err(|e| format!("Не удалось открыть реестр: {}", e))?;

        let value: u32 = key.get_value(REG_KEY).unwrap_or(0);
        Ok(value == 1)
    }

    fn broadcast_change() {
        unsafe {
            let _ = SendMessageW(HWND_BROADCAST, WM_SETTINGCHANGE, WPARAM(0), LPARAM(0));
        }
    }
}

#[cfg(not(windows))]
mod platform {
    pub fn set_filter(_: bool) -> Result<(), String> {
        Err("Только для Windows".to_string())
    }

    pub fn get_filter_status() -> Result<bool, String> {
        Err("Только для Windows".to_string())
    }
}
