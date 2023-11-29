use raylib::prelude::*;

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