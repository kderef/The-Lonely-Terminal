//! Screen-space and camera related functions

use std::mem::transmute;

use crate::math::{Matrix, Ray, Vector2, Vector3};

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    pub projection: i32,
}

pub type Camera = Camera3D;

impl Camera2D {
    pub const fn raw(self) -> raylib_ffi::Camera2D {
        unsafe { transmute(self) }
    }
    pub const fn from_raw(raw: raylib_ffi::Camera2D) -> Self {
        unsafe { transmute(raw) }
    }
}

impl Camera3D {
    pub const fn raw(self) -> raylib_ffi::Camera3D {
        unsafe { transmute(self) }
    }
    pub const fn from_raw(raw: raylib_ffi::Camera3D) -> Self {
        unsafe { transmute(raw) }
    }
}

/// Get a ray trace from screen position (i.e mouse)
pub fn GetScreenToWorldRay(position: Vector2, camera: Camera) -> Ray {
    Ray::from_raw(unsafe { raylib_ffi::GetScreenToWorldRay(position.raw(), camera.raw()) })
}

/// Get a ray trace from screen position (i.e mouse) in a viewport
pub fn GetScreenToWorldRayEx(position: Vector2, camera: Camera, width: i32, height: i32) -> Ray {
    Ray::from_raw(unsafe {
        raylib_ffi::GetScreenToWorldRayEx(position.raw(), camera.raw(), width, height)
    })
}

/// Get screen space position for a 3d world space position
pub fn GetWorldToScreen(position: Vector3, camera: Camera) -> Vector2 {
    Vector2::from_raw(unsafe { raylib_ffi::GetWorldToScreen(position.raw(), camera.raw()) })
}

/// Get sized screen space position for a 3d world space position
pub fn GetWorldToScreenEx(position: Vector3, camera: Camera, width: i32, height: i32) -> Vector2 {
    Vector2::from_raw(unsafe {
        raylib_ffi::GetWorldToScreenEx(position.raw(), camera.raw(), width, height)
    })
}

/// Get screen space position for a 2d camera world space position
pub fn GetWorldToScreen2D(position: Vector2, camera: Camera2D) -> Vector2 {
    Vector2::from_raw(unsafe { raylib_ffi::GetWorldToScreen2D(position.raw(), camera.raw()) })
}

/// Get world space position for a 2d camera screen space position
pub fn GetScreenToWorld2D(position: Vector2, camera: Camera2D) -> Vector2 {
    Vector2::from_raw(unsafe { raylib_ffi::GetScreenToWorld2D(position.raw(), camera.raw()) })
}

/// Get camera transform matrix (view matrix)
pub fn GetCameraMatrix(camera: Camera) -> Matrix {
    Matrix::from_raw(unsafe { raylib_ffi::GetCameraMatrix(camera.raw()) })
}
