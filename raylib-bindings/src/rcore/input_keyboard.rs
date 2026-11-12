//! Input-related functions: keyboard

use std::mem::transmute;

/// Check if key has been pressed once
pub fn is_key_pressed(key: KeyboardKey) -> bool {
    unsafe { raylib_ffi::IsKeyPressed(key as i32) }
}

/// Check if key has been pressed again
pub fn is_key_pressed_repeat(key: KeyboardKey) -> bool {
    unsafe { raylib_ffi::IsKeyPressedRepeat(key as i32) }
}

/// Check if key is being pressed
pub fn is_key_down(key: KeyboardKey) -> bool {
    unsafe { raylib_ffi::IsKeyDown(key as i32) }
}

/// Check if key has been released once
pub fn is_key_released(key: KeyboardKey) -> bool {
    unsafe { raylib_ffi::IsKeyReleased(key as i32) }
}

/// Check if key is NOT being pressed
pub fn is_key_up(key: KeyboardKey) -> bool {
    unsafe { raylib_ffi::IsKeyUp(key as i32) }
}

/// Get key pressed (keycode), call it multiple times for keys queued, returns 0 when the queue is empty
pub fn get_key_pressed() -> KeyboardKey {
    // TODO UNSAFE: validate key
    unsafe { transmute(raylib_ffi::GetKeyPressed()) }
}

/// Get char pressed (unicode), call it multiple times for chars queued, returns 0 when the queue is empty
pub fn get_char_pressed() -> i32 {
    unsafe { raylib_ffi::GetCharPressed() }
}

/// Set a custom key to exit program (default is ESC)
pub fn set_exit_key(key: KeyboardKey) {
    unsafe { raylib_ffi::SetExitKey(key as i32) }
}

#[derive(Clone, Copy, Default, Debug)]
#[repr(i32)]
pub enum KeyboardKey {
    #[default]
    Null = 0,
    Apostrophe = 39,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Zero = 48,
    One = 49,
    Two = 50,
    Three = 51,
    Four = 52,
    Five = 53,
    Six = 54,
    Seven = 55,
    Eight = 56,
    Nine = 57,
    Semicolon = 59,
    Equal = 61,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftBracket = 91,
    Backslash = 92,
    RightBracket = 93,
    Grave = 96,
    Space = 32,
    Escape = 256,
    Enter = 257,
    Tab = 258,
    Backspace = 259,
    Insert = 260,
    Delete = 261,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
    PageUp = 266,
    PageDown = 267,
    Home = 268,
    End = 269,
    CapsLock = 280,
    ScrollLock = 281,
    NumLock = 282,
    PrintScreen = 283,
    Pause = 284,
    F1 = 290,
    F2 = 291,
    F3 = 292,
    F4 = 293,
    F5 = 294,
    F6 = 295,
    F7 = 296,
    F8 = 297,
    F9 = 298,
    F10 = 299,
    F11 = 300,
    F12 = 301,
    LeftShift = 340,
    LeftControl = 341,
    LeftAlt = 342,
    LeftSuper = 343,
    RightShift = 344,
    RightControl = 345,
    RightAlt = 346,
    RightSuper = 347,
    KbMenu = 348,
    Kp0 = 320,
    Kp1 = 321,
    Kp2 = 322,
    Kp3 = 323,
    Kp4 = 324,
    Kp5 = 325,
    Kp6 = 326,
    Kp7 = 327,
    Kp8 = 328,
    Kp9 = 329,
    KpDecimal = 330,
    KpDivide = 331,
    KpMultiply = 332,
    KpSubtract = 333,
    KpAdd = 334,
    KpEnter = 335,
    KpEqual = 336,
    Back = 4,
    VolumeUp = 24,
    VolumeDown = 25,
}
