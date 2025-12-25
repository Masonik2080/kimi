//! API для работы с виртуальными рабочими столами Windows

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowInfo {
    pub hwnd: isize,
    pub title: String,
    pub process_id: u32,
    pub desktop_index: Option<usize>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Tauri Commands
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_virtual_desktop_count() -> Result<usize, String> {
    platform::get_desktop_count()
}

#[tauri::command]
pub async fn get_current_virtual_desktop() -> Result<usize, String> {
    platform::get_current_desktop()
}

#[tauri::command]
pub async fn switch_virtual_desktop(index: usize) -> Result<(), String> {
    platform::switch_to_desktop(index)
}

#[tauri::command]
pub async fn move_window_to_desktop(hwnd_raw: isize, desktop_index: usize) -> Result<(), String> {
    platform::move_window(hwnd_raw, desktop_index)
}

#[tauri::command]
pub async fn is_window_on_current_desktop(hwnd_raw: isize) -> Result<bool, String> {
    platform::is_on_current_desktop(hwnd_raw)
}

#[tauri::command]
pub async fn get_window_desktop_index(hwnd_raw: isize) -> Result<usize, String> {
    platform::get_window_desktop(hwnd_raw)
}

#[tauri::command]
pub async fn get_all_windows() -> Result<Vec<WindowInfo>, String> {
    platform::enumerate_windows()
}

#[tauri::command]
pub async fn get_windows_on_current_desktop() -> Result<Vec<WindowInfo>, String> {
    let all = platform::enumerate_windows()?;
    let current = platform::get_current_desktop()?;
    
    Ok(all.into_iter().filter(|w| w.desktop_index == Some(current)).collect())
}

// ─────────────────────────────────────────────────────────────────────────────
// Platform-specific implementation
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(windows)]
mod platform {
    use super::WindowInfo;
    use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible,
    };

    pub fn get_desktop_count() -> Result<usize, String> {
        winvd::get_desktop_count()
            .map(|c| c as usize)
            .map_err(|e| format!("{:?}", e))
    }

    pub fn get_current_desktop() -> Result<usize, String> {
        winvd::get_current_desktop().map_err(|e| format!("{:?}", e))
    }

    pub fn switch_to_desktop(index: usize) -> Result<(), String> {
        winvd::go_to_desktop(index).map_err(|e| format!("{:?}", e))
    }

    pub fn move_window(hwnd: isize, desktop: usize) -> Result<(), String> {
        winvd::move_window_to_desktop(hwnd as u32, desktop).map_err(|e| format!("{:?}", e))
    }

    pub fn is_on_current_desktop(hwnd: isize) -> Result<bool, String> {
        winvd::is_window_on_current_virtual_desktop(hwnd as u32).map_err(|e| format!("{:?}", e))
    }

    pub fn get_window_desktop(hwnd: isize) -> Result<usize, String> {
        winvd::get_desktop_by_window(hwnd as u32).map_err(|e| format!("{:?}", e))
    }

    pub fn enumerate_windows() -> Result<Vec<WindowInfo>, String> {
        struct EnumData {
            windows: Vec<WindowInfo>,
        }

        unsafe extern "system" fn callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
            let data = &mut *(lparam.0 as *mut EnumData);

            if IsWindowVisible(hwnd).as_bool() {
                let len = GetWindowTextLengthW(hwnd);
                if len > 0 {
                    let mut title = vec![0u16; (len + 1) as usize];
                    GetWindowTextW(hwnd, &mut title);
                    let title_str = String::from_utf16_lossy(&title[..len as usize]);

                    if !title_str.is_empty() {
                        let mut pid = 0u32;
                        GetWindowThreadProcessId(hwnd, Some(&mut pid));

                        let desktop_index = winvd::get_desktop_by_window(hwnd.0 as u32).ok();

                        data.windows.push(WindowInfo {
                            hwnd: hwnd.0 as isize,
                            title: title_str,
                            process_id: pid,
                            desktop_index,
                        });
                    }
                }
            }
            BOOL(1)
        }

        let mut data = EnumData { windows: Vec::new() };

        unsafe {
            let _ = EnumWindows(Some(callback), LPARAM(&mut data as *mut _ as isize));
        }

        Ok(data.windows)
    }
}

#[cfg(not(windows))]
mod platform {
    use super::WindowInfo;

    pub fn get_desktop_count() -> Result<usize, String> {
        Err("Только для Windows".to_string())
    }

    pub fn get_current_desktop() -> Result<usize, String> {
        Err("Только для Windows".to_string())
    }

    pub fn switch_to_desktop(_: usize) -> Result<(), String> {
        Err("Только для Windows".to_string())
    }

    pub fn move_window(_: isize, _: usize) -> Result<(), String> {
        Err("Только для Windows".to_string())
    }

    pub fn is_on_current_desktop(_: isize) -> Result<bool, String> {
        Err("Только для Windows".to_string())
    }

    pub fn get_window_desktop(_: isize) -> Result<usize, String> {
        Err("Только для Windows".to_string())
    }

    pub fn enumerate_windows() -> Result<Vec<WindowInfo>, String> {
        Err("Только для Windows".to_string())
    }
}
