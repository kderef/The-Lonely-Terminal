use raylib::{
    ffi::{
        rlDisableBackfaceCulling, rlDisableDepthMask, rlEnableBackfaceCulling, rlEnableDepthMask,
    },
    prelude::*,
};

pub struct Skybox {
    model: Model,
}

impl Skybox {
    pub fn new(rl: &mut RaylibHandle, thr: &RaylibThread, image: &Image) -> Self {
        let cube = unsafe { Mesh::gen_mesh_cube(&thr, 1.0, 1.0, 1.0).make_weak() };
        let mut model = rl.load_model_from_mesh(&thr, cube).unwrap();

        // --- skybox shader
        let mut skybox_shader =
            rl.load_shader(&thr, Some("shader/skybox.vs"), Some("shader/skybox.fs"));

        skybox_shader.set_shader_value(
            skybox_shader.get_shader_location("environmentMap"),
            &[MaterialMapIndex::MATERIAL_MAP_CUBEMAP as i32][..],
        );
        skybox_shader.set_shader_value(skybox_shader.get_shader_location("doGamma"), &[0][..]);
        skybox_shader.set_shader_value(skybox_shader.get_shader_location("vflipped"), &[0][..]);

        let skybox_shader = skybox_shader.to_raw();

        model.materials_mut()[0].shader = skybox_shader;

        // --- cubemap texture
        let cubemap_texture = rl
            .load_texture_cubemap(&thr, image, CubemapLayout::CUBEMAP_LAYOUT_AUTO_DETECT)
            .unwrap()
            .to_raw();

        dbg!(&cubemap_texture);

        model.materials_mut()[0].maps_mut()[MaterialMapIndex::MATERIAL_MAP_CUBEMAP as usize]
            .texture = cubemap_texture;

        Self { model }
    }

    pub fn draw(&self, rl: &mut impl RaylibDraw3D) {
        unsafe {
            rlDisableBackfaceCulling();
            rlDisableDepthMask();
        }
        rl.draw_model(&self.model, Vector3::zero(), 1.0, Color::WHITE);
        unsafe {
            rlEnableBackfaceCulling();
            rlEnableDepthMask();
        }
    }
}

impl Drop for Skybox {
    fn drop(&mut self) {
        unsafe {
            raylib::ffi::UnloadShader(self.model.materials_mut()[0].shader);
            raylib::ffi::UnloadTexture(
                self.model.materials_mut()[0].maps_mut()
                    [MaterialMapIndex::MATERIAL_MAP_CUBEMAP as usize]
                    .texture,
            );
        }
    }
}
