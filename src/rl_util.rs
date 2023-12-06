use raylib::prelude::*;

// faster Vector3 initialization
#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vector3{x: $x, y: $y, z: $z}
    }
}
#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        Vector2{x: $x, y: $y}
    }
}

#[macro_export]
macro_rules! load_font {
    ($name:literal) => {
        unsafe {
            let bytes = include_bytes!($name);
            let bytes_len = bytes.len() as i32;

            let raw = raylib::ffi::LoadFontFromMemory(
                ".ttf".as_ptr() as *const i8,
                bytes.as_ptr(),
                bytes_len,
                100,
                std::ptr::null_mut(),
                0
            );
            Font::from_raw(raw)
        }
    }
}