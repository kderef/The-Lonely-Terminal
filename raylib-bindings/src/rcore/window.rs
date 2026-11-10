//! window related functions

use std::{
    ffi::{CStr, CString, NulError, c_void},
    str::Utf8Error,
};

use crate::{image::Image, math::Vector2};

/// raylib window handle, see `init_window()`
/// `close_window()` is replaced with drop.
pub struct Window(());

/// System/Window config flags
/// NOTE: Every bit registers one state (use it with bit masks)
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum ConfigFlags {
    FLAG_VSYNC_HINT = 64,
    FLAG_FULLSCREEN_MODE = 2,
    FLAG_WINDOW_RESIZABLE = 4,
    FLAG_WINDOW_UNDECORATED = 8,
    FLAG_WINDOW_HIDDEN = 128,
    FLAG_WINDOW_MINIMIZED = 512,
    FLAG_WINDOW_MAXIMIZED = 1_024,
    FLAG_WINDOW_UNFOCUSED = 2_048,
    FLAG_WINDOW_TOPMOST = 4_096,
    FLAG_WINDOW_ALWAYS_RUN = 256,
    FLAG_WINDOW_TRANSPARENT = 16,
    FLAG_WINDOW_HIGHDPI = 8_192,
    FLAG_WINDOW_MOUSE_PASSTHROUGH = 16_384,
    FLAG_BORDERLESS_WINDOWED_MODE = 32_768,
    FLAG_MSAA_4X_HINT = 32,
    FLAG_INTERLACED_HINT = 65_536,
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            raylib_ffi::CloseWindow();
        }
    }
}

/// Initialize window and OpenGL context.
/// panics if title is not a valid string
pub fn init_window(width: i32, height: i32, title: &str) -> Window {
    let title_cstr = CString::new(title).expect("Window title should be a valid utf8 string");

    unsafe {
        raylib_ffi::InitWindow(width, height, title_cstr.as_ptr());
    }

    Window(())
}

/// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
pub fn window_should_close() -> bool {
    unsafe { raylib_ffi::WindowShouldClose() }
}

/// Check if window has been initialized successfully
pub fn is_window_ready() -> bool {
    unsafe { raylib_ffi::IsWindowReady() }
}

/// Check if window is currently fullscreen
pub fn is_window_fullscreen() -> bool {
    unsafe { raylib_ffi::IsWindowFullscreen() }
}

/// Check if window is currently hidden
pub fn is_window_hidden() -> bool {
    unsafe { raylib_ffi::IsWindowHidden() }
}

/// Check if window is currently minimized
pub fn is_window_minimized() -> bool {
    unsafe { raylib_ffi::IsWindowMinimized() }
}

/// Check if window is currently maximized
pub fn is_window_maximized() -> bool {
    unsafe { raylib_ffi::IsWindowMaximized() }
}

/// Check if window is currently focused
pub fn is_window_focused() -> bool {
    unsafe { raylib_ffi::IsWindowFocused() }
}

/// Check if window has been resized last frame
pub fn is_window_resized() -> bool {
    unsafe { raylib_ffi::IsWindowResized() }
}

/// Check if one specific window flag is enabled
pub fn is_window_state(flag: ConfigFlags) -> bool {
    unsafe { raylib_ffi::IsWindowState(flag as u32) }
}

/// Set window configuration state using flags
pub fn set_window_state(flags: ConfigFlags) {
    unsafe { raylib_ffi::SetWindowState(flags as u32) }
}
/// Clear window configuration state flags
pub fn clear_window_state(flags: ConfigFlags) {
    unsafe { raylib_ffi::ClearWindowState(flags as u32) }
}
/// Toggle window state: fullscreen/windowed, resizes monitor to match window resolution
pub fn toggle_fullscreen() {
    unsafe { raylib_ffi::ToggleFullscreen() }
}
/// Toggle window state: borderless windowed, resizes window to match monitor resolution
pub fn toggle_borderless_windowed() {
    unsafe { raylib_ffi::ToggleBorderlessWindowed() }
}
/// Set window state: maximized, if resizable
pub fn maximize_window() {
    unsafe { raylib_ffi::MaximizeWindow() }
}
/// Set window state: minimized, if resizable
pub fn minimize_window() {
    unsafe { raylib_ffi::MinimizeWindow() }
}
/// Restore window from being minimized/maximized
pub fn restore_window() {
    unsafe { raylib_ffi::RestoreWindow() }
}
/// Set icon for window (single image, RGBA 32bit)
pub fn set_window_icon(image: Image) {
    unsafe { raylib_ffi::SetWindowIcon(image.0) }
}

/// Set icon for window (multiple images, RGBA 32bit)
pub fn set_window_icons(images: &[Image], count: i32) {
    let images_ptr = images.as_ptr() as *mut raylib_ffi::Image;

    unsafe { raylib_ffi::SetWindowIcons(images_ptr, count) }
}
/// Set title for window
/// panics if title is invalid.
pub fn set_window_title(title: &str) {
    let title_cstr = CString::new(title).expect("Window title should be a valid utf8 string.");
    unsafe { raylib_ffi::SetWindowTitle(title_cstr.as_ptr()) }
}

