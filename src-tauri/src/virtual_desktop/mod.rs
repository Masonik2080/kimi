//! Модуль виртуальных рабочих столов Windows

pub mod api;
pub mod keyboard;

pub use keyboard::ensure_virtual_desktops_exist;
