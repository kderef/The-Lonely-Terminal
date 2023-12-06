// faster Vector3 initialization
#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vector3 {
            x: $x,
            y: $y,
            z: $z,
        }
    };
}
#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        Vector2 { x: $x, y: $y }
    };
}

#[macro_export]
macro_rules! load_font {
    ($name:literal) => {
        crate::rl_port::load_font_from_memory(".ttf", include_bytes!($name), 100)
    };
}

#[macro_export]
macro_rules! load_sound {
    ($name:literal, $ftype:literal) => {{
        const BYTES: &[u8] = include_bytes!($name);
        crate::rl_port::load_sound_from_memory($ftype, BYTES)
    }};
}
