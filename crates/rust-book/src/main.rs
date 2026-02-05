#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rust_embed_for_web::{EmbedableFile, RustEmbed};
use std::borrow::Cow;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};
use wry::{
    NewWindowFeatures, NewWindowResponse, Rect, WebView, WebViewBuilder,
    dpi::{LogicalPosition, LogicalSize, PhysicalSize},
    http::{Request, Response},
};
#[derive(RustEmbed)]
#[folder = "book/book"]
struct EmbedAssets;

#[derive(Default)]
struct RustBook {
    window: Option<Box<dyn Window>>,
    webview: Option<WebView>,
}
impl RustBook {
    fn resize(&mut self, size: PhysicalSize<u32>) {
        if let Some(window) = self.window.as_ref() {
            let size = size.to_logical::<u32>(window.scale_factor());
            if let Some(wv) = self.webview.as_ref() {
                _ = wv.set_bounds(Rect {
                    position: LogicalPosition::new(0, 0).into(),
                    size: LogicalSize::new(size.width, size.height).into(),
                });
            }
        }
    }
    fn create_webview(&mut self, window: &Box<dyn Window>) -> anyhow::Result<()> {
        let protocol_name = "book";
        let webview = WebViewBuilder::new()
            .with_devtools(true)
            .with_custom_protocol(protocol_name.to_string(), move |_wv_id, request| {
                match RustBook::handle_protocol_request(request) {
                    Ok(response) => response,
                    Err(e) => {
                        eprintln!("Protocol error: {}", e);
                        RustBook::error_response()
                    }
                }
            })
            // .with_new_window_req_handler(Self::handle_new_window_request)
            .with_url(format!("{}://localhost/", protocol_name));
        let wv = webview.build(window)?;
        self.webview = Some(wv);
        Ok(())
    }
    fn handle_protocol_request(
        request: Request<Vec<u8>>,
    ) -> anyhow::Result<Response<Cow<'static, [u8]>>> {
        let path = request.uri().path().trim_start_matches('/');
        let path = if path.is_empty() { "index.html" } else { path };
        if let Some(content) = EmbedAssets::get(path).map(|f| f.data().to_vec()) {
            return RustBook::create_response(path, content);
        }

        if !path.contains('.') {
            if let Some(content) = EmbedAssets::get("index.html").map(|f| f.data().to_vec()) {
                return RustBook::create_response("index.html", content);
            }
        }

        Ok(Response::builder()
            .status(404)
            .header("Content-Type", "text/plain")
            .body(Cow::Borrowed(b"404 Not Found" as &[u8]))?)
    }
    fn error_response() -> Response<Cow<'static, [u8]>> {
        Response::builder()
            .status(500)
            .header("Content-Type", "text/plain")
            .body(Cow::Owned(b"Internal server error".to_vec()))
            .unwrap()
    }
    fn create_response(
        path: &str,
        content: Vec<u8>,
    ) -> anyhow::Result<Response<Cow<'static, [u8]>>> {
        let mime = mime_guess::from_path(path)
            .first_or_octet_stream()
            .as_ref()
            .to_string();

        Ok(Response::builder()
            .status(200)
            .header("Content-Type", mime)
            .header("Access-Control-Allow-Origin", "*")
            .body(Cow::Owned(content))?)
    }
    pub fn handle_new_window_request(url: String, _: NewWindowFeatures) -> NewWindowResponse {
        // Command::new("pwsh").args(&["-NoLogo", "-NoProfile", "-C", "start", "", &url]).spawn();
        NewWindowResponse::Allow
        // if let Err(e) = open(&url) {
        //     eprintln!("Failed to open URL: {}", e);
        // }
        // NewWindowResponse::Deny
    }
}
impl ApplicationHandler for RustBook {
    fn can_create_surfaces(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop) {
        let attrs = WindowAttributes::default();
        self.window = match event_loop.create_window(attrs) {
            Ok(window) => {
                if let Err(err) = self.create_webview(&window) {
                    println!("{err}");
                }
                Some(window)
            }
            Err(_) => {
                event_loop.exit();
                return;
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &dyn winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::SurfaceResized(size) => {
                self.resize(size);
            }
            _ => {}
        }
    }
}

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.run_app(RustBook::default())?;
    Ok(())
}
