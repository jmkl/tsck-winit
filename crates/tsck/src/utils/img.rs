use image;
use winit::icon::{Icon, RgbaIcon};
pub fn load_icon(icon_bytes: Vec<u8>) -> Option<Icon> {
    if let Ok(image) = image::load_from_memory(&icon_bytes) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();

        if let Ok(ic) = RgbaIcon::new(rgba, width, height) {
            return Some(ic.into());
        }
    }
    None
}
