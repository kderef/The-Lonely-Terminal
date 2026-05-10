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

        // Skybox shader
        let mut skybox_shader =
            rl.load_shader(thr, Some("shader/skybox.vs"), Some("shader/skybox.fs"));
        skybox_shader.set_shader_value(
            skybox_shader.get_shader_location("environmentMap"),
            &[MaterialMapIndex::MATERIAL_MAP_CUBEMAP as i32][..],
        );
        skybox_shader.set_shader_value(skybox_shader.get_shader_location("doGamma"), &[1][..]);
        skybox_shader.set_shader_value(skybox_shader.get_shader_location("vflipped"), &[0][..]);

        // Cubemap conversion shader
        let mut cubemap_shader =
            rl.load_shader(thr, Some("shader/cubemap.vs"), Some("shader/cubemap.fs"));
        cubemap_shader.set_shader_value(
            cubemap_shader.get_shader_location("equirectangularMap"),
            &[0][..],
        );

        // Load panorama as regular texture then convert to cubemap
        let panorama = rl.load_texture_from_image(thr, image).unwrap();
        let cubemap = unsafe { gen_texture_cubemap(&cubemap_shader, &panorama, 1024) };

        // cubemap_shader no longer needed
        drop(cubemap_shader);

        let skybox_shader = unsafe { skybox_shader.unwrap() };
        model.materials_mut()[0].shader = skybox_shader;
        model.materials_mut()[0].maps_mut()[MaterialMapIndex::MATERIAL_MAP_CUBEMAP as usize]
            .texture = cubemap;

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

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn gen_texture_cubemap(
    shader: &Shader,
    panorama: &Texture2D,
    size: i32,
) -> ffi::TextureCubemap {
    use raylib::ffi::*;
    use rlFramebufferAttachTextureType::*;
    use rlFramebufferAttachType::*;
    use std::ptr;

    let mut cubemap: TextureCubemap = unsafe { std::mem::zeroed() };

    rlDisableBackfaceCulling();

    // STEP 1: Setup framebuffer
    let rbo = rlLoadTextureDepth(size, size, true);
    cubemap.id = rlLoadTextureCubemap(
        ptr::null(),
        size,
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32,
        1,
    );

    let fbo = rlLoadFramebuffer();
    rlFramebufferAttach(
        fbo,
        rbo,
        RL_ATTACHMENT_DEPTH as i32,
        RL_ATTACHMENT_RENDERBUFFER as i32,
        0,
    );
    rlFramebufferAttach(
        fbo,
        cubemap.id,
        RL_ATTACHMENT_COLOR_CHANNEL0 as i32,
        RL_ATTACHMENT_CUBEMAP_POSITIVE_X as i32,
        0,
    );

    if rlFramebufferComplete(fbo) {
        TraceLog(
            TraceLogLevel::LOG_INFO as i32,
            "FBO: [ID %u] Framebuffer object created successfully\0".as_ptr() as *const i8,
            fbo,
        );
    }

    // STEP 2: Draw to framebuffer
    rlEnableShader(shader.id);

    let mat_projection = Matrix::perspective(
        90.0 * DEG2RAD as f32,
        1.0,
        rlGetCullDistanceNear() as f32,
        rlGetCullDistanceFar() as f32,
    );
    rlSetUniformMatrix(
        shader.locs()[ShaderLocationIndex::SHADER_LOC_MATRIX_PROJECTION as usize],
        mat_projection.into(),
    );

    type Matrix = raylib::math::Matrix;
    type Vector3 = raylib::math::Vector3;

    const fn vec3(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    let fbo_views = [
        Matrix::look_at(vec3(0., 0., 0.), vec3(1., 0., 0.), vec3(0., -1., 0.)),
        Matrix::look_at(vec3(0., 0., 0.), vec3(-1., 0., 0.), vec3(0., -1., 0.)),
        Matrix::look_at(vec3(0., 0., 0.), vec3(0., 1., 0.), vec3(0., 0., 1.)),
        Matrix::look_at(vec3(0., 0., 0.), vec3(0., -1., 0.), vec3(0., 0., -1.)),
        Matrix::look_at(vec3(0., 0., 0.), vec3(0., 0., 1.), vec3(0., -1., 0.)),
        Matrix::look_at(vec3(0., 0., 0.), vec3(0., 0., -1.), vec3(0., -1., 0.)),
    ];

    rlViewport(0, 0, size, size);
    rlActiveTextureSlot(0);
    rlEnableTexture(panorama.id);

    for i in 0..6 {
        rlSetUniformMatrix(
            shader.locs()[ShaderLocationIndex::SHADER_LOC_MATRIX_VIEW as usize],
            fbo_views[i].into(),
        );
        rlFramebufferAttach(
            fbo,
            cubemap.id,
            RL_ATTACHMENT_COLOR_CHANNEL0 as i32,
            RL_ATTACHMENT_CUBEMAP_POSITIVE_X as i32 + i as i32,
            0,
        );
        rlEnableFramebuffer(fbo);
        rlClearScreenBuffers();
        rlLoadDrawCube();
    }

    // STEP 3: Cleanup
    rlDisableShader();
    rlDisableTexture();
    rlDisableFramebuffer();
    rlUnloadFramebuffer(fbo);

    rlViewport(0, 0, rlGetFramebufferWidth(), rlGetFramebufferHeight());
    rlEnableBackfaceCulling();

    cubemap.width = size;
    cubemap.height = size;
    cubemap.mipmaps = 1;
    cubemap.format = PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32;

    cubemap
}
