//! Конфигурация приложения Kimi

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub const KIMI_DIR: &str = "C:\\Kimi";
pub const MAX_DESKTOPS: usize = 20;

const CONFIG_FILE: &str = "C:\\Kimi\\kimi.json";

pub static CONFIG_LOCK: Mutex<()> = Mutex::new(());

// ─────────────────────────────────────────────────────────────────────────────
// Data structures
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Desktop {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub is_active: bool,
    pub file_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DesktopConfig {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct KimiConfig {
    pub desktops: Vec<DesktopConfig>,
    pub active_desktop_id: i32,
    pub original_desktop_path: String,
    pub virtual_desktop_mapping: HashMap<i32, String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Public API
// ─────────────────────────────────────────────────────────────────────────────

pub fn ensure_kimi_dir() -> Result<(), String> {
    let path = PathBuf::from(KIMI_DIR);
    
    if !path.exists() {
        fs::create_dir_all(&path)
            .map_err(|e| format!("Не удалось создать директорию Kimi: {}", e))?;
    }
    
    Ok(())
}

pub fn load_config() -> KimiConfig {
    let path = PathBuf::from(CONFIG_FILE);
    
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
    }

    KimiConfig {
        desktops: vec![],
        active_desktop_id: 0,
        original_desktop_path: get_default_desktop_path(),
        virtual_desktop_mapping: HashMap::new(),
    }
}

pub fn save_config(config: &KimiConfig) -> Result<(), String> {
    ensure_kimi_dir()?;
    
    let content = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(CONFIG_FILE, content).map_err(|e| format!("Не удалось сохранить конфиг: {}", e))
}

pub fn get_desktop_folder_path(id: i32) -> PathBuf {
    PathBuf::from(KIMI_DIR).join(format!("Desktop{}", id))
}

pub fn count_files_in_dir(path: &PathBuf) -> i32 {
    fs::read_dir(path)
        .map(|entries| entries.filter_map(|e| e.ok()).count() as i32)
        .unwrap_or(0)
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

fn get_default_desktop_path() -> String {
    #[cfg(windows)]
    {
        let path = crate::desktop::get_desktop_path_from_registry()
            .unwrap_or_else(|_| fallback_desktop_path());
        
        // Никогда не сохраняем C:\Kimi\* как оригинальный путь!
        if path.starts_with(KIMI_DIR) {
            return fallback_desktop_path();
        }
        
        path
    }
    #[cfg(not(windows))]
    {
        fallback_desktop_path()
    }
}

fn fallback_desktop_path() -> String {
    let profile = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".to_string());
    format!("{}\\Desktop", profile)
}
