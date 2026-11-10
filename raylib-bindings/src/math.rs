use std::mem::transmute;

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Ray {
    pub position: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub const fn from_raw(raw: raylib_ffi::Ray) -> Self {
        unsafe { transmute(raw) }
    }
    pub const fn raw(self) -> raylib_ffi::Ray {
        unsafe { transmute(self) }
    }
}

impl Vector3 {
    pub const fn from_raw(raw: raylib_ffi::Vector3) -> Self {
        unsafe { transmute(raw) }
    }
    pub const fn raw(self) -> raylib_ffi::Vector3 {
        unsafe { transmute(self) }
    }
}

impl Vector2 {
    pub const fn from_raw(raw: raylib_ffi::Vector2) -> Self {
        Self { x: raw.x, y: raw.y }
    }
    pub const fn raw(self) -> raylib_ffi::Vector2 {
        unsafe { transmute(self) }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Matrix {
    pub m0: f32,
    pub m4: f32,
    pub m8: f32,
    pub m12: f32,
    pub m1: f32,
    pub m5: f32,
    pub m9: f32,
    pub m13: f32,
    pub m2: f32,
    pub m6: f32,
    pub m10: f32,
    pub m14: f32,
    pub m3: f32,
    pub m7: f32,
    pub m11: f32,
    pub m15: f32,
}

impl Matrix {
    pub const fn from_raw(raw: raylib_ffi::Matrix) -> Self {
        unsafe { transmute(raw) }
    }
    pub const fn raw(self) -> raylib_ffi::Matrix {
        unsafe { transmute(self) }
    }
}
