//! Глобальные горячие клавиши для переключения рабочих столов

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tauri::AppHandle;

const SETTINGS_FILE: &str = "C:\\Kimi\\hotkeys.json";

// ─────────────────────────────────────────────────────────────────────────────
// Data structures
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HotkeySettings {
    pub enabled: bool,
    pub modifier: String, // "alt", "ctrl+alt", "ctrl+shift"
}

impl Default for HotkeySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            modifier: "alt".to_string(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Settings persistence
// ─────────────────────────────────────────────────────────────────────────────

pub fn load_hotkey_settings() -> HotkeySettings {
    let path = PathBuf::from(SETTINGS_FILE);
    
    if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
            .unwrap_or_default()
    } else {
        HotkeySettings::default()
    }
}

fn save_hotkey_settings(settings: &HotkeySettings) -> Result<(), String> {
    let path = PathBuf::from(SETTINGS_FILE);
    
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    
    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
// Tauri Commands
// ─────────────────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_hotkey_settings() -> Result<HotkeySettings, String> {
    Ok(load_hotkey_settings())
}

#[tauri::command]
pub async fn set_hotkey_settings(settings: HotkeySettings) -> Result<(), String> {
    save_hotkey_settings(&settings)?;
    
    #[cfg(windows)]
    hook::update_settings(&settings);
    
    Ok(())
}

#[tauri::command]
pub async fn toggle_hotkeys(enabled: bool) -> Result<(), String> {
    let mut settings = load_hotkey_settings();
    settings.enabled = enabled;
    save_hotkey_settings(&settings)?;
    
    #[cfg(windows)]
    hook::update_settings(&settings);
    
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Hook management
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(windows)]
pub fn start_hotkey_listener(app_handle: AppHandle, _running: Arc<AtomicBool>) {
    hook::start(app_handle);
}

#[cfg(not(windows))]
pub fn start_hotkey_listener(_app_handle: AppHandle, _running: Arc<AtomicBool>) {}

// ─────────────────────────────────────────────────────────────────────────────
// Windows keyboard hook
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(windows)]
mod hook {
    use super::{load_hotkey_settings, HotkeySettings};
    use std::sync::{Mutex, OnceLock};
    use tauri::{AppHandle, Emitter};
    use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
    use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
    use windows::Win32::UI::WindowsAndMessaging::{
        CallNextHookEx, GetMessageW, SetWindowsHookExW, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL,
        WM_KEYDOWN, WM_SYSKEYDOWN,
    };

    static APP_HANDLE: OnceLock<Mutex<Option<AppHandle>>> = OnceLock::new();
    static SETTINGS: OnceLock<Mutex<HotkeySettings>> = OnceLock::new();

    pub fn start(app_handle: AppHandle) {
        APP_HANDLE
            .get_or_init(|| Mutex::new(None))
            .lock()
            .map(|mut g| *g = Some(app_handle))
            .ok();

        SETTINGS
            .get_or_init(|| Mutex::new(HotkeySettings::default()))
            .lock()
            .map(|mut g| *g = load_hotkey_settings())
            .ok();

        std::thread::spawn(|| unsafe {
            if let Ok(hook) = SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), None, 0) {
                let mut msg = MSG::default();
                while GetMessageW(&mut msg, None, 0, 0).0 > 0 {}
                let _ = windows::Win32::UI::WindowsAndMessaging::UnhookWindowsHookEx(hook);
            }
        });
    }

    pub fn update_settings(settings: &HotkeySettings) {
        SETTINGS
            .get_or_init(|| Mutex::new(HotkeySettings::default()))
            .lock()
            .map(|mut g| *g = settings.clone())
            .ok();
    }

    unsafe extern "system" fn hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if code >= 0 {
            let kb = *(lparam.0 as *const KBDLLHOOKSTRUCT);
            let is_keydown = wparam.0 == WM_KEYDOWN as usize || wparam.0 == WM_SYSKEYDOWN as usize;

            if is_keydown && (0x31..=0x39).contains(&kb.vkCode) {
                if let Some(desktop_num) = try_handle_hotkey(&kb) {
                    emit_switch_event(desktop_num);
                    return LRESULT(1);
                }
            }
        }

        CallNextHookEx(None, code, wparam, lparam)
    }

    unsafe fn try_handle_hotkey(kb: &KBDLLHOOKSTRUCT) -> Option<i32> {
        let settings = SETTINGS
            .get()?
            .lock()
            .ok()?
            .clone();

        if !settings.enabled {
            return None;
        }

        let alt = (kb.flags.0 & 0x20) != 0 || is_key_pressed(0x12);
        let ctrl = is_key_pressed(0x11);
        let shift = is_key_pressed(0x10);

        let modifier_match = match settings.modifier.as_str() {
            "ctrl+alt" => ctrl && alt && !shift,
            "ctrl+shift" => ctrl && shift && !alt,
            _ => alt && !ctrl && !shift,
        };

        if modifier_match {
            Some((kb.vkCode - 0x30) as i32)
        } else {
            None
        }
    }

    unsafe fn is_key_pressed(vk: i32) -> bool {
        (GetAsyncKeyState(vk) as u16 & 0x8000) != 0
    }

    fn emit_switch_event(desktop_num: i32) {
        if let Some(app_mutex) = APP_HANDLE.get() {
            if let Ok(guard) = app_mutex.lock() {
                if let Some(ref app) = *guard {
                    let _ = app.emit("hotkey-switch-desktop", desktop_num);
                }
            }
        }
    }
}
