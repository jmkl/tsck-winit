use crate::event::UserEvent;
use crate::photoshop::ps_misc::Bounds;
use std::fs::File;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::copy,
    path::Path,
    thread::JoinHandle,
};
pub fn dl_image(download_url: &str, comfyui_root: &str) -> Option<UserEvent> {
    let path = Path::new(comfyui_root).join("output");
    if path.exists() {
        if let Some(image) = download_image(download_url, &path.to_string_lossy().to_string()) {
            let payload = UserEvent::AppendComfyUIOutput {
                images: vec![image.clone()],
                bounds: Bounds::default(),
            };

            return Some(payload);
        }
    }
    None
}

fn download_image(url: &str, folder: &str) -> Option<String> {
    download_image_threaded(url, folder)
        .join()
        .ok()
        .and_then(|res| res.ok())
}

fn download_image_threaded(url: &str, folder: &str) -> JoinHandle<anyhow::Result<String>> {
    let url = url.to_string();
    let folder = folder.to_string();
    std::thread::spawn(move || _download_image(&url, &folder))
}

fn _download_image(url: &str, folder: &str) -> anyhow::Result<String> {
    let response = ureq::get(url).call()?;
    let (part, body) = response.into_parts();
    let ext = match part.headers.get("content-type") {
        Some(mime) => {
            match mime.to_str().unwrap() {
                "image/jpeg" => ".jpg",
                "image/png" => ".png",
                "image/gif" => ".gif",
                "image/bmp" => ".bmp",
                "image/webp" => ".webp",
                "image/tiff" => ".tiff",
                "image/svg+xml" => ".svg",
                "image/x-icon" => ".ico",
                "image/heif" => ".heif",
                "image/heic" => ".heic",
                "image/avif" => ".avif",
                _ => ".jpg", // fallback
            }
        }
        None => ".jpg",
    };
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let filename = format!("{}{}", hasher.finish(), ext);
    let output = Path::new(folder).join(&filename);
    if output.exists() {
        println!("File exist.. no need to download");
        return Ok(filename);
    };
    let output = output.to_string_lossy().to_string();
    println!("output : {:?}", output);
    let mut file = File::create(&output)?;
    let mut reader = body.into_reader();
    copy(&mut reader, &mut file)?;
    Ok(filename)
}
