#[derive(Debug)]
#[repr(C)]
pub struct Shader {
    pub id: u32,
    pub locs: *mut i32,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { raylib_ffi::UnloadShader(std::mem::transmute_copy(self)) }
    }
}
