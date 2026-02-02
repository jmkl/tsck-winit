pub(crate) mod animation;
pub(crate) mod app_tokenizer;
pub mod download;
pub mod logger;
pub(crate) mod winview_util;
pub mod youtubeapi;

pub fn url_encode(input: &str) -> String {
    let mut out = String::new();

    for b in input.bytes() {
        match b {
            // Allowed characters (RFC 3986)
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }

            // Everything else → percent-encode
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }

    out
}
