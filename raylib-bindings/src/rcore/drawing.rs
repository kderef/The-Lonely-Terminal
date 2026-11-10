//! Drawing-related functions

use crate::rtextures::color::Color;

use super::screenspace::{Camera2D, Camera3D};
use super::shader::Shader;

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
pub fn begin_blend_mode(mode: i32) {}

/// End blending mode (reset to default: alpha blending)
pub fn end_blend_mode() {}

/// Begin scissor mode (define screen area for following drawing)
pub fn begin_scissor_mode(x: i32, y: i32, width: i32, height: i32) {}

/// End scissor mode
pub fn end_scissor_mode() {}

/// Begin stereo rendering (requires VR simulator)
//TODO pub fn BeginVrStereoMode(config: VrStereoConfig) {}

/// End stereo rendering (requires VR simulator)
pub fn end_vr_stereo_mode() {}
