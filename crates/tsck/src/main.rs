mod app;
mod app_config;
mod event;
mod hotkee;
mod io;
mod ipc;
mod macros;
mod photoshop;
mod store;
mod utils;
use crate::app::TsckApp;
use crate::app_config::AppConfigHandler;
use crate::event::{ChannelEvent, UserEvent};
use crate::hotkee::init_hotkee;
use crate::io::{HttpServer, Response};
use crate::photoshop::SmartObjectItem;
use crate::store::config::WindowConf;
use crate::utils::winview_util::webview_bounds;
use flume::{Receiver, Sender, unbounded};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use winit::event_loop::{EventLoop, EventLoopProxy};
use winit::window::Window;
use wry::WebView;
use wry::dpi::PhysicalSize;
const DOTFILE_DIR: &'static str = "tsck-winit";

struct WindowState {
    title: String,
    window: Arc<dyn Window>,
    webview: Arc<WebView>,
    panel: Arc<Option<WebView>>,
    window_conf: Arc<WindowConf>,
    on_top: Arc<Mutex<bool>>,
}

#[allow(unused)]
impl WindowState {
    fn new(
        event_loop: &TsckApp,
        title: String,
        window: Box<dyn Window>,
        webview: WebView,
        panel: Option<WebView>,
        window_conf: Arc<WindowConf>,
    ) -> anyhow::Result<Self> {
        let window = Arc::from(window);
        let webview = Arc::from(webview);
        let panel = Arc::from(panel);
        let state = Self {
            title,
            window,
            webview,
            panel,
            window_conf,
            on_top: Arc::new(Mutex::new(false)),
        };
        Ok(state)
    }
    fn resize(&mut self, size: PhysicalSize<u32>) {
        let size = size.to_logical::<u32>(self.window.scale_factor());
        if size.width <= 100 || size.height <= 100 {
            return;
        }
        _ = self.webview.set_bounds(webview_bounds(
            "view",
            size,
            &self.window_conf.toolbar_panel,
        ));

        if let Some(panel) = self.panel.as_ref() {
            _ = panel.set_bounds(webview_bounds(
                "panel",
                size,
                &self.window_conf.toolbar_panel,
            ));
        };
    }
}

type WebsocketMessagePayload = (Option<u64>, String);

pub struct ChannelBus {
    config_handler: AppConfigHandler,
    sender: Sender<ChannelEvent>,
    receiver: Receiver<ChannelEvent>,
    proxy: Arc<EventLoopProxy>,
    clients: Arc<Mutex<HashMap<u64, io::ws::Client>>>,
    websocket_bus: (
        Sender<WebsocketMessagePayload>,
        Receiver<WebsocketMessagePayload>,
    ),
}
impl ChannelBus {
    fn new(proxy: EventLoopProxy) -> Self {
        let (tx, rx) = unbounded::<ChannelEvent>();
        Self {
            config_handler: AppConfigHandler::new(),
            sender: tx,
            receiver: rx,
            proxy: Arc::new(proxy),
            clients: Arc::new(Mutex::new(HashMap::new())),
            websocket_bus: unbounded::<WebsocketMessagePayload>(),
        }
    }
    pub fn send(&self, event: ChannelEvent) {
        if let Err(err) = self.sender.send(event) {
            log_error!("Error sending ", err);
        }
    }
    pub fn wake_up(&self) {
        self.proxy.wake_up();
    }
    pub fn get_config(&self) -> &AppConfigHandler {
        &self.config_handler
    }
    pub fn get_receiver(&self) -> Receiver<ChannelEvent> {
        self.receiver.clone()
    }
    //
    fn bind_websocket(self) -> Self {
        _ = self.spawn_ws_server();
        self
    }

