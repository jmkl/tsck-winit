#![allow(unused)]
use crate::win::win_callback::{AppInfo, AppPosition, AppSize, Window, wc_event_callback};
use anyhow::{Context, Result};
use std::{ffi::OsString, os::windows::ffi::OsStringExt};
use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE, HWND, LPARAM, RECT},
        System::Threading::{
            OpenProcess, PROCESS_ACCESS_RIGHTS, PROCESS_NAME_FORMAT, PROCESS_QUERY_INFORMATION,
            PROCESS_VM_READ, QueryFullProcessImageNameW,
        },
        UI::WindowsAndMessaging::{
            EnumWindows, GetClassNameW, GetWindowRect, GetWindowTextLengthW, GetWindowTextW,
            GetWindowThreadProcessId,
        },
    },
    core::PWSTR,
};

macro_rules! as_ptr {
    ($value:expr) => {
        $value as *mut core::ffi::c_void
    };
}

fn list_active_apps() -> Vec<AppInfo> {
    let mut apps: Vec<AppInfo> = Vec::new();
    let lparam = LPARAM(&mut apps as *mut _ as isize);
    unsafe { EnumWindows(Some(wc_event_callback), lparam) };
    apps
}
pub fn get_app_rect(hwnd: HWND) -> (AppSize, AppPosition) {
    let rect = {
        unsafe {
            let mut rect = RECT::default();
            GetWindowRect(hwnd, &mut rect);
            rect
        }
    };
    let x = rect.left;
    let y = rect.top;
    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;
    (AppSize { width, height }, AppPosition { x, y })
}
pub fn get_app_position(hwnd: HWND) -> AppPosition {
    let (_, pos) = get_app_rect(hwnd);
    pos
}
pub fn get_app_size(hwnd: HWND) -> AppSize {
    let (size, _) = get_app_rect(hwnd);
    size
}
pub fn get_window_title(hwnd: HWND) -> Option<String> {
    unsafe {
        let length = GetWindowTextLengthW(hwnd);
        if length == 0 {
            return None;
        }

        let mut buffer: Vec<u16> = vec![0; (length + 1) as usize];
        let copied = GetWindowTextW(hwnd, &mut buffer);

        if copied > 0 {
            buffer.truncate(copied as usize);
            Some(OsString::from_wide(&buffer).to_string_lossy().into_owned())
        } else {
            None
        }
    }
}
pub fn get_process_path(hwnd: HWND) -> Option<String> {
    unsafe {
        // Get process ID
        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        if process_id == 0 {
            return None;
        }

        // Open process handle
        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            process_id,
        )
        .ok();

        if process_handle.is_none() {
            return None;
        }

        // Query executable path
        let mut path_buffer: Vec<u16> = vec![0; 1024];
        let mut size: u32 = path_buffer.len() as u32;

        let result = QueryFullProcessImageNameW(
            process_handle?,
            PROCESS_NAME_FORMAT(0),
            PWSTR(path_buffer.as_mut_ptr()),
            &mut size,
        )
        .ok();

        _ = CloseHandle(process_handle?);

        if result.is_some() && size > 0 {
            path_buffer.truncate(size as usize);
            Some(
                OsString::from_wide(&path_buffer)
                    .to_string_lossy()
                    .into_owned(),
            )
        } else {
            None
        }
    }
}
pub fn exe(hwnd: HWND) -> Option<String> {
    let result = get_process_path(hwnd)?.split('\\').next_back()?.to_string();
    Some(result)
}
pub fn window_thread_process_id(hwnd: isize) -> (u32, u32) {
    let mut process_id: u32 = 0;
    let thread_id = unsafe {
        GetWindowThreadProcessId(
            HWND(as_ptr!(hwnd)),
            Option::from(std::ptr::addr_of_mut!(process_id)),
        )
    };

    (process_id, thread_id)
}
fn open_process(
    access_rights: PROCESS_ACCESS_RIGHTS,
    inherit_handle: bool,
    process_id: u32,
) -> Result<HANDLE> {
    unsafe { OpenProcess(access_rights, inherit_handle, process_id) }
        .context("Failed to open access rights process")
}
pub fn close_process(handle: HANDLE) -> Result<()> {
    unsafe { CloseHandle(handle) }.context("Error close process")
}

pub fn process_handle(process_id: u32) -> Result<HANDLE> {
    open_process(PROCESS_QUERY_INFORMATION, false, process_id)
}
pub fn real_window_class_w(hwnd: HWND) -> Option<String> {
    let mut buffer: [u16; 256] = [0; 256];
    let copied = unsafe { GetClassNameW(hwnd, &mut buffer) };

    if copied > 0 {
        let class_name = OsString::from_wide(&buffer[..copied as usize]);
        Some(class_name.to_string_lossy().into_owned())
    } else {
        None
    }
}

#[cfg(test)]
mod test_winapi {
    use std::time::Instant;

    use crate::win::win_api;

    use super::*;
    #[test]
    fn list_active_app() {
        for i in 0..10 {
            let start = Instant::now();
            let apps = win_api::list_active_apps();
            println!("{}", "+".repeat(40));
            println!(
                "{:?}",
                apps.iter()
                    .map(|f| format!("{} {:>10}", f.exe, f.title))
                    .collect::<Vec<_>>()
            );
            println!("{}", "+".repeat(40));
            println!("{}ms", start.elapsed().as_millis());
        }
    }
}
