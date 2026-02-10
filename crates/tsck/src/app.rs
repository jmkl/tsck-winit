use crate::event::{
    EventPayload, UserEvent, WinLevel, WindowInfoExt, WsMessagePayload, WsPayloadContent,
};
use crate::ipc::{IpcHelper, IpcRequest, IpcResponse};
use crate::photoshop::customscripts::CustomScripts;
use crate::protocol::setup_custom_protocol;
use crate::store::config::{ConfigParser, PluginConf, WindowPosition, WindowSize, WindowSrc};
use crate::utils::animation::map_value;
use crate::utils::download::dl_image;
use crate::utils::img::load_icon;
use crate::utils::url_encode;
use crate::utils::winview_util::webview_bounds;
use crate::utils::youtubeapi::YoutubeApi;
use crate::{ChannelBus, WindowState, dp, log_debug, log_error, log_warn, response_success};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tsck_kee::list_windows;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::platform::windows::{WindowAttributesWindows, WindowExtWindows};
use winit::window::{Window, WindowAttributes, WindowId};
use wry::dpi::{LogicalPosition, Position, Size};
use wry::http::Request;
use wry::{Rect, WebView, WebViewBuilder};

pub struct TsckApp {
    windows: HashMap<WindowId, WindowState>,
    channel_bus: Arc<ChannelBus>,
    dev_url: String,
    plugin_config: Arc<HashMap<String, PluginConf>>,
}

macro_rules! get_window_by_label {
    ($self:expr,$label:expr,|$ws:ident|$space:block) => {
        if let Some((_, ws)) = $self.windows.iter().find(|(_, ws)| ws.title == $label) {
            let $ws = ws;
            $space
        }
    };
}

macro_rules! get_window {
    ($self:expr, $wid:ident,|$ws:ident|$space:block) => {
        if let Some(window_id) = $wid {
            if let Some(ws) = $self.windows.get(&window_id) {
                let $ws = ws;
                $space
            }
        }
    };
}

macro_rules! to_frontend {
    ($self:expr,$content:expr) => {
        get_window_by_label!($self, "main", |ws| {
            if let Ok(payload) = IpcHelper::compile(EventPayload::FrontEnd.to_string(), $content) {
                _ = ws.webview.evaluate_script(&payload);
            }
        });
    };
}
type UE = UserEvent;
impl TsckApp {
    pub fn new(channel_bus: Arc<ChannelBus>) -> Self {
        let config = ConfigParser::parse(include_str!("../tsck.json"));
        Self::init(&channel_bus, &config);
        Self {
            windows: HashMap::new(),
            channel_bus,
            dev_url: config.dev_url,
            plugin_config: Arc::new(config.plugins),
        }
    }
    pub fn reload_config(&mut self) {
        let config = ConfigParser::parse(include_str!("../tsck.json"));
        self.plugin_config = Arc::new(config.plugins);
    }
    fn init(sender: &Arc<ChannelBus>, config: &ConfigParser) {
        for (name, plugin) in &config.plugins {
            if plugin.with_window && plugin.window.as_ref().is_some_and(|w| w.auto_launch) {
                let _ = sender.send((UE::LaunchPlugin(name.clone()), None, None));
            }
        }
    }

