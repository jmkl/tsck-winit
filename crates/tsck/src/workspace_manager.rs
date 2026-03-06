#![allow(unused)]
use crate::{UserEvent, sender};
use flume::Sender;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use ts_rs::TS;
use tsck_kee::{AppInfo, AppPosition};
use winit::event_loop::EventLoopProxy;

use crate::{
    dp,
    event::{ChannelEvent, TS_PATH},
    log_error,
};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export,export_to=TS_PATH)]
pub struct WorkspaceEntry {
    pub app: String,
    pub title: String,
    pub hwnd: isize,
    pub workspace: usize,
    pub real_pos: (i32, i32),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export,export_to=TS_PATH)]
pub struct WorkspacePayload {
    entries: Vec<WorkspaceEntry>,
    active: usize,
    count: usize,
}

pub enum WorkspaceApiEvent {
    Delete(isize),
    Create,
    FocusChange,
    Update,
    Unknown(String),
}

pub struct WorkspaceManager {
    active: usize,
    active_app_index: usize,
    count: usize,
    app_cycle_whitelist: Vec<String>,
    entries: Vec<WorkspaceEntry>,
}
impl WorkspaceManager {
    pub fn new() -> Self {
        Self {
            active: 0,
            active_app_index: 0,
            count: 2,
            app_cycle_whitelist: Vec::new(),
            entries: Vec::new(),
        }
    }

    pub fn on_event(
        &mut self,
        event: WorkspaceApiEvent,
        sender: Sender<ChannelEvent>,
        proxy: Arc<EventLoopProxy>,
    ) {
        match event {
            WorkspaceApiEvent::Delete(hwnd) => {
                self.remove_entry(hwnd);
                crate::sender!(
                    sender,
                    WorkspaceSendPayload,
                    self.get_workspace_payload(),
                    proxy
                );
            }
            WorkspaceApiEvent::Create => {
                self.update_entries(&tsck_kee::api::app_get_all());
                sender!(
                    sender,
                    WorkspaceSendPayload,
                    self.get_workspace_payload(),
                    proxy
                );
            }
            WorkspaceApiEvent::FocusChange => {
                if let Some(active) = tsck_kee::api::app_get_active() {
                    sender!(sender, WorkspaceAppFocusChange, active.hwnd as i32, proxy);
                }
            }
            WorkspaceApiEvent::Update => {
                let apps = tsck_kee::api::app_get_all();
                self.update_entries(&apps);
                sender!(
                    sender,
                    WorkspaceSendPayload,
                    self.get_workspace_payload(),
                    proxy
                );
            }
            WorkspaceApiEvent::Unknown(event) => {
                log_error!(event);
            }
        }
    }

    pub fn count_workspace(&self) -> usize {
        self.count
    }
    pub fn focus_next_app(&mut self) {
        self.active_app_index = (self.active_app_index + 1) % self.entries.len();
        self.bring_app_to_front();
    }
    pub fn focus_prev_app(&mut self) {
        let len = self.entries.len();
        if len > 0 {
            self.active_app_index = (self.active_app_index + len - 1) % len;
        }
        self.bring_app_to_front();
    }
    fn bring_app_to_front(&mut self) {
        if let Some(app) = self.entries.get(self.active_app_index) {
            tsck_kee::api::app_bring_to_front(app.hwnd);
        }
    }
    pub fn get_workspace_payload(&self) -> WorkspacePayload {
        WorkspacePayload {
            entries: self.entries.clone(),
            active: self.active,
            count: self.count,
        }
    }
    pub fn get_active_app(&self) -> Option<AppInfo> {
        tsck_kee::api::app_get_active()
    }
    pub fn get_active_workspace(&self) -> i32 {
        self.active as i32
    }
    pub fn get_entries(&self) -> Vec<WorkspaceEntry> {
        self.entries.clone()
    }
    pub fn cycle_workspace(&mut self) -> usize {
        self.active = (self.active + 1) % self.entries.len();
        self.active
    }
    pub fn add_new_workspace(&mut self) {
        self.count += 1;
    }

    pub fn update_entries(&mut self, apps: &Vec<AppInfo>) {
        for app in apps {
            if let Some(a) = self.entries.iter_mut().find(|f| f.hwnd == app.hwnd) {
                a.app = app.exe.clone();
                a.title = app.title.clone();
                a.real_pos = app.position.to_tuple();
            } else {
                self.entries.push(WorkspaceEntry {
                    app: app.exe.clone(),
                    title: app.title.clone(),
                    hwnd: app.hwnd,
                    workspace: 0,
                    real_pos: app.position.to_tuple(),
                });
            }
        }
    }

    pub fn add_entry(&mut self, app: &AppInfo) {
        self.entries.push(WorkspaceEntry {
            app: app.exe.clone(),
            title: app.title.clone(),
            hwnd: app.hwnd,
            workspace: 0,
            real_pos: app.position.to_tuple(),
        });
    }
    pub fn add_entries(&mut self, apps: &Vec<AppInfo>) {
        for app in apps {
            if let Some(a) = self.entries.iter_mut().find(|f| f.hwnd == app.hwnd) {
                a.app = app.exe.clone();
                a.workspace = 0;
            } else {
                self.entries.push(WorkspaceEntry {
                    app: app.exe.clone(),
                    title: app.title.clone(),
                    hwnd: app.hwnd,
                    workspace: 0,
                    real_pos: app.position.to_tuple(),
                });
            }
        }
    }
    pub fn remove_entry(&mut self, hwnd: isize) {
        self.entries.retain(|f| f.hwnd != hwnd);
    }

    pub fn switch_to_next_workspace(&mut self) -> usize {
        self.active = (self.active + 1) % self.count;
        self.active
    }
    fn move_window(&mut self) {}
    pub fn move_app_to_workspace(&mut self, app: &AppInfo) {
        let entry_len = self.entries.len();
        if entry_len <= 0 {
            return;
        }
        let mut workspace = (self.active + 1) % entry_len;
        if let Some(entry) = self.entries.iter_mut().find(|f| f.hwnd == app.hwnd) {
            workspace = (entry.workspace + 1) % entry_len;
            entry.workspace = workspace;
        } else {
            self.entries.push(WorkspaceEntry {
                app: app.exe.clone(),
                hwnd: app.hwnd,
                title: app.title.clone(),
                workspace,
                real_pos: app.position.to_tuple(),
            });
        }
        tsck_kee::api::app_move_to(
            app,
            app.position.x,
            app.position.y - (workspace as i32 * 1440),
        );
        log_error!(workspace, app.position.y, self.active);
    }
}
