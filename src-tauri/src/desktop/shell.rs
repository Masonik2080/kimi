//! Взаимодействие с Windows Shell (Explorer)

#[cfg(windows)]
use windows::core::PCWSTR;
#[cfg(windows)]
use windows::Win32::Foundation::{BOOL, HANDLE, HWND, LPARAM, WPARAM};
#[cfg(windows)]
use windows::Win32::UI::Shell::{
    SHChangeNotify, SHSetKnownFolderPath, FOLDERID_Desktop, KF_FLAG_DONT_UNEXPAND,
    SHCNE_ASSOCCHANGED, SHCNF_FLUSH, SHCNF_IDLIST,
};
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    EnumChildWindows, FindWindowExW, FindWindowW, GetClassNameW, PostMessageW, SendMessageW,
    HWND_BROADCAST, WM_KEYDOWN, WM_KEYUP, WM_SETTINGCHANGE,
};

const VK_F5: usize = 0x74;

// ─────────────────────────────────────────────────────────────────────────────
// Public API
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(windows)]
pub fn set_desktop_path(path: &str) -> Result<(), String> {
    let wide_path: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        SHSetKnownFolderPath(
            &FOLDERID_Desktop,
            KF_FLAG_DONT_UNEXPAND.0 as u32,
            HANDLE::default(),
            PCWSTR::from_raw(wide_path.as_ptr()),
        )
        .map_err(|e| format!("SHSetKnownFolderPath failed: {}", e))?;

        SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST | SHCNF_FLUSH, None, None);
        broadcast_settings_change();
    }

    std::thread::sleep(std::time::Duration::from_millis(100));
    refresh_desktop();

    Ok(())
}

#[cfg(not(windows))]
pub fn set_desktop_path(_path: &str) -> Result<(), String> {
    Err("Только для Windows".to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(windows)]
unsafe fn broadcast_settings_change() {
    let environment: Vec<u16> = "Environment\0".encode_utf16().collect();
    let _ = SendMessageW(
        HWND_BROADCAST,
        WM_SETTINGCHANGE,
        WPARAM(0),
        LPARAM(environment.as_ptr() as isize),
    );
}

#[cfg(windows)]
fn refresh_desktop() {
    unsafe {
        if let Some(hwnd) = find_desktop_listview() {
            send_f5(hwnd);
        }

        if let Ok(progman) = FindWindowW(w!("Progman"), PCWSTR::null()) {
            send_f5(progman);
        }
    }
}

#[cfg(windows)]
unsafe fn send_f5(hwnd: HWND) {
    let _ = PostMessageW(hwnd, WM_KEYDOWN, WPARAM(VK_F5), LPARAM(0));
    std::thread::sleep(std::time::Duration::from_millis(50));
    let _ = PostMessageW(hwnd, WM_KEYUP, WPARAM(VK_F5), LPARAM(0));
}

#[cfg(windows)]
fn find_desktop_listview() -> Option<HWND> {
    unsafe {
        // Пробуем Progman
        if let Ok(progman) = FindWindowW(w!("Progman"), PCWSTR::null()) {
            if let Some(lv) = find_listview_in_parent(progman) {
                return Some(lv);
            }
        }

        // Пробуем WorkerW
        let mut worker = HWND::default();
        loop {
            match FindWindowExW(HWND::default(), worker, w!("WorkerW"), PCWSTR::null()) {
                Ok(hwnd) if !hwnd.is_invalid() => {
                    worker = hwnd;
                    if let Some(lv) = find_listview_in_parent(hwnd) {
                        return Some(lv);
                    }
                }
                _ => break,
            }
        }

        None
    }
}

#[cfg(windows)]
fn find_listview_in_parent(parent: HWND) -> Option<HWND> {
    find_child_by_class(parent, "SHELLDLL_DefView")
        .and_then(|shell_view| find_child_by_class(shell_view, "SysListView32"))
}

#[cfg(windows)]
fn find_child_by_class(parent: HWND, class_name: &str) -> Option<HWND> {
    struct SearchData {
        target: Vec<u16>,
        found: Option<HWND>,
    }

    unsafe extern "system" fn callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let data = &mut *(lparam.0 as *mut SearchData);
        let mut buf = [0u16; 256];
        let len = GetClassNameW(hwnd, &mut buf);

        if len > 0 {
            let class: Vec<u16> = buf[..len as usize].to_vec();
            let target: Vec<u16> = data.target.iter().take_while(|&&c| c != 0).copied().collect();

            if class == target {
                data.found = Some(hwnd);
                return BOOL(0);
            }
        }
        BOOL(1)
    }

    let mut data = SearchData {
        target: class_name.encode_utf16().chain(std::iter::once(0)).collect(),
        found: None,
    };

    let _ = unsafe { EnumChildWindows(parent, Some(callback), LPARAM(&mut data as *mut _ as isize)) };
    data.found
}

#[cfg(windows)]
use windows::core::w;
