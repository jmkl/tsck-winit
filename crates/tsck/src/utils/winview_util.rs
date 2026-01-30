use crate::store::config::{ToolbarPanel, ToolbarPosition};
use winit::dpi::{LogicalPosition, LogicalSize};
use wry::Rect;

pub fn webview_bounds(which: &str, size: LogicalSize<u32>, toolbar_panel: &ToolbarPanel) -> Rect {
    let w = size.width;
    let h = size.height;

    let (panel_width, padding) = calculate_panel_dimensions(w, toolbar_panel);
    let panel_height = toolbar_panel.height;
    let margin = if toolbar_panel.absolute {
        0
    } else {
        panel_height
    };

    let (toolbar_pos, webview_rect) = calculate_positions(
        w,
        h,
        panel_width,
        panel_height,
        padding,
        margin,
        &toolbar_panel.toolbar_position,
    );

    match which {
        "panel" => Rect {
            position: toolbar_pos,
            size: LogicalSize::new(panel_width, panel_height).into(),
        },
        _ => webview_rect,
    }
}

fn calculate_panel_dimensions(window_width: u32, toolbar_panel: &ToolbarPanel) -> (u32, u32) {
    match toolbar_panel.max_width {
        Some(max_width) => {
            let width = window_width.min(max_width * 2) / 2;
            (width, toolbar_panel.padding)
        }
        None => (window_width, 0),
    }
}

fn calculate_positions(
    w: u32,
    h: u32,
    panel_width: u32,
    panel_height: u32,
    padding: u32,
    margin: u32,
    position: &ToolbarPosition,
) -> (wry::dpi::Position, Rect) {
    use ToolbarPosition::*;

    let webview_rect = |y_offset: u32, height: u32| Rect {
        position: LogicalPosition::new(0, y_offset).into(),
        size: LogicalSize::new(w, height).into(),
    };

    match position {
        TopLeft => (
            LogicalPosition::new(padding, padding).into(),
            webview_rect(margin, h - margin),
        ),
        TopRight => (
            LogicalPosition::new(w - panel_width - padding, padding).into(),
            webview_rect(margin, h - margin),
        ),
        BottomLeft => (
            LogicalPosition::new(padding, h - panel_height - padding).into(),
            webview_rect(0, h - margin),
        ),
        BottomRight => (
            LogicalPosition::new(w - panel_width - padding, h - panel_height - padding).into(),
            webview_rect(0, h - margin),
        ),
    }
}

#[cfg(test)]
mod test_window {
    #[test]
    fn test_calc() {
        let h: i32 = (-8) / 1440;
        println!("{}", h.abs());
    }
}
