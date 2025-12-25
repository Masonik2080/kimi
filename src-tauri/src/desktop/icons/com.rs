//! Windows COM API для работы с иконками рабочего стола (IFolderView2)

use super::layout::{DesktopIconsLayout, IconPosition};
use std::path::PathBuf;

use windows::core::{Interface, HSTRING, VARIANT};
use windows::Win32::Foundation::POINT;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoTaskMemFree, CLSCTX_LOCAL_SERVER, COINIT_APARTMENTTHREADED,
};
use windows::Win32::UI::Shell::Common::ITEMIDLIST;
use windows::Win32::UI::Shell::{
    IFolderView2, IShellFolder, IShellWindows, SHGetNameFromIDList, ShellWindows,
    SIGDN_PARENTRELATIVEPARSING, SVGIO_BACKGROUND, SVSI_POSITIONITEM, SWC_DESKTOP,
    SWFO_NEEDDISPATCH, FVM_ICON,
};

const FWF_AUTOARRANGE: u32 = 0x00000001;
const FWF_SNAPTOGRID: u32 = 0x00000002;
const GRID_SIZE: i32 = 75;

// ─────────────────────────────────────────────────────────────────────────────
// Public API
// ─────────────────────────────────────────────────────────────────────────────

pub fn get_current_icon_positions() -> Result<DesktopIconsLayout, String> {
    unsafe {
        let folder_view = get_desktop_folder_view()?;
        let mut layout = DesktopIconsLayout::default();

        let count = folder_view
            .ItemCount(SVGIO_BACKGROUND)
            .map_err(|e| format!("ItemCount: {}", e))?;

        for i in 0..count as i32 {
            if let Ok(pidl) = folder_view.Item(i) {
                if !pidl.is_null() {
                    if let (Ok(point), Ok(name)) = (folder_view.GetItemPosition(pidl), get_item_name(pidl)) {
                        if !name.is_empty() {
                            layout.icons.insert(name, IconPosition { x: point.x, y: point.y });
                        }
                    }
                    CoTaskMemFree(Some(pidl as *const _));
                }
            }
        }

        Ok(layout)
    }
}

pub fn restore_icon_positions(layout: &DesktopIconsLayout, _desktop_path: &PathBuf) -> Result<(), String> {

    if layout.icons.is_empty() {
        return Ok(());
    }

    unsafe {
        let folder_view = get_desktop_folder_view()?;
        let desktop_folder = get_shell_folder(&folder_view)?;
        
        disable_auto_arrange(&folder_view)?;
        wait_for_icons(&folder_view, layout.icons.len())?;
        
        for (filename, pos) in &layout.icons {
            position_icon(&folder_view, &desktop_folder, filename, pos);
        }

        Ok(())
    }
}

pub fn disable_auto_arrange_public() -> Result<(), String> {
    unsafe {
        let folder_view = get_desktop_folder_view()?;
        disable_auto_arrange(&folder_view)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

unsafe fn get_desktop_folder_view() -> Result<IFolderView2, String> {
    let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

    let shell_windows: IShellWindows =
        CoCreateInstance(&ShellWindows, None, CLSCTX_LOCAL_SERVER)
            .map_err(|e| format!("CoCreateInstance: {}", e))?;

    let mut hwnd: i32 = 0;
    let desktop = shell_windows
        .FindWindowSW(
            &VARIANT::default(),
            &VARIANT::default(),
            SWC_DESKTOP,
            &mut hwnd,
            SWFO_NEEDDISPATCH,
        )
        .map_err(|e| format!("FindWindowSW: {}", e))?;

    let service_provider: windows::Win32::System::Com::IServiceProvider = desktop
        .cast()
        .map_err(|e| format!("Cast to IServiceProvider: {}", e))?;

    let sid = windows::core::GUID::from_u128(0x4C96BE40_915C_11CF_99D3_00AA004AE837);

    let shell_browser: windows::Win32::UI::Shell::IShellBrowser = service_provider
        .QueryService(&sid)
        .map_err(|e| format!("QueryService: {}", e))?;

    let shell_view = shell_browser
        .QueryActiveShellView()
        .map_err(|e| format!("QueryActiveShellView: {}", e))?;

    shell_view
        .cast()
        .map_err(|e| format!("Cast to IFolderView2: {}", e))
}

unsafe fn get_shell_folder(folder_view: &IFolderView2) -> Result<IShellFolder, String> {
    let folder_unknown: windows::core::IUnknown = folder_view
        .GetFolder()
        .map_err(|e| format!("GetFolder: {}", e))?;

    folder_unknown
        .cast()
        .map_err(|e| format!("Cast to IShellFolder: {}", e))
}

unsafe fn get_item_name(pidl: *const ITEMIDLIST) -> Result<String, String> {
    let name_ptr = SHGetNameFromIDList(pidl, SIGDN_PARENTRELATIVEPARSING)
        .map_err(|e| format!("SHGetNameFromIDList: {}", e))?;

    if name_ptr.is_null() {
        return Ok(String::new());
    }

    let name = name_ptr.to_string().unwrap_or_default();
    CoTaskMemFree(Some(name_ptr.0 as *const _));
    Ok(name)
}

unsafe fn disable_auto_arrange(folder_view: &IFolderView2) -> Result<(), String> {
    folder_view
        .SetCurrentViewMode(FVM_ICON.0 as u32)
        .map_err(|e| format!("SetCurrentViewMode: {}", e))?;

    let current_flags = folder_view
        .GetCurrentFolderFlags()
        .map_err(|e| format!("GetCurrentFolderFlags: {}", e))?;

    let new_flags = current_flags & !(FWF_AUTOARRANGE | FWF_SNAPTOGRID);
    let mask = FWF_AUTOARRANGE | FWF_SNAPTOGRID;

    folder_view
        .SetCurrentFolderFlags(mask, new_flags)
        .map_err(|e| format!("SetCurrentFolderFlags: {}", e))
}

unsafe fn wait_for_icons(folder_view: &IFolderView2, expected: usize) -> Result<(), String> {
    use std::{thread, time::Duration};
    
    let expected = expected as i32;
    
    for _ in 0..15 {
        let current = folder_view.ItemCount(SVGIO_BACKGROUND).unwrap_or(0);
        if current >= 1 && current >= expected / 2 {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(200));
    }
    
    Ok(()) // Timeout — пробуем расставить что есть
}

unsafe fn position_icon(
    folder_view: &IFolderView2,
    desktop_folder: &IShellFolder,
    filename: &str,
    pos: &IconPosition,
) {
    let name_hstring = HSTRING::from(filename);
    let mut pidl: *mut ITEMIDLIST = std::ptr::null_mut();

    let hr = desktop_folder.ParseDisplayName(
        None,
        None,
        &name_hstring,
        None,
        &mut pidl,
        std::ptr::null_mut(),
    );

    if hr.is_ok() && !pidl.is_null() {
        let (x, y) = snap_to_grid(pos.x, pos.y);
        let point = POINT { x, y };
        let pidl_array = [pidl as *const ITEMIDLIST];

        let _ = folder_view.SelectAndPositionItems(
            1,
            pidl_array.as_ptr(),
            Some(&point),
            SVSI_POSITIONITEM.0 as u32,
        );

        CoTaskMemFree(Some(pidl as *const _));
    }
}

fn snap_to_grid(x: i32, y: i32) -> (i32, i32) {
    ((x / GRID_SIZE) * GRID_SIZE, (y / GRID_SIZE) * GRID_SIZE)
}
