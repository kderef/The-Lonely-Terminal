#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const fn from_raw(raw: raylib_ffi::Vector2) -> Self {
        Self { x: raw.x, y: raw.y }
    }
}
