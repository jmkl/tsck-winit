use windows::{
    Win32::{
        Foundation::*,
        Graphics::{
            Direct2D::{Common::*, *},
            Dwm::*,
            Gdi::*,
        },
        System::LibraryLoader::*,
        UI::{Controls::MARGINS, WindowsAndMessaging::*},
    },
    core::*,
};

// Window data stored in GWLP_USERDATA
struct BorderWindowData {
    d2d_factory: ID2D1Factory,
    render_target: Option<ID2D1HwndRenderTarget>,
    brush: Option<ID2D1SolidColorBrush>,
    color: D2D1_COLOR_F,
    thickness: f32,
    corner_radius: f32,
}

pub unsafe fn create_transparent_border(
    target_hwnd: HWND,
    color: u32,
    thickness: f32,
) -> Result<HWND> {
    // Extract RGB components
    let r = ((color >> 16) & 0xFF) as f32 / 255.0;
    let g = ((color >> 8) & 0xFF) as f32 / 255.0;
    let b = (color & 0xFF) as f32 / 255.0;

    let d2d_color = D2D1_COLOR_F { r, g, b, a: 1.0 };

    // Get target window rect with DWM frame
    let mut target_rect = RECT::default();
    (unsafe {
        DwmGetWindowAttribute(
            target_hwnd,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut target_rect as *mut _ as *mut _,
            std::mem::size_of::<RECT>() as u32,
        )
    })?;

    // Expand rect by thickness
    let border = thickness as i32;
    target_rect.left -= border;
    target_rect.top -= border;
    target_rect.right += border;
    target_rect.bottom += border;

    // Create D2D factory

    let d2d_factory: ID2D1Factory =
        unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None) }?;

    // Create window data
    let window_data = Box::new(BorderWindowData {
        d2d_factory: d2d_factory.clone(),
        render_target: None,
        brush: None,
        color: d2d_color,
        thickness,
        corner_radius: 0.0,
    });

    // Register window class
    let class_name = w!("TransparentBorderWindow");
    let hinstance = unsafe { GetModuleHandleW(None) }?;

    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        lpfnWndProc: Some(border_wnd_proc),
        hInstance: HINSTANCE(hinstance.0),
        lpszClassName: class_name,
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW) }?,
        ..Default::default()
    };
    unsafe { RegisterClassExW(&wc) };

    // Create window WITHOUT WS_EX_TRANSPARENT first

    let border_hwnd = unsafe {
        CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
            class_name,
            w!("Border"),
            WS_POPUP | WS_VISIBLE,
            target_rect.left,
            target_rect.top,
            target_rect.right - target_rect.left,
            target_rect.bottom - target_rect.top,
            None,
            None,
            Some(HINSTANCE(hinstance.0)),
            Some(Box::into_raw(window_data) as *const _),
        )
    }?;

    // Enable DWM blur behind for transparency

    let margins = MARGINS {
        cxLeftWidth: -1,
        cxRightWidth: -1,
        cyTopHeight: -1,
        cyBottomHeight: -1,
    };
    (unsafe { DwmExtendFrameIntoClientArea(border_hwnd, &margins) })?;

    // Set layered attributes
    (unsafe { SetLayeredWindowAttributes(border_hwnd, COLORREF(0), 255, LWA_ALPHA) })?;

    // Initialize D2D render target
    let width = (target_rect.right - target_rect.left) as u32;
    let height = (target_rect.bottom - target_rect.top) as u32;

    let ptr = unsafe { GetWindowLongPtrW(border_hwnd, GWLP_USERDATA) };
    if ptr != 0 {
        let data = unsafe { &mut *(ptr as *mut BorderWindowData) };

        let props = D2D1_RENDER_TARGET_PROPERTIES {
            r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: 0.0,
            dpiY: 0.0,
            usage: D2D1_RENDER_TARGET_USAGE_NONE,
            minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
        };

        let hwnd_props = D2D1_HWND_RENDER_TARGET_PROPERTIES {
            hwnd: border_hwnd,
            pixelSize: D2D_SIZE_U { width, height },
            presentOptions: D2D1_PRESENT_OPTIONS_IMMEDIATELY,
        };

        let render_target =
            unsafe { data.d2d_factory.CreateHwndRenderTarget(&props, &hwnd_props) }?;

        let brush = unsafe { render_target.CreateSolidColorBrush(&data.color, None) }?;

        data.render_target = Some(render_target);
        data.brush = Some(brush);
    } else {
        return Err(Error::empty());
    }

    // Force initial paint

    _ = unsafe { InvalidateRect(Some(border_hwnd), None, true) };
    _ = unsafe { UpdateWindow(border_hwnd) };

    // Now make it click-through by setting extended style

    let mut style = unsafe { GetWindowLongW(border_hwnd, GWL_EXSTYLE) } as u32;
    style |= WS_EX_TRANSPARENT.0;
    unsafe { SetWindowLongW(border_hwnd, GWL_EXSTYLE, style as i32) };

    _ = unsafe { update_border_corner_radius(border_hwnd, 10.0) };

    Ok(border_hwnd)
}

