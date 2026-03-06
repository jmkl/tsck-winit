#![allow(unused)]
use std::{fmt::write, io::Stdout, os::raw::c_void};

use anyhow::Context;
use flume::{Receiver, Sender, bounded};
use windows::Win32::Foundation::HWND;

struct WorkspaceInfo {
    workspace_count: i32,
    active_workspace: i32,
}

use crate::{
    update_app_list,
    win::{
        self,
        win_callback::{self, ActiveAppInfos, AppInfo, AppStatus, BORDER_MANAGER},
        win_manager::WindowManagerEvent,
    },
};
pub fn app_get_active() -> Option<AppInfo> {
    ActiveAppInfos::get_active_app()
}
pub fn app_get_all() -> Vec<AppInfo> {
    ActiveAppInfos::applist_get_all()
}
pub fn app_move_to(app_info: &AppInfo, x: i32, y: i32) {
    ActiveAppInfos::move_window(app_info, x, y);
}
pub fn app_bring_to_front(hwnd: isize) {
    ActiveAppInfos::bring_to_front(hwnd);
}
pub fn app_resize(app_info: &AppInfo, width: i32, height: i32) {
    ActiveAppInfos::resize_window(app_info, width, height);
}
pub fn app_update_workspace() {}

macro_rules! wrap {
    ($($what:expr)*) => {
      $(format!("\x1b[39m{}\x1b[0m",$what))*
    };
}

fn debug() {
    // ActiveAppInfos::with_app_list(|app| {
    //     for (idx, (_, ai)) in app.iter().enumerate() {
    //         println!(
    //             "{}\n{}={}\nsize: [{},{}]\npos:[{},{}]",
    //             "=".repeat(20),
    //             ai.hwnd,
    //             ai.exe,
    //             ai.size.width,
    //             ai.size.height,
    //             ai.position.x,
    //             ai.position.y
    //         );
    //     }
    // });
}
pub enum WindowEvent {
    Delete(isize),
    Create,
    FocusChange,
    Update,
    Unknown(String),
}
pub fn app_begin<F>(f: F)
where
    F: FnOnce(Receiver<WindowEvent>),
{
    let (tx, rx) = bounded::<WindowEvent>(1);
    let t = tx.clone();
    println!("Window Service Running in background...");
    std::thread::spawn(|| app_begin_lock(tx));
    f(rx);
}

pub fn maximize_window() {
    ActiveAppInfos::maximize_window();
}

pub fn app_begin_lock(sender: Sender<WindowEvent>) {
    ActiveAppInfos::init();
    std::thread::spawn(move || {
        while let Ok(win_event) = win_callback::wm_event_rx().recv() {
            match win_event {
                WindowManagerEvent::Destroy(win_event, window) => {
                    sender.send(WindowEvent::Delete(window.hwnd));
                    update_app_list!(window, Delete);
                }
                WindowManagerEvent::Create(win_event, window) => {
                    update_app_list!(window, Create);
                    sender.send(WindowEvent::Create);
                }

                WindowManagerEvent::Cloak(win_event, window) => {
                    update_app_list!(window, Update);
                    sender.send(WindowEvent::Update);
                }
                WindowManagerEvent::LocationChange(win_event, window) => {
                    // update_app_list!(window, Update);
                    // sender.send(WindowEvent::Update);
                }
                WindowManagerEvent::FocusChange(win_event, window) => {
                    update_app_list!(window, Update);
                    sender.send(WindowEvent::FocusChange);
                }
                WindowManagerEvent::Minimize(win_event, window)
                | WindowManagerEvent::Uncloak(win_event, window)
                | WindowManagerEvent::MoveResizeStart(win_event, window)
                | WindowManagerEvent::MouseCapture(win_event, window)
                | WindowManagerEvent::TitleUpdate(win_event, window) => {}
                WindowManagerEvent::MoveResizeEnd(win_event, window)
                | WindowManagerEvent::Hide(win_event, window)
                | WindowManagerEvent::Show(win_event, window) => {
                    update_app_list!(window, Update);
                    sender.send(WindowEvent::Update);
                }
                WindowManagerEvent::Manage(window)
                | WindowManagerEvent::Unmanage(window)
                | WindowManagerEvent::Raise(window) => {}
                _ => {
                    sender.send(WindowEvent::Unknown(win_event.to_string()));
                }
            }
        }
    });
    win_callback::spawn_win_callback_service();
}
#[cfg(test)]
mod test_api {
    use super::*;
    #[test]
    fn start_service() {}
}
