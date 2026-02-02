mod app;
mod app_config;
mod cmd;
mod event;
mod hotkee;
mod io;
mod ipc;
mod macros;
mod photoshop;
mod store;
mod utils;
use crate::app::TsckApp;
use crate::app_config::{AppConfig, AppConfigHandler};
use crate::cmd::{CmdrHelper, CommandConfig};
use crate::event::{ChannelEvent, UserEvent, WinLevel};
use crate::hotkee::init_hotkee;
use crate::io::{HttpServer, Response};
use crate::photoshop::{PaginationItems, SmartObjectItem, SmartObjects, TextureRepo};
use crate::store::config::WindowConf;
use crate::store::{DbStore, PageChunk, Texture};
use crate::utils::winview_util::webview_bounds;
use flume::{Receiver, Sender, unbounded};
use parking_lot::Mutex;
use serde::Serialize;
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
    win_level: Arc<Mutex<WinLevel>>,
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
            win_level: Arc::new(Mutex::new(WinLevel::Normal)),
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
    cmd_helper: CmdrHelper,
    config_handler: Arc<Mutex<AppConfigHandler>>,
    sender: Sender<ChannelEvent>,
    receiver: Receiver<ChannelEvent>,
    proxy: Arc<EventLoopProxy>,
    smartobject: Arc<Mutex<SmartObjects>>,
    textures: Arc<Mutex<TextureRepo>>,
    clients: Arc<Mutex<HashMap<u64, io::ws::Client>>>,
    websocket_bus: (
        Sender<WebsocketMessagePayload>,
        Receiver<WebsocketMessagePayload>,
    ),
}
impl ChannelBus {
    fn new(proxy: EventLoopProxy) -> anyhow::Result<Self> {
        let (tx, rx) = unbounded::<ChannelEvent>();
        let db = DbStore::new()?;
        let proxy = Arc::new(proxy);
        let config_handler = Arc::new(Mutex::new(AppConfigHandler::new()));
        Ok(Self {
            cmd_helper: CmdrHelper::new(config_handler.clone(), proxy.clone()),
            smartobject: Arc::new(Mutex::new(SmartObjects::new())),
            textures: Arc::new(Mutex::new(TextureRepo::new(db))),
            config_handler,
            sender: tx,
            receiver: rx,
            proxy: proxy,
            clients: Arc::new(Mutex::new(HashMap::new())),
            websocket_bus: unbounded::<WebsocketMessagePayload>(),
        })
    }
    pub fn send(&self, event: ChannelEvent) {
        if let Err(err) = self.sender.send(event) {
            log_error!("Error sending ", err);
        }
    }
    pub fn wake_up(&self) {
        self.proxy.wake_up();
    }
    pub fn update_app_config(&self, config: AppConfig) {
        self.config_handler.lock().update_config(config);
    }
    pub fn get_app_config(&self) -> AppConfig {
        let guard = {
            let g = self.config_handler.lock();
            g.config_store.get(|c| c.clone())
        };
        guard
    }
    pub fn get_config(&self) -> Arc<Mutex<AppConfigHandler>> {
        self.config_handler.clone()
    }
    pub fn get_receiver(&self) -> Receiver<ChannelEvent> {
        self.receiver.clone()
    }
    //websocket
    fn bind_websocket(self) -> Self {
        _ = self.spawn_ws_server();
        self
    }
    pub fn broadcast_to_websocket<T: Serialize>(&self, msg: T) {
        let guard = self.clients.lock();
        for (_, client) in guard.iter() {
            if let Ok(message) = serde_json::to_string(&msg) {
                client.send(message);
            }
        }
    }

    pub fn ws_send_to(&self, id: u64, message: String) {
        _ = self.websocket_bus.0.send((Some(id), message));
    }
    pub fn ws_send_to_all(&self, message: String) {
        _ = self.websocket_bus.0.send((None, message));
    }
    //textures API
    fn texture_get_all_categories(&self) -> Option<Vec<String>> {
        self.textures.lock().get_all_categories().ok()
    }

    fn texture_get_texture_chunk(
        &self,
        category: String,
        page: usize,
        limit: usize,
    ) -> Option<PageChunk<Texture>> {
        log_debug!(&category, page, limit);
        match category.as_str() {
            "Favorite" => self.textures.lock().get_favorite_chunk(page, limit).ok(),
            _ => self
                .textures
                .lock()
                .get_textures_chunk_by_category(category, page, limit)
                .ok(),
        }
    }

    fn texture_update_favorite(&self, id: i32, favorite: bool) -> Option<()> {
        self.textures.lock().set_favorite(id, favorite).ok()
    }

    //smartobject API
    fn smartobject_add_file(&self, item: SmartObjectItem) {
        self.smartobject.lock().files.push(item);
    }
    fn smartobject_filter_chunk(
        &self,
        filter: &str,
        page: usize,
        per_page: usize,
    ) -> PaginationItems {
        let result = self.smartobject.lock().filter_chunk(filter, page, per_page);
        result
    }
    fn smartobject_delete(&self, item: SmartObjectItem) {
        let mut guard = self.smartobject.lock();
        if let Ok(success) = guard.delete_psb(&item.name) {
            if let Some(found) = guard.files.iter().position(|it| it.name == item.name) {
                guard.files.remove(found);
            }
        }
    }
    fn smartobject_create_thumb(&self, png_name: &str) -> Option<SmartObjectItem> {
        let mut guard = self.smartobject.lock();
        let thumb = guard.convert_psd_to_png(png_name)?;
        let name = guard.to_psb(png_name)?;
        let soi = SmartObjectItem { id: 0, name, thumb };
        guard.add_file(&soi);
        Some(soi)
    }

    fn spawn_ws_server(&self) -> anyhow::Result<()> {
        let port = {
            let port = self.get_config().lock().websocket_server_port();
            port
        };
        let ws_server = io::ws::listen(port)?;
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

    pub fn cmd_request_command(&self) -> CommandConfig {
        self.cmd_helper.check_pids();
        let cmd_config = self.get_config().lock().command_config();
        cmd_config
    }
    pub fn cmd_run_command(&self, app_name: String) {
        _ = self.cmd_helper.run_command(app_name, self.sender.clone());
    }
    pub fn cmd_kill_command(&self, app_name: String) {
        self.cmd_helper.kill_process(app_name, self.sender.clone());
    }
}

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.listen_device_events(winit::event_loop::DeviceEvents::Never);
    let bus = Arc::new(ChannelBus::new(event_loop.create_proxy())?.bind_websocket());
    init_hotkee(bus.clone());
    init_file_server(bus.clone())?;

    event_loop.run_app(TsckApp::new(bus))?;

    Ok(())
}

struct AppState;

fn init_file_server(channel_bus: Arc<ChannelBus>) -> anyhow::Result<()> {
    let root_path = { channel_bus.get_config().lock().store_root() };
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
        let port = {
            let port = channel_bus.get_config().lock().http_server_port();
            port
        };
        HttpServer::new(port, AppState {})
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
