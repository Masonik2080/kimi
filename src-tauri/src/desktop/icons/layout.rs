//! Структуры данных для позиций иконок

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IconPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DesktopIconsLayout {
    pub icons: HashMap<String, IconPosition>,
}
