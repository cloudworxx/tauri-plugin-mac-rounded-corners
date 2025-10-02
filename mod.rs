// Unterdrücke Warnings von veralteten Cocoa APIs
#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use tauri::{AppHandle, Runtime, WebviewWindow};

#[cfg(target_os = "macos")]
use cocoa::{
    appkit::{NSWindow, NSWindowStyleMask, NSView, NSWindowTitleVisibility},
    base::id,
    foundation::NSPoint,
};

#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};

/// Configuration for Traffic Lights positioning
pub struct TrafficLightsConfig {
    /// Offset in pixels from default position (positive = right, negative = left)
    pub offset_x: f64,
    /// Offset in pixels from default position (positive = down, negative = up)
    pub offset_y: f64,
}

impl Default for TrafficLightsConfig {
    fn default() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

/// Enables rounded corners for the window (macOS only)
/// Uses only public APIs - App Store compatible
#[tauri::command]
pub fn enable_rounded_corners<R: Runtime>(
    _app: AppHandle<R>,
    window: WebviewWindow<R>,
    offset_x: Option<f64>,
    offset_y: Option<f64>,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let config = TrafficLightsConfig {
            offset_x: offset_x.unwrap_or(0.0),
            offset_y: offset_y.unwrap_or(0.0),
        };

        window
            .with_webview(move |webview| {
                #[cfg(target_os = "macos")]
                unsafe {
                    let ns_window = webview.ns_window() as id;
                    
                    let mut style_mask = ns_window.styleMask();
                    
                    // Add necessary styles for rounded corners
                    style_mask |= NSWindowStyleMask::NSFullSizeContentViewWindowMask;
                    style_mask |= NSWindowStyleMask::NSTitledWindowMask;
                    style_mask |= NSWindowStyleMask::NSClosableWindowMask;
                    style_mask |= NSWindowStyleMask::NSMiniaturizableWindowMask;
                    style_mask |= NSWindowStyleMask::NSResizableWindowMask;
                    
                    ns_window.setStyleMask_(style_mask);
                    ns_window.setTitlebarAppearsTransparent_(cocoa::base::YES);
                    
                    let content_view = ns_window.contentView();
                    content_view.setWantsLayer(cocoa::base::YES);
                    
                    position_traffic_lights(ns_window, config.offset_x, config.offset_y);
                }
            })
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        Ok(())
    }
}

/// Enables modern window style with rounded corners and shadow
#[tauri::command]
pub fn enable_modern_window_style<R: Runtime>(
    _app: AppHandle<R>,
    window: WebviewWindow<R>,
    corner_radius: Option<f64>,
    offset_x: Option<f64>,
    offset_y: Option<f64>,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let config = TrafficLightsConfig {
            offset_x: offset_x.unwrap_or(0.0),
            offset_y: offset_y.unwrap_or(0.0),
        };
        let radius = corner_radius.unwrap_or(12.0);

        window
            .with_webview(move |webview| {
                #[cfg(target_os = "macos")]
                unsafe {
                    let ns_window = webview.ns_window() as id;
                    
                    let mut style_mask = ns_window.styleMask();
                    
                    style_mask |= NSWindowStyleMask::NSFullSizeContentViewWindowMask;
                    style_mask |= NSWindowStyleMask::NSTitledWindowMask;
                    style_mask |= NSWindowStyleMask::NSClosableWindowMask;
                    style_mask |= NSWindowStyleMask::NSMiniaturizableWindowMask;
                    style_mask |= NSWindowStyleMask::NSResizableWindowMask;
                    
                    ns_window.setStyleMask_(style_mask);
                    ns_window.setTitlebarAppearsTransparent_(cocoa::base::YES);
                    ns_window.setTitleVisibility_(NSWindowTitleVisibility::NSWindowTitleHidden);
                    ns_window.setHasShadow_(cocoa::base::YES);
                    ns_window.setOpaque_(cocoa::base::NO);
                    
                    let content_view = ns_window.contentView();
                    content_view.setWantsLayer(cocoa::base::YES);
                    
                    let layer: id = msg_send![content_view, layer];
                    if !layer.is_null() {
                        let _: () = msg_send![layer, setCornerRadius: radius];
                        let _: () = msg_send![layer, setMasksToBounds: cocoa::base::YES];
                    }
                    
                    position_traffic_lights(ns_window, config.offset_x, config.offset_y);
                }
            })
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        Ok(())
    }
}

/// Repositions Traffic Lights only (useful after fullscreen toggle)
#[tauri::command]
pub fn reposition_traffic_lights<R: Runtime>(
    _app: AppHandle<R>,
    window: WebviewWindow<R>,
    offset_x: Option<f64>,
    offset_y: Option<f64>,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let config = TrafficLightsConfig {
            offset_x: offset_x.unwrap_or(0.0),
            offset_y: offset_y.unwrap_or(0.0),
        };

        window
            .with_webview(move |webview| {
                #[cfg(target_os = "macos")]
                unsafe {
                    let ns_window = webview.ns_window() as id;
                    position_traffic_lights(ns_window, config.offset_x, config.offset_y);
                }
            })
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        Ok(())
    }
}

#[cfg(target_os = "macos")]
unsafe fn position_traffic_lights(ns_window: id, offset_x: f64, offset_y: f64) {
    let default_x = 20.0;
    let default_y = 0.0;
    
    let close_button: id = msg_send![ns_window, standardWindowButton: 0];
    let miniaturize_button: id = msg_send![ns_window, standardWindowButton: 1];
    let zoom_button: id = msg_send![ns_window, standardWindowButton: 2];
    
    let new_x = default_x + offset_x;
    let new_y = default_y - offset_y;
    
    if !close_button.is_null() {
        let frame: cocoa::foundation::NSRect = msg_send![close_button, frame];
        let new_frame = cocoa::foundation::NSRect::new(
            NSPoint::new(new_x, new_y),
            frame.size,
        );
        let _: () = msg_send![close_button, setFrame: new_frame];
    }
    
    if !miniaturize_button.is_null() {
        let frame: cocoa::foundation::NSRect = msg_send![miniaturize_button, frame];
        let new_frame = cocoa::foundation::NSRect::new(
            NSPoint::new(new_x + 20.0, new_y),
            frame.size,
        );
        let _: () = msg_send![miniaturize_button, setFrame: new_frame];
    }
    
    if !zoom_button.is_null() {
        let frame: cocoa::foundation::NSRect = msg_send![zoom_button, frame];
        let new_frame = cocoa::foundation::NSRect::new(
            NSPoint::new(new_x + 40.0, new_y),
            frame.size,
        );
        let _: () = msg_send![zoom_button, setFrame: new_frame];
    }
}