/// Set window position on screen
pub fn set_window_position(x: i32, y: i32) {
    unsafe { raylib_ffi::SetWindowPosition(x, y) }
}

/// Set monitor for the current window
pub fn set_window_monitor(monitor: i32) {
    unsafe { raylib_ffi::SetWindowMonitor(monitor) }
}

/// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
pub fn set_window_min_size(width: i32, height: i32) {
    unsafe { raylib_ffi::SetWindowMinSize(width, height) }
}

/// Set window maximum dimensions (for FLAG_WINDOW_RESIZABLE)
pub fn set_window_max_size(width: i32, height: i32) {
    unsafe { raylib_ffi::SetWindowMaxSize(width, height) }
}

/// Set window dimensions
pub fn set_window_size(width: i32, height: i32) {
    unsafe { raylib_ffi::SetWindowSize(width, height) }
}

/// Set window opacity [0.0f..1.0f]
pub fn set_window_opacity(opacity: f32) {
    unsafe { raylib_ffi::SetWindowOpacity(opacity) }
}

/// Set window focused
pub fn set_window_focused() {
    unsafe { raylib_ffi::SetWindowFocused() }
}

/// Get native window handle
pub fn get_window_handle() -> *mut c_void {
    unsafe { raylib_ffi::GetWindowHandle() }
}

/// Get current screen width
pub fn get_screen_width() -> i32 {
    unsafe { raylib_ffi::GetScreenWidth() }
}

/// Get current screen height
pub fn get_screen_height() -> i32 {
    unsafe { raylib_ffi::GetScreenHeight() }
}

/// Get current render width (it considers HiDPI)
pub fn get_render_width() -> i32 {
    unsafe { raylib_ffi::GetRenderWidth() }
}

/// Get current render height (it considers HiDPI)
pub fn get_render_height() -> i32 {
    unsafe { raylib_ffi::GetRenderHeight() }
}

/// Get number of connected monitors
pub fn get_monitor_count() -> i32 {
    unsafe { raylib_ffi::GetMonitorCount() }
}

/// Get current monitor where window is placed
pub fn get_current_monitor() -> i32 {
    unsafe { raylib_ffi::GetCurrentMonitor() }
}

/// Get specified monitor position
pub fn get_monitor_position(monitor: i32) -> Vector2 {
    unsafe { Vector2::from_raw(raylib_ffi::GetMonitorPosition(monitor)) }
}

/// Get specified monitor width (current video mode used by monitor)
pub fn get_monitor_width(monitor: i32) -> i32 {
    unsafe { raylib_ffi::GetMonitorWidth(monitor) }
}

/// Get specified monitor height (current video mode used by monitor)
pub fn get_monitor_height(monitor: i32) -> i32 {
    unsafe { raylib_ffi::GetMonitorHeight(monitor) }
}

/// Get specified monitor physical width in millimetres
pub fn get_monitor_physical_width(monitor: i32) -> i32 {
    unsafe { raylib_ffi::GetMonitorPhysicalWidth(monitor) }
}

/// Get specified monitor physical height in millimetres
pub fn get_monitor_physical_height(monitor: i32) -> i32 {
    unsafe { raylib_ffi::GetMonitorPhysicalHeight(monitor) }
}

/// Get specified monitor refresh rate
pub fn get_monitor_refresh_rate(monitor: i32) -> i32 {
    unsafe { raylib_ffi::GetMonitorRefreshRate(monitor) }
}

/// Get window position XY on monitor
pub fn get_window_position() -> Vector2 {
    unsafe { Vector2::from_raw(raylib_ffi::GetWindowPosition()) }
}

/// Get window scale DPI factor
pub fn get_window_scale_dpi() -> Vector2 {
    unsafe { Vector2::from_raw(raylib_ffi::GetWindowScaleDPI()) }
}

/// Get the human-readable, UTF-8 encoded name of the specified monitor
pub fn get_monitor_name(monitor: i32) -> Result<String, Utf8Error> {
    let name_raw = unsafe { raylib_ffi::GetMonitorName(monitor) };
    let name_cstr = unsafe { CStr::from_ptr(name_raw) };

    let name = name_cstr.to_str()?;
    Ok(name.to_string())
}

/// Set clipboard text content
pub fn set_clipboard_text(text: &str) -> Result<(), NulError> {
    let text_cstr = CString::new(text)?;

    unsafe {
        raylib_ffi::SetClipboardText(text_cstr.as_ptr());
    }

    Ok(())
}

/// Get clipboard text content
pub fn get_clipboard_text() -> Result<String, Utf8Error> {
    let text_raw = unsafe { raylib_ffi::GetClipboardText() };
    let text_cstr = unsafe { CStr::from_ptr(text_raw) };

    let name = text_cstr.to_str()?;
    Ok(name.to_string())
}

/// Get clipboard image content
pub fn get_clipboard_image() -> Image {
    let raw = unsafe { raylib_ffi::GetClipboardImage() };

    Image(raw)
}

/// Enable waiting for events on EndDrawing(), no automatic event polling
pub fn enable_event_waiting() {
    unsafe { raylib_ffi::EnableEventWaiting() }
}

/// Disable waiting for events on EndDrawing(), automatic events polling
pub fn disable_event_waiting() {
    unsafe { raylib_ffi::DisableEventWaiting() }
}
