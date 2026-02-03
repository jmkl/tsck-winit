use anyhow::Result;
use rust_embed_for_web::EmbedableFile;
use std::{borrow::Cow, os::windows::process::CommandExt, sync::Arc};
use wry::WebViewBuilder;
use wry::{
    NewWindowFeatures, NewWindowResponse,
    http::{Request, Response},
};

use crate::EmbeddedAssets;

pub(crate) fn setup_custom_protocol<'a>(
    builder: WebViewBuilder<'a>,
    path: &str,
) -> WebViewBuilder<'a> {
    let protocol_name = "tsck";

    builder
        .with_custom_protocol(protocol_name.to_string(), move |_webview_id, request| {
            match handle_protocol_request(request) {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("Protocol error: {}", e);
                    error_response()
                }
            }
        })
        .with_new_window_req_handler(handle_new_window_request)
        .with_url(format!("{}://localhost{}", protocol_name, path))
}
fn handle_protocol_request(request: Request<Vec<u8>>) -> Result<Response<Cow<'static, [u8]>>> {
    let path = request.uri().path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };
    if let Some(content) = EmbeddedAssets::get(path).map(|f| f.data().to_vec()) {
        return create_response(path, content);
    }

    if !path.contains('.') {
        if let Some(content) = EmbeddedAssets::get("index.html").map(|f| f.data().to_vec()) {
            return create_response("index.html", content);
        }
    }

    // Not found
    Ok(Response::builder()
        .status(404)
        .header("Content-Type", "text/plain")
        .body(Cow::Borrowed(b"404 Not Found" as &[u8]))?)
}

fn create_response(path: &str, content: Vec<u8>) -> Result<Response<Cow<'static, [u8]>>> {
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

fn error_response() -> Response<Cow<'static, [u8]>> {
    Response::builder()
        .status(500)
        .header("Content-Type", "text/plain")
        .body(Cow::Owned(b"Internal server error".to_vec()))
        .unwrap()
}
pub fn handle_new_window_request(url: String, _: NewWindowFeatures) -> NewWindowResponse {
    if let Err(e) = open(&url) {
        eprintln!("Failed to open URL: {}", e);
    }
    NewWindowResponse::Deny
}
fn open(url: &str) -> Result<(), std::io::Error> {
    std::process::Command::new("cmd")
        .args(["/C", "start", url])
        .creation_flags(0x08000000)
        .spawn()?;
    Ok(())
}
