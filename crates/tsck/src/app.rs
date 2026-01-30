use crate::event::{EventPayload, UserEvent, WindowInfoExt};
use crate::ipc::{IpcHelper, IpcRequest, IpcResponse};
use crate::store::config::{ConfigParser, PluginConf, WindowPosition, WindowSize, WindowSrc};
use crate::utils::animation::map_value;
use crate::utils::winview_util::webview_bounds;
use crate::{ChannelBus, WindowState, dp, log_debug, log_error, response_success};
use kee::list_windows;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
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
                let _ = sender.send((UserEvent::LaunchPlugin(name.clone()), None, None));
            }
        }
    }

    fn process_cmd(&mut self, event_loop: &dyn ActiveEventLoop) {
        let receiver = self.channel_bus.get_receiver();
        let config = self.plugin_config.clone();
        while let Ok((cmd, request, window_id)) = receiver.try_recv() {
            match cmd {
                UserEvent::ReloadConfig => {
                    self.reload_config();
                }
                UserEvent::ActivateWorkSpace(_) => {
                    get_window_by_label!(self, "workspace", |ws| {
                        if let Ok(payload) =
                            IpcHelper::compile(EventPayload::FrontEnd.to_string(), cmd)
                        {
                            _ = ws.webview.evaluate_script(&payload);
                        }
                    });
                }
                UserEvent::IsOnTop => {
                    get_window!(self, window_id, |ws| {
                        request.map(|req| -> anyhow::Result<()> {
                            let is_on_top = {
                                let guard = ws.on_top.lock();
                                *guard
                            };
                            response_success!(ws.webview, req, is_on_top);
                            Ok(())
                        });
                    });
                }
                UserEvent::GetActiveWindows => {
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
                UserEvent::Minimize => {
                    get_window!(self, window_id, |ws| {
                        ws.window.set_minimized(true);
                    });
                }
                UserEvent::Maximize => {
                    get_window!(self, window_id, |ws| {
                        if ws.window.is_maximized() {
                            ws.window.set_maximized(false);
                        } else {
                            ws.window.set_maximized(true);
                        }
                    });
                }
                UserEvent::DragWindow => {
                    if let Some(window_id) = window_id {
                        if let Some(ws) = self.windows.get(&window_id) {
                            _ = ws.window.drag_window();
                        }
                    }
                }
                UserEvent::CloseWindow => {
                    if let Some(window_id) = window_id {
                        self.windows.remove(&window_id);
                        if self.windows.is_empty() {
                            event_loop.exit();
                        }
                    }
                }
                UserEvent::MouseDown(_, _) => {
                    // println!("UserEvent::MouseDown");
                }
                UserEvent::MouseMove(_, _) => {
                    // println!("UserEvent::MouseMove");
                }
                UserEvent::LaunchPlugin(plugin_name) => {
                    if let Some((_, ws)) = self
                        .windows
                        .iter()
                        .find(|(_, ws)| &ws.title == &plugin_name)
                    {
                        log_debug!("WINDOW EXIST", "SET FOUCS");
                        ws.window.focus_window();
                        _ = ws.webview.focus();
                        return;
                    }
                    if let Some(plugin_conf) = config.get(&plugin_name) {
                        _ = self.create_window(event_loop, plugin_name, plugin_conf);
                    }
                }
                UserEvent::EvalJs(js) => {
                    get_window!(self, window_id, |ws| {
                        _ = ws.webview.evaluate_script(&js);
                    });
                }
                UserEvent::LoadUrl(url) => {
                    get_window!(self, window_id, |ws| {
                        _ = ws.webview.load_url(&url);
                    });
                }
                UserEvent::NavigateWebview(url) => {
                    get_window!(self, window_id, |ws| {
                        _ = ws.webview.load_url(&url);
                    });
                }
                UserEvent::ZoomWebview(scale_factor) => {
                    get_window!(self, window_id, |ws| {
                        let scale = (scale_factor as f64).clamp(0.3, 1.5);
                        _ = ws.webview.zoom(scale);
                    });
                }
                UserEvent::UpdateToolbarPanel(_toolbar_panel) => {
                    println!("UserEvent::UpdateToolbarPanel");
                }
                UserEvent::SetWindowOnTop(on_top) => {
                    get_window!(self, window_id, |ws| {
                        let window_level = match on_top {
                            true => winit::window::WindowLevel::AlwaysOnTop,
                            false => winit::window::WindowLevel::Normal,
                        };
                        {
                            let mut guard = ws.on_top.lock();
                            *guard = on_top;
                        }
                        ws.window.set_window_level(window_level);
                    });
                }
                UserEvent::SetWindowDecorated(_) => {
                    println!("UserEvent::SetWindowDecorated");
                }
                UserEvent::SetWindowShadow(_) => {
                    println!("UserEvent::SetWindowShadow");
                }
                UserEvent::SetWindowSize(window_size) => {
                    get_window!(self, window_id, |ws| {
                        _ = ws.window.request_surface_size(window_size.to_size());
                    });
                }
                UserEvent::SetWindowPosition(window_position) => {
                    get_window!(self, window_id, |ws| {
                        ws.window.set_outer_position(window_position.to_position());
                    });
                }
                UserEvent::SetIgnoreCursorEvent(_) => {
                    println!("UserEvent::SetIgnoreCursorEvent");
                }
                UserEvent::TransformWindow(payload) => {
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
                            // ws.window.set_outer_position(Position::Physical(
                            //     winit::dpi::PhysicalPosition {
                            //         x: to_pos.0 as i32,
                            //         y: to_pos.1 as i32,
                            //     },
                            // ));
                            // _ = ws.window.request_surface_size(Size::Physical(
                            //     winit::dpi::PhysicalSize {
                            //         width: to_size.0 as u32,
                            //         height: to_size.1 as u32,
                            //     },
                            // ));
                        }
                    };
                }
                UserEvent::GoogleDownloadImage(url) => {
                    log_error!("Google Download Image", url);
                }
                UserEvent::IncomingWebsocketMessage(_id, _message) => {
                    log_error!("UserEvent::IncomingWebsocketMessage");
                }
                UserEvent::CyclePages(direction) => {
                    get_window_by_label!(self, "main", |ws| {
                        if let Ok(payload) =
                            IpcHelper::compile(EventPayload::FrontEnd.to_string(), cmd)
                        {
                            _ = ws.webview.evaluate_script(&payload);
                        }
                    });
                }
                _ => {
                    log_debug!("Unimplemented", dp!(&cmd));
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
        let builder = WebViewBuilder::new()
            .with_url("http://localhost:5566")
            .with_bounds(Rect {
                position: Position::Logical(LogicalPosition::new(0.0, 0.0)),
                size: Size::Physical(size.to_physical(scale_factor)),
            })
            .with_initialization_script(include_str!("../scripts/init.js"))
            .with_ipc_handler(self.ipc_handler(window_id));

        let webview = match &conf.window_src {
            WindowSrc::Local(local_path) => {
                let view = builder
                    .with_url(format!("{}{}", self.dev_url, local_path))
                    .with_transparent(conf.transparent)
                    .build(&window)?;
                (view, None)
            }
            WindowSrc::Web(url, page) => {
                let custom_script: String = match plugin_conf.custom_script.as_ref() {
                    Some(script) => {
                        let parent = env!("CARGO_MANIFEST_DIR");
                        let script_path = Path::new(parent).join(script);
                        log_debug!("SCRIPT", script_path.to_string_lossy().to_string());
                        match std::fs::read_to_string(script_path) {
                            Ok(scripts) => scripts,
                            Err(err) => {
                                log_debug!("ERROR", dp!(err));
                                "{}".to_string()
                            }
                        }
                    }
                    None => "{}".to_string(),
                };

                let channel_bus = self.channel_bus.clone();
                let view = builder
                    .with_url(url)
                    .with_new_window_req_handler(move |url, _| {
                        _ = channel_bus.send((
                            UserEvent::NavigateWebview(url),
                            None,
                            Some(window_id),
                        ));
                        wry::NewWindowResponse::Deny
                    })
                    .with_initialization_script(custom_script)
                    .with_bounds(webview_bounds("view", size, toolbar_panel))
                    .with_transparent(conf.transparent)
                    .build_as_child(&window)?;
                view.zoom(scale_factor)?;
                let panel = WebViewBuilder::new()
                    .with_url(format!("{}/Toolbar?page={}", self.dev_url, page))
                    .with_initialization_script(include_str!("../scripts/init.js"))
                    .with_bounds(webview_bounds("panel", size, toolbar_panel))
                    .with_accept_first_mouse(true)
                    .with_ipc_handler(self.ipc_handler(window_id))
                    .with_transparent(conf.transparent)
                    .build_as_child(&window)?;
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
                .with_transparent(window_conf.transparent)
                .with_decorations(window_conf.decorations)
                .with_min_surface_size(window_conf.window_size.to_size())
                .with_position(window_conf.window_position.to_position())
                .with_platform_attributes(Box::new(window_attr))
                .with_window_level(window_level);
            let window = event_loop.create_window(attr)?;
            window.set_undecorated_shadow(window_conf.shadow);
            window.set_cursor_hittest(window_conf.receive_cursor_event)?;
            _ = window.request_surface_size(window_conf.window_size.to_size());
            let winconf = Arc::new(window_conf.clone());
            let window_id = window.id();
            let (webview, panel) = self.create_webview(&window, window_id, plugin_conf)?;
            let window_state = WindowState::new(self, title, window, webview, panel, winconf)?;
            self.windows.insert(window_state.window.id(), window_state);
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
