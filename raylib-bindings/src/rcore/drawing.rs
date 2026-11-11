//! Drawing-related functions

use std::mem::transmute;

use crate::math::Matrix;
use crate::rtextures::color::Color;

use super::screenspace::{Camera2D, Camera3D};
use super::shader::Shader;

#[derive(Debug, Clone, Copy, Default)]
#[repr(i32)]
pub enum BlendMode {
    #[default]
    Alpha = 0,
    Additive = 1,
    Multiplied = 2,
    AddColors = 3,
    SubtractColors = 4,
    AlphaPremultiply = 5,
    Custom = 6,
    CustomSeparate = 7,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VrStereoConfig {
    pub projection: [Matrix; 2],
    pub view_offset: [Matrix; 2],
    pub left_lens_center: [f32; 2],
    pub right_lens_center: [f32; 2],
    pub left_screen_center: [f32; 2],
    pub right_screen_center: [f32; 2],
    pub scale: [f32; 2],
    pub scale_in: [f32; 2],
}

/// Clear background (framebuffer) to color
pub fn clear_background(color: Color) {
    unsafe {
        raylib_ffi::ClearBackground(std::mem::transmute(color));
    }
}

/// Begin canvas (framebuffer) drawing
pub fn begin_drawing() {
    unsafe { raylib_ffi::BeginDrawing() }
}

/// End canvas (framebuffer) drawing and swap buffers (double buffering)
pub fn end_drawing() {
    unsafe { raylib_ffi::EndDrawing() }
}

/// Begin 2D mode with custom camera (2D)
pub fn begin_mode_2d(camera: Camera2D) {
    unsafe { raylib_ffi::BeginMode2D(camera.raw()) }
}

/// End 2D mode with custom camera
pub fn end_mode_2d() {
    unsafe { raylib_ffi::EndMode2D() }
}

/// Begin 3D mode with custom camera (3D)
pub fn begin_mode_3d(camera: Camera3D) {
    unsafe { raylib_ffi::BeginMode3D(camera.raw()) }
}

/// End 3D mode and returns to default 2D orthographic mode
pub fn end_mode_3d() {
    unsafe { raylib_ffi::EndMode3D() }
}

/// Begin drawing to render texture
// TODO pub fn BeginTextureMode(target: RenderTexture2D) {}

/// End drawing to render texture
pub fn end_texture_mode() {
    unsafe { raylib_ffi::EndTextureMode() }
}

/// Begin custom shader drawing
pub fn begin_shader_mode(shader: &Shader) {
    unsafe { raylib_ffi::BeginShaderMode(shader.raw()) }
}

/// End custom shader drawing (use default shader)
pub fn end_shader_mode() {
    unsafe { raylib_ffi::EndShaderMode() }
}

/// Begin blending mode (alpha, additive, multiplied, subtract, custom)
pub fn begin_blend_mode(mode: BlendMode) {
    unsafe { raylib_ffi::BeginBlendMode(mode as i32) }
}

/// End blending mode (reset to default: alpha blending)
pub fn end_blend_mode() {
    unsafe { raylib_ffi::EndBlendMode() }
}

/// Begin scissor mode (define screen area for following drawing)
pub fn begin_scissor_mode(x: i32, y: i32, width: i32, height: i32) {
    unsafe { raylib_ffi::BeginScissorMode(x, y, width, height) }
}

/// End scissor mode
pub fn end_scissor_mode() {
    unsafe { raylib_ffi::EndScissorMode() }
}

/// Begin stereo rendering (requires VR simulator)
pub fn begin_vr_stereo_mode(config: VrStereoConfig) {
    unsafe { raylib_ffi::BeginVrStereoMode(transmute(config)) }
}

/// End stereo rendering (requires VR simulator)
pub fn end_vr_stereo_mode() {
    unsafe { raylib_ffi::EndVrStereoMode() }
}
