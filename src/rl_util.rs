use raylib::prelude::*;
use crate::font;

// faster Vector3 initialization
#[macro_export]
macro_rules! vec3 {
    ($x:literal, $y:literal, $z:literal) => {
        Vector3{x: $x, y: $y, z: $z}
    }
}
#[macro_export]
macro_rules! vec2 {
    ($x:literal, $y:literal) => {
        Vector2{x: $x, y: $y}
    }
}

pub fn fail(err: impl Into<String>) -> ! {
    let _ = msgbox::create("ERROR", &err.into(), msgbox::IconType::Error);
    std::process::exit(1);
}

#[macro_export]
macro_rules! sound {
    ($path:literal) => {
        &format!(".{sep}audio{sep}{}", $path, sep = std::path::MAIN_SEPARATOR)
    }
}