/// Update border color
pub unsafe fn update_border_color(border_hwnd: HWND, color: u32) -> Result<()> {
    let ptr = unsafe { GetWindowLongPtrW(border_hwnd, GWLP_USERDATA) };
    if ptr == 0 {
        return Err(Error::empty());
    }

    let data = unsafe { &mut *(ptr as *mut BorderWindowData) };

    let r = ((color >> 16) & 0xFF) as f32 / 255.0;
    let g = ((color >> 8) & 0xFF) as f32 / 255.0;
    let b = (color & 0xFF) as f32 / 255.0;

    data.color = D2D1_COLOR_F { r, g, b, a: 1.0 };

    if let Some(ref brush) = data.brush {
        unsafe { brush.SetColor(&data.color) };
    } else {
    }

    _ = unsafe { InvalidateRect(Some(border_hwnd), None, false) };
    Ok(())
}

pub unsafe fn update_border_rect(border_hwnd: HWND, rect: RECT) -> Result<()> {
    (unsafe {
        SetWindowPos(
            border_hwnd,
            Some(HWND_TOPMOST),
            rect.left,
            rect.top,
            rect.right - rect.left,
            rect.bottom - rect.top,
            SWP_NOACTIVATE | SWP_SHOWWINDOW,
        )
    })?;

    // Resize render target
    let ptr = unsafe { GetWindowLongPtrW(border_hwnd, GWLP_USERDATA) };
    if ptr != 0 {
        let data = unsafe { &mut *(ptr as *mut BorderWindowData) };
        if let Some(ref render_target) = data.render_target {
            let size = D2D_SIZE_U {
                width: (rect.right - rect.left) as u32,
                height: (rect.bottom - rect.top) as u32,
            };
            (unsafe { render_target.Resize(&size) })?;
        }
    }

    _ = unsafe { InvalidateRect(Some(border_hwnd), None, false) };

    Ok(())
}

/// Update border thickness
pub unsafe fn update_border_thickness(border_hwnd: HWND, thickness: f32) -> Result<()> {
    let ptr = unsafe { GetWindowLongPtrW(border_hwnd, GWLP_USERDATA) };
    if ptr == 0 {
        return Err(Error::empty());
    }

    let data = unsafe { &mut *(ptr as *mut BorderWindowData) };
    data.thickness = thickness;

    _ = unsafe { InvalidateRect(Some(border_hwnd), None, false) };

    Ok(())
}

/// Update border corner radius
pub unsafe fn update_border_corner_radius(border_hwnd: HWND, radius: f32) -> Result<()> {
    let ptr = unsafe { GetWindowLongPtrW(border_hwnd, GWLP_USERDATA) };
    if ptr == 0 {
        return Err(Error::empty());
    }

    let data = unsafe { &mut *(ptr as *mut BorderWindowData) };
    data.corner_radius = radius;

    _ = unsafe { InvalidateRect(Some(border_hwnd), None, false) };

    Ok(())
}

/// Destroy border window
pub unsafe fn destroy_border(border_hwnd: HWND) -> Result<()> {
    unsafe { DestroyWindow(border_hwnd) }
}

