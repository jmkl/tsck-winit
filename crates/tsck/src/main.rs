mod app;
mod config;
mod event;
mod hotkee;
mod ipc;
mod utils;
use crate::app::TsckApp;
use crate::config::WindowConf;
use crate::event::ChannelEvent;
use crate::hotkee::spawn_hotkee;
use crate::utils::winview_util::webview_bounds;
use flume::unbounded;
use parking_lot::Mutex;
use std::sync::Arc;
use winit::event_loop::EventLoop;
use winit::window::Window;
use wry::WebView;
use wry::dpi::PhysicalSize;

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

fn main() -> anyhow::Result<()> {
    // let path =
    //     PathBuf::from("C:\\Webview2\\Microsoft.WebView2.FixedVersionRuntime.142.0.3595.94.x64\\");
    // unsafe {
    //     std::env::set_var(
    //         "WEBVIEW2_BROWSER_EXECUTABLE_FOLDER",
    //         path.to_string_lossy().to_string(),
    //     )
    // };
    let event_loop = EventLoop::new()?;
    event_loop.listen_device_events(winit::event_loop::DeviceEvents::Never);
    let (tx, rx) = unbounded::<ChannelEvent>();
    let proxy = event_loop.create_proxy();
    spawn_hotkee(tx.clone(), proxy);
    event_loop.run_app(TsckApp::new(tx, rx))?;
    Ok(())
}
