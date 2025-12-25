//! Работа с реестром Windows для путей рабочего стола

#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;

#[cfg(windows)]
pub fn get_desktop_path_from_registry() -> Result<String, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    let shell_folders = hkcu
        .open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders")
        .map_err(|e| format!("Не удалось открыть реестр: {}", e))?;

    let desktop_path: String = shell_folders
        .get_value("Desktop")
        .map_err(|e| format!("Не удалось прочитать путь Desktop: {}", e))?;

    Ok(expand_env_vars(&desktop_path))
}

#[cfg(windows)]
fn expand_env_vars(path: &str) -> String {
    let mut result = path.to_string();
    
    if let Ok(profile) = std::env::var("USERPROFILE") {
        result = result.replace("%USERPROFILE%", &profile);
    }
    
    result
}

#[cfg(not(windows))]
pub fn get_desktop_path_from_registry() -> Result<String, String> {
    Err("Только для Windows".to_string())
}
