use raylib::{
    ffi::{
        self, UnloadShader, UnloadTexture, rlDisableBackfaceCulling, rlDisableDepthMask,
        rlEnableBackfaceCulling, rlEnableDepthMask,
    },
    prelude::*,
};

pub struct Skybox {
    model: Model,
}

impl Skybox {
    pub fn new(rl: &mut RaylibHandle, thr: &RaylibThread, image: &Image) -> Self {
        let cube = unsafe { Mesh::gen_mesh_cube(thr, 1.0, 1.0, 1.0).make_weak() };
        let mut model = rl.load_model_from_mesh(thr, cube).unwrap();

        // Load skybox shader into model material slot via raw FFI
        model.materials_mut()[0].shader = unsafe {
            rl.load_shader(thr, Some("shader/skybox.vs"), Some("shader/skybox.fs"))
                .unwrap()
        };

        // Set uniforms via raw FFI
        unsafe {
            let shader = model.materials()[0].shader;
            let cubemap_slot = MaterialMapIndex::MATERIAL_MAP_CUBEMAP as i32;
            let zero = 0i32;
            let uniform_int = ShaderUniformDataType::SHADER_UNIFORM_INT as i32;

            ffi::SetShaderValue(
                shader,
                ffi::GetShaderLocation(shader, b"environmentMap\0".as_ptr() as *const i8),
                &cubemap_slot as *const i32 as *const _,
                uniform_int,
            );
            ffi::SetShaderValue(
                shader,
                ffi::GetShaderLocation(shader, b"doGamma\0".as_ptr() as *const i8),
                &zero as *const i32 as *const _,
                uniform_int,
            );
            ffi::SetShaderValue(
                shader,
                ffi::GetShaderLocation(shader, b"vflipped\0".as_ptr() as *const i8),
                &zero as *const i32 as *const _,
                uniform_int,
            );
        }

        // Load cubemap directly from image using auto-detect layout
        let cubemap = rl
            .load_texture_cubemap(&thr, &image, CubemapLayout::CUBEMAP_LAYOUT_AUTO_DETECT)
            .unwrap();

        model.materials_mut()[0].maps_mut()[MaterialMapIndex::MATERIAL_MAP_CUBEMAP as usize]
            .texture = *cubemap;

        Self { model }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw3D) {
        unsafe {
            rlDisableBackfaceCulling();
            rlDisableDepthMask();
        }
        d.draw_model(&self.model, Vector3::zero(), 1.0, Color::WHITE);
        unsafe {
            rlEnableBackfaceCulling();
            rlEnableDepthMask();
        }
    }
}

impl Drop for Skybox {
    fn drop(&mut self) {
        unsafe {
            UnloadShader(self.model.materials_mut()[0].shader);
            UnloadTexture(
                self.model.materials_mut()[0].maps_mut()
                    [MaterialMapIndex::MATERIAL_MAP_CUBEMAP as usize]
                    .texture,
            );
        }
    }
}
