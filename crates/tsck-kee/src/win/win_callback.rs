#![allow(unused)]
use anyhow::{Context, Result};
use chrono::format::{DelayedFormat, StrftimeItems};
use flume::{Receiver, Sender};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Write,
    sync::OnceLock,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use windows::{
    Win32::{
        Foundation::{FALSE, HWND, LPARAM, RECT, TRUE, WPARAM},
        UI::{
            Accessibility::{HWINEVENTHOOK, SetWinEventHook},
            WindowsAndMessaging::*,
        },
    },
    core::BOOL,
};

use crate::win::{
    win_api::{self},
    win_event::WinEvent,
    win_manager::WindowManagerEvent,
};

lazy_static! {
    static ref BORDER_STATE: Mutex<HashMap<String, Box<Border>>> = Mutex::new(HashMap::new());
    static ref WINDOWS_BORDERS: Mutex<HashMap<isize, String>> = Mutex::new(HashMap::new());
    static ref APP_INFO_LIST: Mutex<HashMap<isize, AppInfo>> = Mutex::new(HashMap::new());
    static ref EVENTS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref APP_WHITELIST: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
}
pub static CHANNEL: OnceLock<(Sender<WindowManagerEvent>, Receiver<WindowManagerEvent>)> =
    OnceLock::new();

