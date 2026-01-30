use std::sync::Arc;

use crate::{
    ipc::IpcRequest,
    store::config::{ToolbarPanel, WindowPosition, WindowSize},
    utils::animation::AnimationPayload,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use winit::window::WindowId;

macro_rules! impl_display {
	($enum_name:ident, $prefix:literal, { $($variant:ident),* $(,)? }) => {
        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = match self {
                    $(
                        $enum_name::$variant => concat!(stringify!($enum_name),"::", stringify!($variant)),
                    )*
                };
                 write!(f, "{}|{}", $prefix, s.to_uppercase())

            }
        }
    };
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export,export_to=TS_PATH)]
pub enum EventPayload {
    #[ts(rename = "tsck::event|EVENTPAYLOAD::FRONTEND")]
    FrontEnd,
    #[ts(rename = "tsck::event|EVENTPAYLOAD::BACKEND")]
    BackEnd,
    #[ts(rename = "tsck::event|EVENTPAYLOAD::HOTKEE")]
    Hotkee,
}

impl_display!(EventPayload, "tsck::event", {
    FrontEnd,
    BackEnd,
    Hotkee
});

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export,export_to=TS_PATH)]
pub struct WindowInfoExt {
    pub title: String,
    pub exe: String,
    pub class: String,
    pub size: WindowSize,
    pub position: WindowPosition,
    pub workspace: i32,
}

pub type ChannelEvent = (UserEvent, Option<Arc<IpcRequest>>, Option<WindowId>);
pub const TS_PATH: &str = "../../../js/frontend/src/lib/tsck.types.ts";
#[derive(Clone, Deserialize, Serialize, Debug, TS)]
#[serde(tag = "type", content = "value")]
#[ts(export,export_to=TS_PATH)]
pub enum UserEvent {
    ReloadConfig,
    Minimize,
    Maximize,
    DragWindow,
    CloseWindow,
    MouseDown(i32, i32),
    MouseMove(i32, i32),
    LaunchPlugin(String),
    EvalJs(String),
    LoadUrl(String),
    IsOnTop,
    NavigateWebview(String),
    ZoomWebview(f32),
    UpdateToolbarPanel(ToolbarPanel),
    SetWindowOnTop(bool),
    SetWindowDecorated(bool),
    SetWindowShadow(bool),
    SetWindowSize(WindowSize),
    SetWindowPosition(WindowPosition),
    TransformWindow(AnimationPayload),
    SetIgnoreCursorEvent(bool),
    GoogleDownloadImage(String),
    ActivateWorkSpace(i64),
    CyclePages(i32),
    GetActiveWindows,
    IncomingWebsocketMessage(u64, String),
}
