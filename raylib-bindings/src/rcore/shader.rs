//! Shader management functions
//! NOTE: Shader functionality is not available on OpenGL 1.1

use std::{
    ffi::{CString, c_void},
    mem::transmute_copy,
};

use raylib_ffi::Texture2D;

use crate::math::Matrix;

#[derive(Debug)]
#[repr(C)]
pub struct Shader {
    pub id: u32,
    pub locs: *mut i32,

    // if it was publicly constructable it would call the destructor on invalid shaders.
    _phantom: (),
}

impl Shader {
    pub const fn raw(&self) -> raylib_ffi::Shader {
        raylib_ffi::Shader {
            id: self.id,
            locs: self.locs,
        }
    }

    /// Load shader from files and bind default locations
    pub fn load(vs_file_name: &str, fs_file_name: &str) -> Self {
        load_shader(vs_file_name, fs_file_name)
    }

    /// Load shader from code strings and bind default locations
    pub fn load_from_memory(vs_code: &[u8], fs_code: &[u8]) -> Self {
        load_shader_from_memory(vs_code, fs_code)
    }

    /// Check if shader is valid (loaded on GPU)
    pub fn is_valid(&self) -> bool {
        is_shader_valid(self)
    }

    /// Get shader uniform location
    pub fn get_location(&self, uniform_name: &str) -> i32 {
        get_shader_location(self, uniform_name)
    }

    /// Get shader attribute location
    pub fn get_location_attrib(&self, attrib_name: &str) -> i32 {
        get_shader_location_attrib(self, attrib_name)
    }

    /// Set shader uniform value
    pub fn set_value<T>(&self, loc_index: i32, value: &T, uniform_type: i32) {
        set_shader_value(self, loc_index, value, uniform_type);
    }

    /// Set shader uniform value vector
    pub fn set_value_v<T>(&self, loc_index: i32, value: &T, uniform_type: i32, count: i32) {
        set_shader_value_v(self, loc_index, value, uniform_type, count);
    }

    /// Set shader uniform value (matrix 4x4)
    pub fn set_value_matrix(shader: &Shader, loc_index: i32, mat: Matrix) {
        set_shader_value_matrix(shader, loc_index, mat);
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { raylib_ffi::UnloadShader(transmute_copy(self)) }
    }
}

/// Load shader from files and bind default locations
pub fn load_shader(vs_file_name: &str, fs_file_name: &str) -> Shader {
    let vs_file_name = CString::new(vs_file_name).expect("vs file name should be valid utf8");
    let fs_file_name = CString::new(fs_file_name).expect("fs file name should be valid utf8");

    let shader_raw =
        unsafe { raylib_ffi::LoadShader(vs_file_name.as_ptr(), fs_file_name.as_ptr()) };

    Shader {
        id: shader_raw.id,
        locs: shader_raw.locs,
        _phantom: (),
    }
}

/// Load shader from code strings and bind default locations
pub fn load_shader_from_memory(vs_code: &[u8], fs_code: &[u8]) -> Shader {
    let shader_raw = unsafe {
        raylib_ffi::LoadShaderFromMemory(
            vs_code.as_ptr() as *const i8,
            fs_code.as_ptr() as *const i8,
        )
    };

    Shader {
        id: shader_raw.id,
        locs: shader_raw.locs,
        _phantom: (),
    }
}
/// Check if shader is valid (loaded on GPU)
pub fn is_shader_valid(shader: &Shader) -> bool {
    unsafe { raylib_ffi::IsShaderValid(transmute_copy(shader)) }
}

/// Get shader uniform location
pub fn get_shader_location(shader: &Shader, uniform_name: &str) -> i32 {
    let uniform_name = CString::new(uniform_name).expect("uniform name should be valid utf8");

    unsafe { raylib_ffi::GetShaderLocation(transmute_copy(shader), uniform_name.as_ptr()) }
}

/// Get shader attribute location
pub fn get_shader_location_attrib(shader: &Shader, attrib_name: &str) -> i32 {
    let attrib_name = CString::new(attrib_name).expect("attrib name should be valid utf8");
    unsafe { raylib_ffi::GetShaderLocationAttrib(transmute_copy(shader), attrib_name.as_ptr()) }
}

/// Set shader uniform value
pub fn set_shader_value<T>(shader: &Shader, loc_index: i32, value: &T, uniform_type: i32) {
    unsafe {
        raylib_ffi::SetShaderValue(
            transmute_copy(shader),
            loc_index,
            value as *const _ as *const c_void,
            uniform_type,
        )
    }
}

/// Set shader uniform value vector
pub fn set_shader_value_v<T>(
    shader: &Shader,
    loc_index: i32,
    value: &T,
    uniform_type: i32,
    count: i32,
) {
    unsafe {
        raylib_ffi::SetShaderValueV(
            transmute_copy(shader),
            loc_index,
            value as *const _ as *const c_void,
            uniform_type,
            count,
        )
    }
}

/// Set shader uniform value (matrix 4x4)
pub fn set_shader_value_matrix(shader: &Shader, loc_index: i32, mat: Matrix) {
    unsafe {
        raylib_ffi::SetShaderValueMatrix(transmute_copy(shader), loc_index, mat.raw());
    }
}

/// Set shader uniform value and bind the texture (sampler2d)
pub fn set_shader_value_texture(shader: &Shader, loc_index: i32, texture: Texture2D) {
    unsafe {
        raylib_ffi::SetShaderValueTexture(transmute_copy(shader), loc_index, texture);
    }
}
