//! Модуль управления рабочими столами

pub mod icons;
pub mod manager;
mod registry;
mod shell;

pub use manager::validate_and_restore_if_needed;
pub use registry::get_desktop_path_from_registry;
pub use shell::set_desktop_path;
