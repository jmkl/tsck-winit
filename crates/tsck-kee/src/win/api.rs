#![allow(unused)]
use std::{fmt::write, io::Stdout};

use anyhow::Context;

use crate::{
    update_app_list,
    win::{
        win_callback::{self, ActiveAppInfos, AppInfo, AppStatus},
        win_manager::WindowManagerEvent,
    },
};
pub fn app_get_all() {}
pub fn app_update_workspace() {}
pub fn whitelist_add() {}
pub fn whitelist_remove() {}
pub fn whitelist_get_all() {}

macro_rules! wrap {
    ($($what:expr)*) => {
      $(format!("\x1b[39m{}\x1b[0m",$what))*
    };
}

fn debug() {
    ActiveAppInfos::with_app_list(|app| {
        for (idx, (_, ai)) in app.iter().enumerate() {
            println!(
                "{}\n{}={}\nsize: [{},{}]\npos:[{},{}]",
                "=".repeat(20),
                ai.hwnd,
                ai.exe,
                ai.size.width,
                ai.size.height,
                ai.position.x,
                ai.position.y
            );
        }
    });
}
pub fn app_begin() {
    std::thread::spawn(app_begin_lock);
}
pub fn app_begin_lock() {
    ActiveAppInfos::init();
    std::thread::spawn(|| {
        while let Ok(win_event) = win_callback::wm_event_rx().recv() {
            match win_event {
                WindowManagerEvent::Destroy(win_event, window) => {
                    update_app_list!(window, Delete);
                    println!("update ::{}", win_event.to_string());
                }
                WindowManagerEvent::Create(win_event, window) => {
                    update_app_list!(window, Create);
                    println!("update ::{}", win_event.to_string());
                }

                WindowManagerEvent::Cloak(win_event, window) => {
                    update_app_list!(window, Update);
                    println!("update ::{}", win_event.to_string());
                }
                WindowManagerEvent::FocusChange(win_event, window) => {
                    update_app_list!(window, Update);
                    println!("{}::{:?}", win_event.to_string(), window.exe());
                }
                WindowManagerEvent::Minimize(win_event, window)
                | WindowManagerEvent::Uncloak(win_event, window)
                | WindowManagerEvent::MoveResizeStart(win_event, window)
                | WindowManagerEvent::MouseCapture(win_event, window)
                | WindowManagerEvent::TitleUpdate(win_event, window) => {}
                WindowManagerEvent::MoveResizeEnd(win_event, window)
                | WindowManagerEvent::Hide(win_event, window)
                | WindowManagerEvent::Show(win_event, window) => {
                    println!("update ::{}", win_event.to_string());
                    update_app_list!(window, Update);
                }
                WindowManagerEvent::Manage(window)
                | WindowManagerEvent::Unmanage(window)
                | WindowManagerEvent::Raise(window) => {}
                _ => {
                    println!("update ::{}", win_event.to_string());
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
    fn start_service() {
        crate::win::api::app_begin_lock();
    }
}