    fn process_cmd(&mut self, event_loop: &dyn ActiveEventLoop) {
        let receiver = self.channel_bus.get_receiver();
        let config = self.plugin_config.clone();
        while let Ok((cmd, request, window_id)) = receiver.try_recv() {
            match cmd {
                UE::ReloadConfig => {
                    self.reload_config();
                }
                UE::ActivateWorkSpace(_) => {
                    get_window_by_label!(self, "workspace", |ws| {
                        if let Ok(payload) =
                            IpcHelper::compile(EventPayload::FrontEnd.to_string(), cmd)
                        {
                            _ = ws.webview.evaluate_script(&payload);
                        }
                    });
                }
                UE::IsOnTop => {
                    get_window!(self, window_id, |ws| {
                        request.map(|req| -> anyhow::Result<()> {
                            let is_on_top = {
                                let guard = ws.win_level.lock();
                                guard.clone()
                            };
                            response_success!(ws.webview, req, is_on_top);
                            Ok(())
                        });
                    });
                }
                UE::GetActiveWindows => {
                    get_window!(self, window_id, |ws| {
                        request.map(|req| -> anyhow::Result<()> {
                            let apps: Vec<WindowInfoExt> = list_windows()
                                .iter()
                                .map(|w| WindowInfoExt {
                                    title: w.title(),
                                    exe: w.name(),
                                    class: w.class().to_string(),
                                    size: WindowSize::new(w.size().width, w.size().height),
                                    position: WindowPosition::new(w.position().x, w.position().y),
                                    workspace: w.workspace(),
                                })
                                .collect();
                            response_success!(ws.webview, req, apps);
                            Ok(())
                        });
                    });
                }
                UE::Minimize => {
                    get_window!(self, window_id, |ws| {
                        ws.window.set_minimized(true);
                    });
                }
                UE::Maximize => {
                    get_window!(self, window_id, |ws| {
                        if ws.window.is_maximized() {
                            ws.window.set_maximized(false);
                        } else {
                            ws.window.set_maximized(true);
                        }
                    });
                }
                UE::DragWindow => {
                    if let Some(window_id) = window_id {
                        if let Some(ws) = self.windows.get(&window_id) {
                            _ = ws.window.drag_window();
                        }
                    }
                }
                UE::CloseWindow => {
                    get_window!(self, window_id, |ws| {
                        if &ws.title == "main" {
                            event_loop.exit();
                        }
                    });
                    {
                        if let Some(window_id) = window_id {
                            self.windows.remove(&window_id);
                            if self.windows.is_empty() {
                                event_loop.exit();
                            }
                        }
                    }
                }
                UE::MouseDown(_, _) => {
                    // println!("UE::MouseDown");
                }
                UE::MouseMove(_, _) => {
                    // println!("UE::MouseMove");
                }
                UE::LaunchPlugin(plugin_name) => {
                    if let Some((_, ws)) = self
                        .windows
                        .iter()
                        .find(|(_, ws)| &ws.title == &plugin_name)
                    {
                        ws.window.focus_window();
                        _ = ws.webview.focus();
                        return;
                    }
                    if let Some(plugin_conf) = config.get(&plugin_name) {
                        _ = self.create_window(event_loop, plugin_name, plugin_conf);
                    }
                }
                UE::EvalJs(js) => {
                    get_window!(self, window_id, |ws| {
                        _ = ws.webview.evaluate_script(&js);
                    });
                }
                UE::LoadUrl(url) => {
                    get_window!(self, window_id, |ws| {
                        _ = ws.webview.load_url(&url);
                    });
                }
                UE::NavigateWebview(url) => {
                    get_window!(self, window_id, |ws| {
                        _ = ws.webview.load_url(&url);
                    });
                }
                UE::ZoomWebview(scale_factor) => {
                    get_window!(self, window_id, |ws| {
                        let scale = (scale_factor as f64).clamp(0.3, 1.5);
                        _ = ws.webview.zoom(scale);
                    });
                }

                UE::SetWindowLevel(level, label) => {
                    get_window_by_label!(self, label, |ws| {
                        let window_level = match level {
                            WinLevel::Top => winit::window::WindowLevel::AlwaysOnTop,
                            WinLevel::Normal => winit::window::WindowLevel::Normal,
                            WinLevel::Bottom => winit::window::WindowLevel::AlwaysOnBottom,
                        };
                        {
                            let mut guard = ws.win_level.lock();
                            *guard = level;
                        }
                        ws.window.set_window_level(window_level);
                    });
                }
                UE::GetReadableHotkee => {
                    get_window!(self, window_id, |ws| {
                        request.map(|_req| -> anyhow::Result<()> {
                            // let kees: Vec<ReadableHotkee> = self
                            //     .channel_bus
                            //     .get_app_config()
                            //     .kees
                            //     .iter()
                            //     .map(|(k, f)| kee_to_readable_hotkee(k, f))
                            //     .collect();
                            // response_success!(ws.webview, req, kees);
                            Ok(())
                        });
                    });
                }

                UE::SetWindowSize(window_size) => {
                    get_window!(self, window_id, |ws| {
                        _ = ws.window.request_surface_size(window_size.to_size());
                    });
                }
                UE::SetWindowPosition(window_position) => {
                    get_window!(self, window_id, |ws| {
                        ws.window.set_outer_position(window_position.to_position());
                    });
                }

                UE::TransformWindow(payload) => {
                    if let Some(window_id) = window_id {
                        if let Some(ws) = self.windows.get(&window_id) {
                            let pos = ws.window.outer_position().unwrap_or_default();
                            let size = ws.window.surface_size();
                            let to_pos = payload.to_pos.unwrap_or((pos.x, pos.y));
                            let to_size = payload
                                .to_size
                                .unwrap_or((size.width as i32, size.height as i32));
                            let start_time = Instant::now();
                            let duration = Duration::from_millis(payload.duration);
                            const TARGET_FPS: u64 = 60;
                            const FRAME_TIME: Duration = Duration::from_millis(1000 / TARGET_FPS);

                            while start_time.elapsed() < duration {
                                let t = start_time.elapsed().as_secs_f64() / duration.as_secs_f64();
                                let eased_t = payload.easing.evaluate(t.min(1.0));

                                let new_pos =
                                    map_value((pos.x, pos.y), (to_pos.0, to_pos.1), eased_t);
                                let new_size = map_value(
                                    (size.width as i32, size.height as i32),
                                    (to_size.0, to_size.1),
                                    eased_t,
                                );

                                ws.window.set_outer_position(Position::Physical(
                                    winit::dpi::PhysicalPosition {
                                        x: new_pos.0 as i32,
                                        y: new_pos.1 as i32,
                                    },
                                ));
                                _ = ws.window.request_surface_size(Size::Physical(
                                    winit::dpi::PhysicalSize {
                                        width: new_size.0 as u32,
                                        height: new_size.1 as u32,
                                    },
                                ));

                                std::thread::sleep(FRAME_TIME);
                            }
                            ws.window.set_outer_position(Position::Physical(
                                winit::dpi::PhysicalPosition {
                                    x: to_pos.0 as i32,
                                    y: to_pos.1 as i32,
                                },
                            ));
                            _ = ws.window.request_surface_size(Size::Physical(
                                winit::dpi::PhysicalSize {
                                    width: to_size.0 as u32,
                                    height: to_size.1 as u32,
                                },
                            ));
                        }
                    };
                }

                UE::IncomingWebsocketMessage(_id, message) => {
                    match serde_json::from_str::<WsMessagePayload>(&message) {
                        Ok(mut m) => {
                            m.from_server = true;
                            match m.msg_type {
                                crate::event::WsPayloadType::Whatsapp => {
                                    let text = match &m.content {
                                        WsPayloadContent::Text(t) => t.as_str(),
                                        _ => "unknown",
                                    };
                                    to_frontend!(
                                        self,
                                        UE::WhatsappUpdate {
                                            msg_type: text.to_string(),
                                        }
                                    );
                                }
                                crate::event::WsPayloadType::ShowLoading => {
                                    let loading = match &m.content {
                                        WsPayloadContent::Bool(t) => *t,
                                        _ => false,
                                    };
                                    to_frontend!(self, UE::LoadingState { loading });
                                }

                                crate::event::WsPayloadType::CreateThumb => {
                                    if let WsPayloadContent::Text(png_file) = m.content {
                                        if let Some(smart_object_item) =
                                            self.channel_bus.smartobject_create_thumb(&png_file)
                                        {
                                            to_frontend!(
                                                self,
                                                UE::SmartobjectThumbnailUpdate {
                                                    name: smart_object_item.name,
                                                    thumb: smart_object_item.thumb
                                                }
                                            );
                                        }
                                    }
                                }
                                crate::event::WsPayloadType::SelectionMode => {
                                    if let WsPayloadContent::SelectionBound(selection_bound) =
                                        m.content
                                    {
                                        to_frontend!(self, UE::SelectionChanged(selection_bound));
                                    }
                                }
                                crate::event::WsPayloadType::RawFilterInfo => {
                                    if let WsPayloadContent::RawFilterDataType(rawfilter_data) =
                                        m.content
                                    {
                                        to_frontend!(self, UE::RawFilterDataUpdate(rawfilter_data));
                                    }
                                }

                                crate::event::WsPayloadType::FacerestorePreviewImage => {
                                    if let WsPayloadContent::List(img) = m.content {
                                        to_frontend!(self, UE::FacerestorePreviewImage(img));
                                    }
                                }
                                crate::event::WsPayloadType::PushToWhatsapp => {
                                    if let WsPayloadContent::Text(filepath) = m.content {
                                        let cfg = self.channel_bus.get_app_config();
                                        let url = cfg.whatsapp_url;
                                        let result = format!(
                                            "http://{}/send-thumbnail?filepath={}&channel={}",
                                            url,
                                            url_encode(&filepath),
                                            url_encode(&m.channel.unwrap_or("".to_string()))
                                        );
                                        if let Ok(mut response) = ureq::get(&result).call()
                                            && let Ok(reply) = response.body_mut().read_to_string()
                                        {
                                            println!("ResponseBody {}", reply);
                                        }
                                    }
                                }
                                crate::event::WsPayloadType::PipRanges => {
                                    if let WsPayloadContent::Listi32(values) = m.content {
                                        to_frontend!(self, UE::PipRanges(values));
                                    }
                                }
                                crate::event::WsPayloadType::RawFilterTextPipRange => {
                                    if let WsPayloadContent::RawFilterTextPipRange(rtp) = m.content
                                    {
                                        to_frontend!(self, UE::RawFilterTextPipRange(rtp));
                                    }
                                }
                                _ => {
                                    log_warn!("WSMESSAGE UNIMPLEMENTED", dp!(m));
                                }
                            }
                        }
                        Err(err) => {
                            log_error!("Error parsing WsMessagePayload", dp!(err));
                        }
                    }
                }
                UE::CyclePages(_) => {
                    get_window_by_label!(self, "main", |ws| {
                        if let Ok(payload) =
                            IpcHelper::compile(EventPayload::FrontEnd.to_string(), cmd)
                        {
                            ws.window.focus_window();
                            _ = ws.webview.evaluate_script(&payload);
                        }
                    });
                }
                UE::FilterSmartObjectChunk {
                    query,
                    page,
                    per_page,
                } => {
                    let result = self
                        .channel_bus
                        .smartobject_filter_chunk(&query, page, per_page);

                    get_window!(self, window_id, |ws| {
                        request.map(|req| -> anyhow::Result<()> {
                            response_success!(ws.webview, req, result);
                            Ok(())
                        });
                    });
                }
                UE::FetchTextureCategories => {
                    if let Some(result) = self.channel_bus.texture_get_all_categories() {
                        get_window!(self, window_id, |ws| {
                            request.map(|req| -> anyhow::Result<()> {
                                response_success!(ws.webview, req, result);
                                Ok(())
                            });
                        });
                    }
                }
                UE::FetchTextures(category, page, limit) => {
                    if let Some(result) = self
                        .channel_bus
                        .texture_get_texture_chunk(category, page, limit)
                    {
                        get_window!(self, window_id, |ws| {
                            request.map(|req| -> anyhow::Result<()> {
                                response_success!(ws.webview, req, result);
                                Ok(())
                            });
                        });
                    }
                }
                UE::GetAppConfig => {
                    get_window!(self, window_id, |ws| {
                        let app_config = self.channel_bus.get_app_config();
                        request.map(|req| -> anyhow::Result<()> {
                            response_success!(ws.webview, req, app_config);
                            Ok(())
                        });
                    });
                }
                UE::SetAppConfig(config) => {
                    self.channel_bus.update_app_config(config);
                }
                UE::UpdateTextureFavorite(id, favorite) => {
                    self.channel_bus.texture_update_favorite(id, favorite);
                }
                UE::RequestCommand => {
                    get_window!(self, window_id, |ws| {
                        let result = self.channel_bus.cmd_request_command();
                        request.map(|req| -> anyhow::Result<()> {
                            response_success!(ws.webview, req, result);
                            Ok(())
                        });
                    });
                }
                UE::RunCommand(app_name) => {
                    self.channel_bus.cmd_run_command(app_name);
                }
                UE::KillCommand(cmd_name) => {
                    self.channel_bus.cmd_kill_command(cmd_name);
                }
                UE::BroadcastToFrontEnd(target, script) => {
                    get_window_by_label!(self, target, |ws| {
                        _ = ws.webview.evaluate_script(&script);
                    });
                }
                UE::WindowFocusChange(_) => {
                    get_window_by_label!(self, "main", |ws| {
                        if let Ok(payload) =
                            IpcHelper::compile(EventPayload::FrontEnd.to_string(), cmd)
                        {
                            _ = ws.webview.evaluate_script(&payload);
                        }
                    });
                }
                UE::FunctionCall { .. }
                | UE::ApplyRawFilter(..)
                | UE::PerformSelectionToImage
                | UE::PerformLayerToImage
                | UE::GenerateImage
                | UE::ApplyTriColor { .. }
                | UE::AppendComfyUIOutput { .. } => {
                    self.channel_bus.broadcast_to_websocket(cmd);
                }
                UE::Template { template } => {
                    self.channel_bus.broadcast_to_websocket(UE::Template {
                        template: template.modify(),
                    });
                }
                UE::UpdateRawfilterTemplates(templates) => {
                    _ = self
                        .channel_bus
                        .get_config()
                        .lock()
                        .config_store
                        .set(|c| c.rawfilter_template = templates);
                }
                UE::GoogleDownloadImage(url) => {
                    let comfyui_root = &self.channel_bus.get_app_config().comfyui_root;
                    if let Some(payload) = dl_image(&url, comfyui_root) {
                        self.channel_bus.broadcast_to_websocket(payload);
                    }
                }
                UE::SmartObjectDelete(smart_object_item) => {
                    self.channel_bus.smartobject_delete(smart_object_item);
                    get_window_by_label!(self, "main", |ws| {
                        request.map(|req| -> anyhow::Result<()> {
                            response_success!(ws.webview, req, true);
                            Ok(())
                        });
                    });
                }
                UE::YoutubeTitle(video_url) => {
                    if let Ok(response) = YoutubeApi::new().fetch(&video_url) {
                        get_window_by_label!(self, "main", |ws| {
                            request.map(|req| -> anyhow::Result<()> {
                                response_success!(ws.webview, req, response);
                                Ok(())
                            });
                        });
                    }
                }
                UserEvent::ExecuteScript(script) => {
                    let cs_script = self.channel_bus.get_app_config().store_root;
                    let customscripts = Path::new(&cs_script).join("customscripts");
                    if let Ok(scr) = CustomScripts::new().script_to_str(&customscripts, &script) {
                        log_debug!(&scr);
                        if let Ok(payload) = serde_json::to_string(&UE::ExecuteScript(scr)) {
                            self.channel_bus.ws_send_to_all(payload);
                        }
                    }
                }
                UserEvent::FocusWindow(label) => {
                    get_window_by_label!(self, label, |ws| {
                        ws.window.focus_window();
                    });
                }

                UserEvent::FocusPage(..)
                | UserEvent::ToggleShadow
                | UserEvent::ToggleWindowLevel
                | UserEvent::ToggleCompactMode => {
                    to_frontend!(self, cmd);
                }
                UE::YoutubeTitleWithApiKey(video_url, api_key) => {
                    if let Ok(response) = YoutubeApi::with_api(api_key).fetch(&video_url) {
                        get_window_by_label!(self, "main", |ws| {
                            request.map(|req| -> anyhow::Result<()> {
                                response_success!(ws.webview, req, response);
                                Ok(())
                            });
                        });
                    }
                }

                _ => {
                    log_error!("UNIMPLEMENTED", dp!(cmd));
                }
            }
        }
    }
    fn ipc_handler(&self, window_id: WindowId) -> impl Fn(Request<String>) + 'static {
        let channel_bus = self.channel_bus.clone();
        move |req| {
            let body = req.body();
            if let Ok(request) = serde_json::from_str::<IpcRequest>(body) {
                let request = Arc::from(request);
                if let Some(data) = request.data.clone() {
                    if let Ok(ev) = serde_json::from_value::<UserEvent>(data) {
                        _ = channel_bus.send((ev, Some(request), Some(window_id)));
                    }
                }
            }
        }
    }
    fn create_webview(
        &mut self,
        window: &Box<dyn Window>,
        window_id: WindowId,
        plugin_conf: &PluginConf,
    ) -> anyhow::Result<(WebView, Option<WebView>)> {
        let conf = plugin_conf.window.as_ref().unwrap();
        let size = window
            .surface_size()
            .to_logical::<u32>(window.scale_factor());
        let toolbar_panel = &conf.toolbar_panel;
        let scale_factor = conf.webview_zoom_factor;
        let mut builder = WebViewBuilder::new().with_bounds(Rect {
            position: Position::Logical(LogicalPosition::new(0.0, 0.0)),
            size: Size::Physical(size.to_physical(scale_factor)),
        });

        let webview = match &conf.window_src {
            WindowSrc::Local(local_path) => {
                builder = builder
                    .with_transparent(conf.transparent)
                    .with_initialization_script(include_str!("../scripts/init.js"))
                    .with_ipc_handler(self.ipc_handler(window_id));
                let view = {
                    match cfg!(debug_assertions) {
                        true => builder
                            .with_url(format!("{}{}", self.dev_url, local_path))
                            .build(&window)?,
                        false => setup_custom_protocol(builder, local_path).build(&window)?,
                    }
                };
                (view, None)
            }
            WindowSrc::Web(url, page) => {
                let custom_script = include_str!("../scripts/web.js");
                let channel_bus = self.channel_bus.clone();
                let view = builder
                    .with_url(url)
                    .with_new_window_req_handler(move |url, _| {
                        _ = channel_bus.send((UE::NavigateWebview(url), None, Some(window_id)));
                        wry::NewWindowResponse::Deny
                    })
                    .with_initialization_script(include_str!("../scripts/init.js"))
                    .with_initialization_script(custom_script)
                    .with_bounds(webview_bounds("view", size, toolbar_panel))
                    .with_ipc_handler(self.ipc_handler(window_id))
                    .with_transparent(conf.transparent)
                    .build_as_child(&window)?;
                view.zoom(scale_factor)?;
                let panel_builder = WebViewBuilder::new()
                    .with_initialization_script(include_str!("../scripts/init.js"))
                    .with_bounds(webview_bounds("panel", size, toolbar_panel))
                    .with_accept_first_mouse(true)
                    .with_ipc_handler(self.ipc_handler(window_id))
                    .with_transparent(conf.transparent);
                let panel = {
                    match cfg!(debug_assertions) {
                        true => panel_builder
                            .with_url(format!("{}/Toolbar?page={}", self.dev_url, page))
                            .build_as_child(&window)?,
                        false => {
                            setup_custom_protocol(panel_builder, &format!("/Toolbar?page={}", page))
                                .build_as_child(&window)?
                        }
                    }
                };
                (view, Some(panel))
            }
        };

        Ok(webview)
    }
    fn create_window(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        title: String,
        plugin_conf: &PluginConf,
    ) -> anyhow::Result<()> {
        if let Some(window_conf) = plugin_conf.window.as_ref() {
            let window_level = match window_conf.always_on_top {
                true => winit::window::WindowLevel::AlwaysOnTop,
                false => winit::window::WindowLevel::Normal,
            };
            let window_attr = WindowAttributesWindows::default()
                .with_system_backdrop(winit::platform::windows::BackdropType::None)
                .with_skip_taskbar(window_conf.skip_taskbar)
                .with_no_redirection_bitmap(true);
            let attr = WindowAttributes::default()
                .with_title(&title)
                .with_window_icon(load_icon(include_bytes!("../ic.png").to_vec()))
                .with_transparent(window_conf.transparent)
                .with_decorations(window_conf.decorations)
                .with_surface_size(window_conf.window_size.to_size())
                .with_position(window_conf.window_position.to_position())
                .with_platform_attributes(Box::new(window_attr))
                .with_window_level(window_level);
            let window = event_loop.create_window(attr)?;
            window.set_undecorated_shadow(window_conf.shadow);
            window.set_cursor_hittest(window_conf.receive_cursor_event)?;
            _ = window.request_surface_size(window_conf.window_size.to_size());
            let winconf = Arc::new(window_conf.clone());
            let window_id = window.id();
            let (webview, panel) = self.create_webview(&window, window_id.clone(), plugin_conf)?;
            let window_state = WindowState::new(self, title, window, webview, panel, winconf)?;
            self.windows.insert(window_id, window_state);
        }

        Ok(())
    }
}

impl ApplicationHandler for TsckApp {
    fn can_create_surfaces(&mut self, _: &dyn ActiveEventLoop) {
        log_error!("On Ready");
        // self.create_window(event_loop, "tsck".to_string())
        //     .expect("Error creating window");
    }
    fn proxy_wake_up(&mut self, _: &dyn ActiveEventLoop) {
        for w in self.windows.values() {
            w.window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.process_cmd(event_loop);
        let window = {
            match self.windows.get_mut(&window_id) {
                Some(window) => window,
                None => return,
            }
        };

        match event {
            WindowEvent::CloseRequested => {
                self.windows.remove(&window_id);
            }
            WindowEvent::Focused(focus) => {
                self.channel_bus
                    .send((UE::WindowFocusChange(focus), None, None));
            }
            WindowEvent::SurfaceResized(size) => {
                window.resize(size);
            }

            _ => {}
        }
    }
    fn about_to_wait(&mut self, event_loop: &dyn ActiveEventLoop) {
        self.process_cmd(event_loop);
    }
}
