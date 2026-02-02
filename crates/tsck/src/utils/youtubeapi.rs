#![allow(unused)]
use crate::{event::TS_PATH, ts_struct};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

const API_URL: &str = "https://www.googleapis.com/youtube/v3";
#[derive(Debug, Deserialize, Serialize)]
pub struct YouTubeResponse {
    pub kind: String,
    pub etag: String,
    pub items: Vec<VideoItem>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoItem {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub snippet: Snippet,
    pub statistics: Statistics,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Snippet {
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
    #[serde(rename = "channelTitle")]
    pub channel_title: String,
    #[serde(rename = "categoryId")]
    pub category_id: String,
    #[serde(rename = "liveBroadcastContent")]
    pub live_broadcast_content: String,
    #[serde(rename = "defaultLanguage")]
    pub default_language: Option<String>,
    pub localized: Localized,
    #[serde(rename = "defaultAudioLanguage")]
    pub default_audio_language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Thumbnails {
    pub default: Option<Thumbnail>,
    pub medium: Option<Thumbnail>,
    pub high: Option<Thumbnail>,
    pub standard: Option<Thumbnail>,
    pub maxres: Option<Thumbnail>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

impl Thumbnails {
    /// Get the highest quality thumbnail available
    pub fn get_highest_quality(&self) -> Option<&Thumbnail> {
        self.maxres
            .as_ref()
            .or(self.standard.as_ref())
            .or(self.high.as_ref())
            .or(self.medium.as_ref())
            .or(self.default.as_ref())
    }

    /// Get all thumbnails sorted by size (largest first)
    pub fn get_all_sorted(&self) -> Vec<&Thumbnail> {
        let mut thumbs = vec![];

        if let Some(t) = &self.maxres {
            thumbs.push(t);
        }
        if let Some(t) = &self.standard {
            thumbs.push(t);
        }
        if let Some(t) = &self.high {
            thumbs.push(t);
        }
        if let Some(t) = &self.medium {
            thumbs.push(t);
        }
        if let Some(t) = &self.default {
            thumbs.push(t);
        }

        thumbs
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Localized {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Statistics {
    #[serde(rename = "viewCount")]
    pub view_count: String,
    #[serde(rename = "likeCount")]
    pub like_count: String,
    #[serde(rename = "favoriteCount")]
    pub favorite_count: String,
    #[serde(rename = "commentCount")]
    pub comment_count: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageInfo {
    #[serde(rename = "totalResults")]
    pub total_results: u32,
    #[serde(rename = "resultsPerPage")]
    pub results_per_page: u32,
}
ts_struct! {
    path = TS_PATH,
pub struct YoutubeResponse {
    pub title: String,
    pub thumbnail_url: String,
}
}
pub struct YoutubeApi {
    api_key: Option<String>,
}
pub fn get_youtube_id(url: &str) -> Option<String> {
    let url = url.trim();

    // Handle youtu.be short links
    if url.contains("youtu.be/") {
        return url
            .split("youtu.be/")
            .nth(1)?
            .split(&['?', '&', '#'][..])
            .next()
            .map(|s| s.to_string());
    }

    // Handle youtube.com links with v= parameter
    if url.contains("youtube.com") && url.contains("v=") {
        return url
            .split("v=")
            .nth(1)?
            .split(&['&', '#'][..])
            .next()
            .map(|s| s.to_string());
    }

    // Handle youtube.com/embed/ links
    if url.contains("youtube.com/embed/") {
        return url
            .split("embed/")
            .nth(1)?
            .split(&['?', '&', '#'][..])
            .next()
            .map(|s| s.to_string());
    }

    // If it's just the ID (11 characters)
    if url.len() == 11
        && url
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Some(url.to_string());
    }

    None
}

#[derive(Debug)]
pub enum YTError {
    Request(String),
    Json(String),
    Else(String),
}
impl std::fmt::Display for YTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YTError::Request(error) => write!(f, "Error request: {}", error),
            YTError::Json(error) => write!(f, "Error parsing: {}", error),
            YTError::Else(error) => write!(f, "Error Else {}", error),
        }
    }
}
impl std::error::Error for YTError {}
impl From<ureq::Error> for YTError {
    fn from(err: ureq::Error) -> Self {
        YTError::Request(err.to_string())
    }
}
impl From<serde_json::Error> for YTError {
    fn from(err: serde_json::Error) -> Self {
        YTError::Json(err.to_string())
    }
}
impl YoutubeApi {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("YOUTUBE_API_KEY").ok(),
        }
    }
    pub fn with_api(api_key: String) -> Self {
        Self {
            api_key: Some(api_key),
        }
    }

    pub fn fetch(&self, url: &str) -> Result<YoutubeResponse, YTError> {
        let video_id = get_youtube_id(url).ok_or_else(|| YTError::Else("Invalid URL".into()))?;

        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| YTError::Else("YOUTUBE_API_KEY not found".into()))?;

        let api_url = format!(
            "{}/videos?part=snippet,statistics&id={}&key={}",
            API_URL, video_id, api_key
        );

        println!("{}", &api_url);
        let response_text = ureq::get(&api_url).call()?.body_mut().read_to_string()?;
        let response: YouTubeResponse = serde_json::from_str(&response_text)?;
        let video = response
            .items
            .first()
            .ok_or_else(|| YTError::Else("Video undefined. Incorrect video ID".into()))?;

        let thumbnail = video
            .snippet
            .thumbnails
            .get_highest_quality()
            .ok_or_else(|| YTError::Else("Thumbnail not found".into()))?;

        Ok(YoutubeResponse {
            title: video.snippet.title.clone(),
            thumbnail_url: thumbnail.url.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_yt_api() {
        let yt = YoutubeApi::new();
        let response = yt.fetch("-sePewgzVfU");
        println!("{:?}", response);
    }
}