    pub fn ws_send_to(&self, id: u64, message: String) {
        _ = self.websocket_bus.0.send((Some(id), message));
    }
    pub fn ws_send_to_all(&self, message: String) {
        _ = self.websocket_bus.0.send((None, message));
    }
    //textures API
    fn texture_get_all_categories(&self) {
        todo!("unimplemented")
    }
    fn texture_get_all_textures(&self) {
        todo!("unimplemented")
    }
    fn texture_get_texture_chunk(&self, page: usize, limit: usize) {
        todo!("unimplemented")
    }
    fn texture_get_favorite_chunk(&self, page: usize, limit: usize) {
        todo!("unimplemented")
    }
    fn texture_update_favorite(&self, id: i32, favorite: bool) {
        todo!("unimplemented")
    }
    fn texture_get_texture_chunk_by_category(&self, page: usize, limit: usize) {
        todo!("unimplemented")
    }

    //smartobject API
    fn smartobject_add_file(&self, item: SmartObjectItem) {
        todo!("unimplemented")
    }
    fn smartobject_filter_chunk(&self, filter: &str, page: usize, per_page: usize) {
        todo!("unimplemented")
    }
    fn smartobject_delete(&self, item: SmartObjectItem) {
        todo!("unimplemented")
    }
    fn smartobject_create_thumb(&self, png_name: &str) {
        todo!("unimplemented")
    }

    fn spawn_ws_server(&self) -> anyhow::Result<()> {
        let ws_server = io::ws::listen(self.get_config().websocket_server_port())?;
        let bus_sender = self.sender.clone();
        let ws_bus_receiver = self.websocket_bus.1.clone();
        let clients = self.clients.clone();
        std::thread::spawn(move || -> anyhow::Result<()> {
            while let Ok(recv) = ws_bus_receiver.recv() {
                let clients = {
                    let guard = clients.lock();
                    guard
                };

                match recv.0 {
                    Some(found_id) => {
                        if let Some((_, c)) = clients.iter().find(|(id, _)| *id == &found_id) {
                            c.send(&recv.1);
                        }
                    }
                    None => {
                        clients.iter().for_each(|(_, c)| {
                            c.send(&recv.1);
                        });
                    }
                }
            }
            Ok(())
        });
        let clients = self.clients.clone();
        let proxy = self.proxy.clone();
        std::thread::spawn(move || -> anyhow::Result<()> {
            loop {
                match ws_server.recv()? {
                    io::ws::Event::Connected(id, client) => {
                        log_debug!("Client added:", &id);
                        clients.lock().insert(id, client);
                    }
                    io::ws::Event::Disconnected(id) => {
                        log_debug!("Client remove:", &id);
                        clients.lock().remove(&id);
                    }
                    io::ws::Event::Message(id, message) => {
                        _ = bus_sender.send((
                            UserEvent::IncomingWebsocketMessage(id, message),
                            None,
                            None,
                        ));
                        proxy.wake_up();
                    }
                }
            }
        });

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.listen_device_events(winit::event_loop::DeviceEvents::Never);
    let bus = Arc::new(ChannelBus::new(event_loop.create_proxy()).bind_websocket());
    init_hotkee(bus.clone());
    init_file_server(bus.clone())?;
    event_loop.run_app(TsckApp::new(bus))?;

    Ok(())
}

struct AppState;

fn init_file_server(channel_bus: Arc<ChannelBus>) -> anyhow::Result<()> {
    let root_path = channel_bus.get_config().store_root();
    let (smartobject, texture) = {
        let smartobject = Path::new(&root_path)
            .join("smartobject")
            .join("thumbs")
            .to_string_lossy()
            .to_string();
        let texture = Path::new(&root_path)
            .join("texture")
            .join(".thumbnail")
            .to_string_lossy()
            .to_string();
        (smartobject, texture)
    };
    std::thread::spawn(move || -> anyhow::Result<()> {
        HttpServer::new(channel_bus.get_config().http_server_port(), AppState {})
            .cors(true)
            .static_files(smartobject, "/smartobject")
            .static_files(texture, "/texture")
            .on_request(|req, _| match (req.method, req.path.as_str()) {
                (io::Method::GET, "/") => Response::json("[\"404 Not Fuck\"]"),
                _ => Response::not_found(),
            })
            .listen()
    });
    Ok(())
}
