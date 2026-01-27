use serde::{Deserialize, Serialize};
use winit::window::WindowId;

use crate::{
    config::{ToolbarPanel, WindowPosition, WindowSize},
    utils::animation::AnimationPayload,
};

pub type ChannelEvent = (UserEvent, Option<WindowId>);

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum UserEvent {
    Minimize,
    Maximize,
    DragWindow,
    CloseWindow,
    MouseDown(i32, i32),
    MouseMove(i32, i32),
    LaunchPlugin(String),
    EvalJs(String),
    LoadUrl(String),
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
}
