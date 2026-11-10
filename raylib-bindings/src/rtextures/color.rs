//! Color/pixel related functions

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Light Gray
    pub const LIGHTGRAY: Self = Self::new(200, 200, 200, 255);

    /// Gray
    pub const GRAY: Self = Self::new(130, 130, 130, 255);

    /// Dark Gray
    pub const DARKGRAY: Self = Self::new(80, 80, 80, 255);

    /// Yellow
    pub const YELLOW: Self = Self::new(253, 249, 0, 255);

    /// Gold
    pub const GOLD: Self = Self::new(255, 203, 0, 255);

    /// Orange
    pub const ORANGE: Self = Self::new(255, 161, 0, 255);

    /// Pink
    pub const PINK: Self = Self::new(255, 109, 194, 255);

    /// Red
    pub const RED: Self = Self::new(230, 41, 55, 255);

    /// Maroon
    pub const MAROON: Self = Self::new(190, 33, 55, 255);

    /// Green
    pub const GREEN: Self = Self::new(0, 228, 48, 255);

    /// Lime
    pub const LIME: Self = Self::new(0, 158, 47, 255);

    /// Dark Green
    pub const DARKGREEN: Self = Self::new(0, 117, 44, 255);

    /// Sky Blue
    pub const SKYBLUE: Self = Self::new(102, 191, 255, 255);

    /// Blue
    pub const BLUE: Self = Self::new(0, 121, 241, 255);

    /// Dark Blue
    pub const DARKBLUE: Self = Self::new(0, 82, 172, 255);

    /// Purple
    pub const PURPLE: Self = Self::new(200, 122, 255, 255);

    /// Violet
    pub const VIOLET: Self = Self::new(135, 60, 190, 255);

    /// Dark Purple
    pub const DARKPURPLE: Self = Self::new(112, 31, 126, 255);

    /// Beige
    pub const BEIGE: Self = Self::new(211, 176, 131, 255);

    /// Brown
    pub const BROWN: Self = Self::new(127, 106, 79, 255);

    /// Dark Brown
    pub const DARKBROWN: Self = Self::new(76, 63, 47, 255);

    /// White
    pub const WHITE: Self = Self::new(255, 255, 255, 255);

    /// Black
    pub const BLACK: Self = Self::new(0, 0, 0, 255);

    /// Blank (Transparent)
    pub const BLANK: Self = Self::new(0, 0, 0, 0);

    /// Magenta
    pub const MAGENTA: Self = Self::new(255, 0, 255, 255);

    /// My own White (raylib logo)
    pub const RAYWHITE: Self = Self::new(245, 245, 245, 255);
}
