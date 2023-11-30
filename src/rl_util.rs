use raylib::prelude::*;
use crate::font;

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

#[macro_export]
macro_rules! rl_sleep {
    // TODO
    ($secs:literal, $dr:ident) => {
        for _ in 0..($secs * 100) {
            std::thread::sleep(std::time::Duration::from_nanos(100));
            $dr.update()
        }
    }
}