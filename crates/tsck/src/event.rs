use std::sync::Arc;

use crate::{
    app_config::{AppConfig, RawFilterTemplate},
    ipc::IpcRequest,
    photoshop::{
        SmartObjectItem,
        ps_misc::{Bounds, RawFilterTextPipRange, SelectionBound},
        ps_rawfilter::RawFilterDataType,
        ps_template::Template,
    },
    store::config::{ToolbarPanel, WindowPosition, WindowSize},
    ts_struct,
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
    #[ts(rename = "tsck::event|EVENTPAYLOAD::COMMAND")]
    Command,
}

impl_display!(EventPayload, "tsck::event", {
    FrontEnd,
    BackEnd,
    Command,
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
pub const TS_PATH: &str = "../../../js/@tsck/src/tsck.types.ts";

#[derive(Clone, Deserialize, Serialize, Debug, TS)]
#[ts(export,export_to=TS_PATH)]
#[serde(untagged)]
pub enum FuncCallArgs {
    Number(i32),
    Text(String),
    Boolean(bool),
}
ts_struct! {
    path = TS_PATH,
    pub enum WinLevel{
    Normal,
    Top,
    Bottom
    }
}
#[derive(Clone, Deserialize, Serialize, Debug, TS)]
#[serde(tag = "type", content = "value")]
#[ts(export,export_to=TS_PATH)]
pub enum UserEvent {
    GetReadableHotkee,
    WindowFocusChange(bool),
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
    SetWindowLevel(WinLevel, String),
    SetWindowDecorated(bool),
    SetWindowShadow(bool),
    SetWindowSize(WindowSize),
    SetWindowPosition(WindowPosition),
    TransformWindow(AnimationPayload),
    SetIgnoreCursorEvent(bool),
    GoogleDownloadImage(String),
    ActivateWorkSpace(i32),
    CyclePages(i32),
    GetActiveWindows,
    IncomingWebsocketMessage(u32, String),
    //PHOTOSHOP
    Template {
        template: Template,
    },
    LoadingState {
        loading: bool,
    },
    UpdateTextureFavorite(i32, bool),
    FetchTextures(String, usize, usize),
    FetchTextureCategories,
    SmartObjectDelete(SmartObjectItem),
    SmartobjectThumbnailUpdate {
        name: String,
        thumb: String,
    },
    FilterSmartObjectChunk {
        query: String,
        page: usize,
        per_page: usize,
    },
    FunctionCall {
        func: String,
        args: Vec<FuncCallArgs>,
    },
    GetAppConfig,
    SetAppConfig(AppConfig),
    SelectionChanged(SelectionBound),
    RawFilterDataUpdate(RawFilterDataType),
    PerformSelectionToImage,
    PerformLayerToImage,
    GenerateImage,
    FacerestorePreviewImage(Vec<String>),
    AppendComfyUIOutput {
        images: Vec<String>,
        bounds: Bounds,
    },

    RawFilterTextPipRange(RawFilterTextPipRange),
    ApplyRawFilter(RawFilterDataType),
    ApplyTriColor {
        tri_color: Vec<String>,
        position: Vec<i32>,
    },
    PipRanges(Vec<i32>),
    RequestCommand,
    RunCommand(String),
    KillCommand(String),
    ReloadCommandConfig,
    UpdateRawfilterTemplates(Vec<RawFilterTemplate>),
    UpdateActiveApps,
    ExecuteScript(String),
    BroadcastToFrontEnd(String, String),
    ToggleShadow,
    YoutubeTitle(String),
    YoutubeTitleWithApiKey(String, String),
    ToggleCompactMode,
    FocusPage(i32),
    FocusWindow(String),
    ToggleWindowLevel,

    WhatsappUpdate {
        #[serde(rename = "type")]
        msg_type: String,
    },
}
ts_struct! {path = TS_PATH,
    #[serde(untagged)]
    pub enum WsPayloadContent {
        Text(String),
        Bool(bool),
        List(Vec<String>),
        Listi32(Vec<i32>),
        Tuple(String, String),
        SelectionBound(SelectionBound),
        RawFilterDataType(RawFilterDataType),
        RawFilterTextPipRange(RawFilterTextPipRange),
        Null,
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = TS_PATH)]
pub enum WsPayloadType {
    Whatsapp,
    ShowLoading,
    Unknown,
    CreateThumb,
    SelectionMode,
    RawFilterInfo,
    CropSelectionToImage,
    SelectedLayerToImage,
    GenerateImage,
    FacerestorePreviewImage,
    PushToWhatsapp,
    //- receive all textlayer top pos ---> plugin send it array of i32 -> PipRange: Vec<i32> PipRange(Vec<i32>)
    PipRanges, //- send tricolor data + piprange color:["#fff","#fff","#fff"],pipRanges:[0,2,3,4,5] PipNTriColor{colors:Vec<String>,pip_ranges:Vec<i32>}
    RawFilterTextPipRange, //it return {textlayer,pipRanges,rawfiltertype}
    ExecuteScript,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, export_to = TS_PATH)]
pub struct WsMessagePayload {
    pub from_server: bool,
    #[serde(rename = "type")]
    pub msg_type: WsPayloadType,
    pub content: WsPayloadContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub channel: Option<String>,
}
