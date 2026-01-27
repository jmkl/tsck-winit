use flume::{Receiver, Sender};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::platform::windows::WindowExtWindows;
use winit::window::{Window, WindowAttributes, WindowId};
use wry::dpi::{LogicalSize, Position, Size};
use wry::http::Request;
use wry::{WebView, WebViewBuilder};

use crate::config::{ConfigParser, PluginConf, WindowSrc};
use crate::event::{ChannelEvent, UserEvent};
use crate::ipc::IpcRequest;
use crate::utils::animation::map_value;
use crate::utils::winview_util::webview_bounds;
use crate::{WindowState, dp, log_debug, log_error};

pub struct TsckApp {
    windows: HashMap<WindowId, WindowState>,
    receiver: Receiver<ChannelEvent>,
    sender: Sender<ChannelEvent>,
    dev_url: String,
    plugin_config: Arc<HashMap<String, PluginConf>>,
}

macro_rules! get_window {
    ($self:expr, $wid:ident,|$window:ident|$space:block) => {
        if let Some(window_id) = $wid {
            if let Some(ws) = $self.windows.get(&window_id) {
                let $window = ws;
                $space
            }
        }
    };
}

impl TsckApp {
    pub fn new(sender: Sender<ChannelEvent>, receiver: Receiver<ChannelEvent>) -> Self {
        let config = ConfigParser::parse(include_str!("../tsck.json"));
        // _ = sender.send((UserEvent::CreateWindow("main".to_string()), None));
        Self::init(&sender, &config);
        Self {
            windows: HashMap::new(),
            receiver,
            sender,
            dev_url: config.dev_url,
            plugin_config: Arc::new(config.plugins),
        }
    }
    fn init(sender: &Sender<ChannelEvent>, config: &ConfigParser) {
        for (n, p) in config.plugins.iter() {
            if p.with_window {
                if let Some(window) = p.window.as_ref() {
                    if window.auto_launch {
                        _ = sender.send((UserEvent::LaunchPlugin(n.clone()), None));
                    }
                }
            }
        }
    }

    fn process_cmd(&mut self, event_loop: &dyn ActiveEventLoop) {
        let receiver = self.receiver.clone();
        let config = self.plugin_config.clone();
        while let Ok((cmd, window_id)) = receiver.try_recv() {
            match cmd {
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
                    println!("UserEvent::DragWindow");
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
                UserEvent::SetWindowOnTop(_) => {
                    println!("UserEvent::SetWindowOnTop");
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
                UserEvent::GoogleDownloadImage(url) => {
                    log_error!("Google Download Image", url);
                }
            }
        }
    }
    fn ipc_handler(&self, window_id: WindowId) -> impl Fn(Request<String>) + 'static {
        let sender = self.sender.clone();
        move |req| {
            let body = req.body();
            if let Ok(response) = serde_json::from_str::<IpcRequest>(body) {
                if let Some(data) = response.data {
                    if let Ok(ev) = serde_json::from_value::<UserEvent>(data) {
                        _ = sender.send((ev, Some(window_id)));
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

                let sender = self.sender.clone();
                let view = builder
                    .with_url(url)
                    .with_new_window_req_handler(move |url, _| {
                        _ = sender.send((UserEvent::NavigateWebview(url), Some(window_id)));
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
            let attrs = WindowAttributes::default()
                .with_title(&title)
                .with_transparent(window_conf.transparent)
                .with_decorations(window_conf.decorations)
                .with_surface_size(LogicalSize::new(0, 0))
                // .with_surface_size(window_conf.window_size.to_logical_size())
                .with_window_level(window_level);

            let window = event_loop.create_window(attrs)?;
            // window.set_system_backdrop(winit::platform::windows::BackdropType::TransientWindow);
            window.set_corner_preference(winit::platform::windows::CornerPreference::Round);
            // FIXME strangely, wry create white background that stay onload
            // workaraound is to set the window size to zero and scale back in
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
