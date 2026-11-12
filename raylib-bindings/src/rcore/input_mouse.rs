//! Input-related functions: mouse

use crate::math::Vector2;

#[derive(Clone, Copy, Default, Debug)]
#[repr(i32)]
pub enum MouseButton {
    #[default]
    Left = 0,
    Right = 1,
    Middle = 2,
    Side = 3,
    Extra = 4,
    Forward = 5,
    Back = 6,
}

#[derive(Clone, Copy, Default, Debug)]
#[repr(i32)]
pub enum MouseCursor {
    #[default]
    Default = 0,
    Arrow = 1,
    Ibeam = 2,
    Crosshair = 3,
    PointingHand = 4,
    ResizeEw = 5,
    ResizeNs = 6,
    ResizeNwse = 7,
    ResizeNesw = 8,
    ResizeAll = 9,
    NotAllowed = 10,
}

/// Check if mouse button has been pressed once
pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
    unsafe { raylib_ffi::IsMouseButtonPressed(button as i32) }
}

/// Check if mouse button is being pressed
pub fn is_mouse_button_down(button: MouseButton) -> bool {
    unsafe { raylib_ffi::IsMouseButtonDown(button as i32) }
}

/// Check if mouse button has been released once
pub fn is_mouse_button_released(button: MouseButton) -> bool {
    unsafe { raylib_ffi::IsMouseButtonReleased(button as i32) }
}

/// Check if mouse button is NOT being pressed
pub fn is_mouse_button_up(button: MouseButton) -> bool {
    unsafe { raylib_ffi::IsMouseButtonUp(button as i32) }
}

/// Get mouse position X
pub fn get_mouse_x() -> i32 {
    unsafe { raylib_ffi::GetMouseX() }
}

/// Get mouse position Y
pub fn get_mouse_y() -> i32 {
    unsafe { raylib_ffi::GetMouseY() }
}

/// Get mouse position XY
pub fn get_mouse_position() -> Vector2 {
    Vector2::from_raw(unsafe { raylib_ffi::GetMousePosition() })
}

/// Get mouse delta between frames
pub fn get_mouse_delta() -> Vector2 {
    Vector2::from_raw(unsafe { raylib_ffi::GetMouseDelta() })
}

/// Set mouse position XY
pub fn set_mouse_position(x: i32, y: i32) {
    unsafe { raylib_ffi::SetMousePosition(x, y) }
}

/// Set mouse offset
pub fn set_mouse_offset(offset_x: i32, offset_y: i32) {
    unsafe { raylib_ffi::SetMouseOffset(offset_x, offset_y) }
}

/// Set mouse scaling
pub fn set_mouse_scale(scale_x: f32, scale_y: f32) {
    unsafe { raylib_ffi::SetMouseScale(scale_x, scale_y) }
}

/// Get mouse wheel movement for X or Y, whichever is larger
pub fn get_mouse_wheel_move() -> f32 {
    unsafe { raylib_ffi::GetMouseWheelMove() }
}

/// Get mouse wheel movement for both X and Y
pub fn get_mouse_wheel_move_v() -> Vector2 {
    Vector2::from_raw(unsafe { raylib_ffi::GetMouseWheelMoveV() })
}

/// Set mouse cursor
pub fn set_mouse_cursor(cursor: MouseCursor) {
    unsafe { raylib_ffi::SetMouseCursor(cursor as i32) }
}
