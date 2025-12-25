//! Управление виртуальными столами через эмуляцию клавиатуры

use std::thread;
use std::time::Duration;

// ─────────────────────────────────────────────────────────────────────────────
// Tauri Commands
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_virtual_desktop() -> Result<usize, String> {
    platform::create_desktop()
}

#[tauri::command]
pub async fn remove_current_virtual_desktop() -> Result<(), String> {
    platform::remove_current_desktop()
}

#[tauri::command]
pub async fn switch_desktop_left() -> Result<usize, String> {
    platform::switch_left()
}

#[tauri::command]
pub async fn switch_desktop_right() -> Result<usize, String> {
    platform::switch_right()
}

// ─────────────────────────────────────────────────────────────────────────────
// Public helper
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(windows)]
pub fn ensure_virtual_desktops_exist(required: usize) -> Result<(), String> {
    platform::ensure_desktops_exist(required)
}

#[cfg(not(windows))]
pub fn ensure_virtual_desktops_exist(_required: usize) -> Result<(), String> {
    Err("Только для Windows".to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
// Platform-specific implementation
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(windows)]
mod platform {
    use super::*;
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        keybd_event, KEYBD_EVENT_FLAGS, VIRTUAL_KEY, VK_CONTROL, VK_D, VK_F4, VK_LEFT, VK_LWIN, VK_RIGHT,
    };
    use winvd::{get_current_desktop, get_desktop_count};

    pub fn create_desktop() -> Result<usize, String> {
        let before = get_desktop_count().map_err(|e| format!("{:?}", e))? as usize;

        send_combo(&[(VK_LWIN, false), (VK_CONTROL, false), (VK_D, false)]);
        thread::sleep(Duration::from_millis(500));

        let after = get_desktop_count().map_err(|e| format!("{:?}", e))? as usize;

        if after > before {
            Ok(after - 1)
        } else {
            Err("Не удалось создать виртуальный стол".to_string())
        }
    }

    pub fn remove_current_desktop() -> Result<(), String> {
        let count = get_desktop_count().map_err(|e| format!("{:?}", e))?;

        if count <= 1 {
            return Err("Нельзя удалить последний виртуальный стол".to_string());
        }

        send_combo(&[(VK_LWIN, false), (VK_CONTROL, false), (VK_F4, false)]);
        thread::sleep(Duration::from_millis(500));

        Ok(())
    }

    pub fn switch_left() -> Result<usize, String> {
        send_combo(&[(VK_LWIN, false), (VK_CONTROL, false), (VK_LEFT, true)]);
        thread::sleep(Duration::from_millis(300));
        get_current_desktop().map_err(|e| format!("{:?}", e))
    }

    pub fn switch_right() -> Result<usize, String> {
        send_combo(&[(VK_LWIN, false), (VK_CONTROL, false), (VK_RIGHT, true)]);
        thread::sleep(Duration::from_millis(300));
        get_current_desktop().map_err(|e| format!("{:?}", e))
    }

    pub fn ensure_desktops_exist(required: usize) -> Result<(), String> {
        let mut current = get_desktop_count().map_err(|e| format!("{:?}", e))? as usize;

        while current < required {
            send_combo(&[(VK_LWIN, false), (VK_CONTROL, false), (VK_D, false)]);
            thread::sleep(Duration::from_millis(500));

            let new = get_desktop_count().map_err(|e| format!("{:?}", e))? as usize;
            if new == current {
                return Err(format!("Не удалось создать стол. Нужно: {}, есть: {}", required, current));
            }
            current = new;
        }

        Ok(())
    }

    fn send_combo(keys: &[(VIRTUAL_KEY, bool)]) {
        unsafe {
            // Press
            for (vk, extended) in keys {
                let flags = if *extended { 1 } else { 0 };
                keybd_event(vk.0 as u8, 0, KEYBD_EVENT_FLAGS(flags), 0);
            }

            thread::sleep(Duration::from_millis(50));

            // Release
            for (vk, extended) in keys.iter().rev() {
                let flags = if *extended { 3 } else { 2 }; // 2 = KEYEVENTF_KEYUP
                keybd_event(vk.0 as u8, 0, KEYBD_EVENT_FLAGS(flags), 0);
            }
        }
    }
}

#[cfg(not(windows))]
mod platform {
    pub fn create_desktop() -> Result<usize, String> {
        Err("Только для Windows".to_string())
    }

    pub fn remove_current_desktop() -> Result<(), String> {
        Err("Только для Windows".to_string())
    }

    pub fn switch_left() -> Result<usize, String> {
        Err("Только для Windows".to_string())
    }

    pub fn switch_right() -> Result<usize, String> {
        Err("Только для Windows".to_string())
    }

    pub fn ensure_desktops_exist(_: usize) -> Result<(), String> {
        Err("Только для Windows".to_string())
    }
}
