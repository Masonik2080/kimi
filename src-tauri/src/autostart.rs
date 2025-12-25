//! Управление автозапуском приложения

const APP_NAME: &str = "Kimi";

// ─────────────────────────────────────────────────────────────────────────────
// Tauri Commands
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_autostart_enabled() -> Result<bool, String> {
    Ok(platform::is_enabled())
}

#[tauri::command]
pub async fn set_autostart_enabled(enabled: bool) -> Result<(), String> {
    platform::set_enabled(enabled)
}

// ─────────────────────────────────────────────────────────────────────────────
// Platform-specific implementation
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(windows)]
mod platform {
    use super::APP_NAME;
    use std::env;
    use winreg::enums::*;
    use winreg::RegKey;

    const REG_PATH: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";

    pub fn is_enabled() -> bool {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        
        hkcu.open_subkey(REG_PATH)
            .map(|key| key.get_value::<String, _>(APP_NAME).is_ok())
            .unwrap_or(false)
    }

    pub fn set_enabled(enabled: bool) -> Result<(), String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        
        let key = hkcu
            .open_subkey_with_flags(REG_PATH, KEY_WRITE)
            .map_err(|e| format!("Не удалось открыть реестр: {}", e))?;

        if enabled {
            let exe_path = get_exe_path()?;
            key.set_value(APP_NAME, &exe_path)
                .map_err(|e| format!("Не удалось добавить в автозапуск: {}", e))
        } else {
            let _ = key.delete_value(APP_NAME);
            Ok(())
        }
    }

    fn get_exe_path() -> Result<String, String> {
        env::current_exe()
            .map(|p| format!("\"{}\"", p.display()))
            .map_err(|_| "Не удалось получить путь к exe".to_string())
    }
}

#[cfg(not(windows))]
mod platform {
    pub fn is_enabled() -> bool {
        false
    }

    pub fn set_enabled(_: bool) -> Result<(), String> {
        Err("Только для Windows".to_string())
    }
}
