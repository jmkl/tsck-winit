#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::dpi::{Position, Size};
use wry::dpi::{PhysicalPosition, PhysicalSize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WindowSrc {
    Local(String),
    Web(String, String),
}
impl WindowSrc {
    pub fn from_url(url: impl Into<String>, page: impl Into<String>) -> Self {
        WindowSrc::Web(url.into(), page.into())
    }
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WindowSize {
    pub width: i32,
    pub height: i32,
}
impl WindowSize {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
    pub fn to_logical_size(&self) -> LogicalSize<i32> {
        LogicalSize {
            width: self.width,
            height: self.height,
        }
    }
    pub fn to_size(&self) -> Size {
        Size::Physical(PhysicalSize {
            width: self.width as u32,
            height: self.height as u32,
        })
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowPosition {
    pub x: f32,
    pub y: f32,
}
impl WindowPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn to_logical_position(&self) -> LogicalPosition<f32> {
        LogicalPosition {
            x: self.x,
            y: self.y,
        }
    }
    pub fn to_position(&self) -> Position {
        Position::Physical(PhysicalPosition {
            x: self.x as i32,
            y: self.y as i32,
        })
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolbarPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ToolbarPanel {
    pub max_width: Option<u32>,
    pub height: u32,
    pub padding: u32,
    pub absolute: bool,
    pub toolbar_position: ToolbarPosition,
}
impl Default for ToolbarPanel {
    fn default() -> Self {
        Self {
            max_width: None,
            height: 32,
            absolute: true,
            toolbar_position: ToolbarPosition::TopLeft,
            padding: 0,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct WindowConf {
    pub window_src: WindowSrc,
    pub decorations: bool,
    pub shadow: bool,
    pub always_on_top: bool,
    pub ignore_cursor_event: bool,
    pub transparent: bool,
    pub skip_taskbar: bool,
    pub auto_launch: bool,
    pub window_size: WindowSize,
    pub window_position: WindowPosition,
    pub webview_zoom_factor: f64,
    pub toolbar_panel: ToolbarPanel,
}
impl Default for WindowConf {
    fn default() -> Self {
        let toolbar = ToolbarPanel::default();
        Self {
            window_src: WindowSrc::Local("/".into()),
            decorations: true,
            shadow: true,
            always_on_top: false,
            ignore_cursor_event: false,
            transparent: true,
            skip_taskbar: false,
            auto_launch: true,
            window_size: WindowSize {
                width: 300,
                height: 200,
            },
            webview_zoom_factor: 1.0,
            toolbar_panel: toolbar,
            window_position: WindowPosition { x: 0.0, y: 0.0 },
        }
    }
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct PluginConf {
    pub with_window: bool,
    pub custom_script: Option<String>,
    pub device_event_filter: bool,
    pub window: Option<WindowConf>,
}
impl Default for PluginConf {
    fn default() -> Self {
        Self {
            with_window: false,
            custom_script: None,
            device_event_filter: false,
            window: None,
        }
    }
}
impl PluginConf {
    pub fn get_initial_script(&self) -> String {
        let result = match self.custom_script.as_ref() {
            Some(script) => script.clone(),
            None => String::from("{}"),
        };
        result
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigParser {
    pub dev_url: String,
    pub plugins: HashMap<String, PluginConf>,
}
impl ConfigParser {
    pub fn parse(config: &str) -> Self {
        serde_json::from_str::<ConfigParser>(config).expect("Failed to parse tsck.json")
    }
}

#[cfg(test)]
mod test {
    use crate::{config::ConfigParser, dp, log_error};

    #[test]
    fn test() {
        let config = ConfigParser::parse(include_str!("../tsck.json"));
        log_error!(dp!(config));
    }
}
