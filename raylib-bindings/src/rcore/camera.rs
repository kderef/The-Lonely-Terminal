use crate::math::Vector2;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Camera3D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}
