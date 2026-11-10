//! Cursor-related functions

/// Show cursor
pub fn show_cursor() {
    unsafe { raylib_ffi::ShowCursor() }
}

/// Hide cursor
pub fn hide_cursor() {
    unsafe {
        raylib_ffi::HideCursor();
    }
}

/// Check if cursor is not visible
pub fn is_cursor_hidden() -> bool {
    unsafe { raylib_ffi::IsCursorHidden() }
}

/// Enable cursor (unlock cursor)
pub fn enable_cursor() {
    unsafe { raylib_ffi::EnableCursor() }
}

/// Disable cursor (lock cursor)
pub fn disable_cursor() {
    unsafe { raylib_ffi::DisableCursor() }
}

/// Check if cursor is on the screen
pub fn is_cursor_on_screen() -> bool {
    unsafe { raylib_ffi::IsCursorOnScreen() }
}