pub fn wm_event_channel() -> &'static (Sender<WindowManagerEvent>, Receiver<WindowManagerEvent>) {
    CHANNEL.get_or_init(|| flume::bounded(20))
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AppPosition {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AppSize {
    pub width: i32,
    pub height: i32,
}

macro_rules! appinfo {
    ($window:ident) => {
        if let (Some(exe), Some(title), Some(class), Some(exe_path)) = (
            $window.exe(),
            $window.title(),
            $window.class(),
            $window.process_path(),
        ) {
            Some(AppInfo {
                hwnd: $window.hwnd,
                exe: exe,
                position: $window.position(),
                size: $window.size(),
                exe_path,
                title: title,
                class: class,
            })
        } else {
            None
        }
    };
    ($window:ident,$size:expr,$pos:expr) => {
        if let (Some(exe), Some(title), Some(class), Some(exe_path)) = (
            $window.exe(),
            $window.title(),
            $window.class(),
            $window.process_path(),
        ) {
            Some(AppInfo {
                hwnd: $window.hwnd,
                exe: exe,
                position: $pos,
                size: $size,
                exe_path,
                title: title,
                class: class,
            })
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! update_app_list {
    ($window:ident,$status:ident) => {
        if let (Some(exe), Some(title), Some(class), Some(exe_path)) = (
            $window.exe(),
            $window.title(),
            $window.class(),
            $window.process_path(),
        ) {
            ActiveAppInfos::update_app_list(
                AppInfo {
                    hwnd: $window.hwnd,
                    exe: exe,
                    position: $window.position(),
                    size: $window.size(),
                    exe_path,
                    title: title,
                    class: class,
                },
                AppStatus::$status,
            )
        }
    };
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AppInfo {
    pub hwnd: isize,
    pub exe: String,
    pub exe_path: String,
    pub size: AppSize,
    pub position: AppPosition,
    pub title: String,
    pub class: String,
}

pub enum AppStatus {
    Create,
    Delete,
    Update,
    Init,
}

fn timestamp<'a>() -> DelayedFormat<StrftimeItems<'a>> {
    let format = "%H:%M:%S";
    if let Some(tz) = chrono::FixedOffset::east_opt(7 * 3600) {
        let now = chrono::Utc::now().with_timezone(&tz);
        now.format(format)
    } else {
        chrono::Utc::now().format(format)
    }
}

static WHITELIST: [&'static str; 4] = ["zed", "notepad", "zen", "whatsapp.root"];

pub struct ActiveAppInfos;
impl ActiveAppInfos {
    pub fn init() {
        WHITELIST
            .iter()
            .for_each(|f| ActiveAppInfos::update_whitelist(f));
    }
    fn update_whitelist(app: &'static str) {
        let mut whitelist = APP_WHITELIST.lock();
        whitelist.push(app);
    }
    fn allow(exe: &str) -> bool {
        let name = std::path::Path::new(exe)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(exe);

        APP_WHITELIST
            .lock()
            .iter()
            .any(|w| name.eq_ignore_ascii_case(w))
    }
    pub fn with_app_list<R>(f: impl FnOnce(&HashMap<isize, AppInfo>) -> R) -> R {
        let list = APP_INFO_LIST.lock();
        f(&list)
    }
    pub fn with_app_list_mut<R>(f: impl FnOnce(&mut HashMap<isize, AppInfo>) -> R) -> R {
        let mut list = APP_INFO_LIST.lock();
        f(&mut list)
    }

    pub fn debug_list() {
        let list = APP_INFO_LIST.lock();
        list.iter()
            .for_each(|(_id, i)| println!("{:>10} {}", i.hwnd, i.exe));
        println!("{} \x1b[3m[{}]\x1b[0m", "=".repeat(20), timestamp());
    }
    pub fn update_app_list(info: AppInfo, status: AppStatus) {
        if Self::allow(&info.exe) {
            let mut list = APP_INFO_LIST.lock();
            let exe_name = info.exe.clone();
            match status {
                AppStatus::Create | AppStatus::Update | AppStatus::Init => {
                    if let Some(old_info) = list.get_mut(&info.hwnd) {
                        *old_info = info;
                    } else {
                        list.insert(info.hwnd, info);
                    }
                }
                AppStatus::Delete => {
                    list.remove(&info.hwnd);
                }
            }
        }
    }
}

pub fn wm_event_tx() -> Sender<WindowManagerEvent> {
    wm_event_channel().0.clone()
}

pub fn wm_event_rx() -> Receiver<WindowManagerEvent> {
    wm_event_channel().1.clone()
}

enum Notification {
    Update(Option<isize>),
    ForceUpdate,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
enum WinKind {
    Single,
    Stack,
    #[default]
    Unfocused,
    Floating,
}
struct Border {
    hwnd: isize,
    window_kind: WinKind,
}
impl From<isize> for Border {
    fn from(value: isize) -> Self {
        Self {
            hwnd: value,
            window_kind: WinKind::Unfocused,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Deserialize, PartialEq)]
pub struct Window {
    pub hwnd: isize,
}
impl std::fmt::Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | {} | {} | {}",
            self.hwnd,
            self.exe().unwrap_or_default(),
            self.title().unwrap_or_default(),
            self.class().unwrap_or_default()
        )
    }
}

impl Window {
    pub const fn hwnd(self) -> HWND {
        HWND(self.hwnd as *mut std::ffi::c_void)
    }

    pub fn exe(self) -> Option<String> {
        win_api::exe(self.hwnd())
    }
    pub fn title(self) -> Option<String> {
        win_api::get_window_title(self.hwnd())
    }
    pub fn process_path(self) -> Option<String> {
        win_api::get_process_path(self.hwnd())
    }
    pub fn class(self) -> Option<String> {
        win_api::real_window_class_w(self.hwnd())
    }
    pub fn position(self) -> AppPosition {
        win_api::get_app_position(self.hwnd())
    }
    pub fn size(self) -> AppSize {
        win_api::get_app_size(self.hwnd())
    }
}

impl From<HWND> for Window {
    fn from(value: HWND) -> Self {
        Self {
            hwnd: value.0 as isize,
        }
    }
}
pub fn spawn_win_callback_service() {
    unsafe { EnumWindows(Some(wc_init_applist), LPARAM(0)) };
    let (_, _) = wm_event_channel();
    unsafe {
        SetWinEventHook(
            EVENT_MIN,
            EVENT_MAX,
            None,
            Some(win_event_hook),
            0,
            0,
            WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS,
        )
    };
    let mut msg: MSG = MSG::default();
    loop {
        unsafe {
            if !GetMessageW(&mut msg, None, 0, 0).as_bool() {
                break;
            }
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
#[derive(Debug, Clone)]
pub struct BorderInfo {
    border_hwnd: isize,
    window_kind: WinKind,
}
impl BorderInfo {
    fn hwnd(&self) -> HWND {
        HWND(self.border_hwnd as *mut std::ffi::c_void)
    }
}

pub fn window_border(hwnd: isize) -> Option<BorderInfo> {
    let id = WINDOWS_BORDERS.lock().get(&hwnd)?.clone();
    BORDER_STATE.lock().get(&id).map(|b| BorderInfo {
        border_hwnd: b.hwnd,
        window_kind: b.window_kind,
    })
}

pub extern "system" fn wc_init_applist(hwnd: HWND, lparam: LPARAM) -> BOOL {
    match unsafe { IsWindowVisible(hwnd) } == FALSE {
        true => return TRUE,
        false => (),
    }

    let win = Window::from(hwnd);
    let (size, pos) = win_api::get_app_rect(hwnd);

    if let Some(appinfo) = appinfo!(win, size, pos) {
        ActiveAppInfos::update_app_list(appinfo.clone(), AppStatus::Init);
    }

    TRUE
}
pub extern "system" fn wc_event_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let appinfos = unsafe { &mut *(lparam.0 as *mut Vec<AppInfo>) };
    match unsafe { IsWindowVisible(hwnd) } == FALSE {
        true => return TRUE,
        false => (),
    }

    let win = Window::from(hwnd);
    let (size, pos) = win_api::get_app_rect(hwnd);

    if let Some(appinfo) = appinfo!(win, size, pos) {
        ActiveAppInfos::update_app_list(appinfo.clone(), AppStatus::Init);
        appinfos.push(appinfo);
    }

    TRUE
}

pub extern "system" fn win_event_hook(
    win_event_hook: HWINEVENTHOOK,
    event: u32,
    hwnd: HWND,
    id_object: i32,
    id_child: i32,
    id_event_thread: u32,
    dwms_event_time: u32,
) {
    if id_object != OBJID_WINDOW.0 || id_child != 0 {
        return;
    }
    if unsafe { GetAncestor(hwnd, GA_ROOT) } != hwnd {
        return;
    }

    let style = WINDOW_STYLE(unsafe { GetWindowLongW(hwnd, GWL_STYLE) } as u32);
    if !style.contains(WS_OVERLAPPEDWINDOW) {
        return;
    }

    let ex_style = WINDOW_EX_STYLE(unsafe { GetWindowLongW(hwnd, GWL_EXSTYLE) } as u32);
    if ex_style.contains(WS_EX_TOOLWINDOW) {
        return;
    }

    if hwnd.is_invalid() {
        return;
    }

    let window = Window::from(hwnd);
    let win_event = match WinEvent::try_from(event) {
        Ok(event) => event,
        Err(_) => return,
    };

    // if matches!(
    //     win_event,
    //     WinEvent::ObjectLocationChange | WinEvent::ObjectDestroy
    // ) {
    //     let border_info = window_border(hwnd.0 as isize);
    //     if let Some(border_info) = border_info {
    //         unsafe {
    //             _ = SendNotifyMessageW(
    //                 border_info.hwnd(),
    //                 event,
    //                 WPARAM(0),
    //                 LPARAM(hwnd.0 as isize),
    //             );
    //         }
    //     }
    // }

    let event = match WindowManagerEvent::from_win_event(win_event, window) {
        None => {
            return;
        }
        Some(event) => event,
    };
    wm_event_tx().send(event).expect("could not send event");
}

fn collect_event(event: String) {
    let mut vec = EVENTS.lock();
    if !vec.contains(&event) {
        vec.push(event);
    }
}

#[cfg(test)]
mod mod_win {
    use super::*;
    #[test]
    fn test_trim() {
        let white_list = &["zed", "notepad", "zen", "whatsapp.root"];
        let zed = "Zed.exe";
        let name = std::path::Path::new(zed)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(zed);

        let is_allowed = white_list.iter().any(|w| name.eq_ignore_ascii_case(w));
        println!("{}{}{:?}", zed, name, is_allowed);
    }

    #[test]
    fn test_service() {
        crate::win::api::app_begin();
    }
}
