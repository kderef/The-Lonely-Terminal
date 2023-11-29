use std::ffi::{c_char, CString};

use raylib::{RaylibHandle, RaylibThread, text::Font, ffi::LoadFontFromMemory};

use crate::config::font;

// faster Vector3 initialization
#[macro_export]
macro_rules! vec3 {
    ($x:literal, $y:literal, $z:literal) => {
        Vector3{x: $x, y: $y, z: $z}
    }
}