unsafe extern "system" fn border_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            let create_struct = unsafe { &*(lparam.0 as *const CREATESTRUCTW) };
            unsafe {
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, create_struct.lpCreateParams as isize)
            };

            LRESULT(0)
        }
        WM_PAINT => {
            let ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) };
            if ptr != 0 {
                let data = unsafe { &*(ptr as *mut BorderWindowData) };

                if let (Some(rt), Some(brush)) = (&data.render_target, &data.brush) {
                    unsafe { rt.BeginDraw() };

                    // Clear with fully transparent background
                    unsafe {
                        rt.Clear(Some(&D2D1_COLOR_F {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }))
                    };

                    let mut rect = RECT::default();
                    unsafe { GetClientRect(hwnd, &mut rect).ok() };

                    let width = (rect.right - rect.left) as f32;
                    let height = (rect.bottom - rect.top) as f32;
                    let thickness = data.thickness;
                    let half_thickness = thickness / 2.0;

                    // Draw border rectangle
                    let border_rect = D2D_RECT_F {
                        left: half_thickness,
                        top: half_thickness,
                        right: width - half_thickness,
                        bottom: height - half_thickness,
                    };

                    if data.corner_radius > 0.0 {
                        let rounded_rect = D2D1_ROUNDED_RECT {
                            rect: border_rect,
                            radiusX: data.corner_radius,
                            radiusY: data.corner_radius,
                        };
                        unsafe { rt.DrawRoundedRectangle(&rounded_rect, brush, thickness, None) };
                    } else {
                        unsafe { rt.DrawRectangle(&border_rect, brush, thickness, None) };
                    }

                    _ = unsafe { rt.EndDraw(None, None) };
                } else {
                }
            } else {
            }

            _ = unsafe { ValidateRect(Some(hwnd), None).ok() };
            LRESULT(0)
        }
        WM_ERASEBKGND => {
            // Don't erase background, we handle everything in WM_PAINT
            LRESULT(1)
        }
        WM_DESTROY => {
            let ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) };
            if ptr != 0 {
                let _ = unsafe { Box::from_raw(ptr as *mut BorderWindowData) };
                unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) };
            }
            LRESULT(0)
        }
        WM_NCHITTEST => LRESULT(HTTRANSPARENT as isize),
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}
pub unsafe fn get_window_border_rect(hwnd: HWND, thickness: i32) -> Result<RECT> {
    let mut rect = RECT::default();
    (unsafe { GetWindowRect(hwnd, &mut rect) })?;

    rect.left -= thickness;
    rect.top -= thickness;
    rect.right += thickness;
    rect.bottom += thickness;

    Ok(rect)
}

/// Get client rect - only the content area (no title bar/frame)
pub unsafe fn get_client_border_rect(hwnd: HWND, thickness: i32) -> Result<RECT> {
    let mut rect = RECT::default();
    (unsafe { GetClientRect(hwnd, &mut rect) })?;

    // Convert client rect to screen coordinates
    let mut top_left = POINT {
        x: rect.left,
        y: rect.top,
    };
    let mut bottom_right = POINT {
        x: rect.right,
        y: rect.bottom,
    };

    _ = unsafe { ClientToScreen(hwnd, &mut top_left) };
    _ = unsafe { ClientToScreen(hwnd, &mut bottom_right) };

    rect.left = top_left.x - thickness;
    rect.top = top_left.y - thickness;
    rect.right = bottom_right.x + thickness;
    rect.bottom = bottom_right.y + thickness;

    Ok(rect)
}

/// Get DWM extended frame bounds - visible window without shadow
pub unsafe fn get_dwm_border_rect(hwnd: HWND, thickness: i32) -> Result<RECT> {
    let mut rect = RECT::default();
    (unsafe {
        DwmGetWindowAttribute(
            hwnd,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut rect as *mut _ as *mut _,
            std::mem::size_of::<RECT>() as u32,
        )
    })?;

    rect.left -= thickness;
    rect.top -= thickness;
    rect.right += thickness;
    rect.bottom += thickness;

    Ok(rect)
}
