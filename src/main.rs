#![cfg_attr(
    any(not(debug_assertions), feature = "nocon"),
    windows_subsystem = "windows"
)]

mod mesh_ex;
mod player_camera;

use macroquad::{
    miniquad::{
        conf::Platform,
        gl::{GL_CULL_FACE, glDepthMask, glDisable, glEnable},
    },
    prelude::*,
};

use crate::player_camera::PlayerCamera;

fn conf() -> Conf {
    Conf {
        window_title: "The Lonely Terminal".to_string(),
        window_width: 1920,
        window_height: 1080,
        window_resizable: false,
        high_dpi: true,
        fullscreen: false,
        sample_count: 4,
        icon: None,
        platform: Platform::default(),
    }
}

fn draw_skybox() {
    // temp disable backface culling & depth mask
    unsafe {
        glDisable(GL_CULL_FACE);
        glDepthMask(0);
    }

    unsafe {
        glEnable(GL_CULL_FACE);
        glDepthMask(1);
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut camera = PlayerCamera::new(vec3(1., 10., 1.));

    let skybox_texture = load_texture("textures/skybox_sky.png").await.unwrap();
    let mut cube = mesh_ex::gen_cube(1.0, 1.0, 1.0);
    cube.texture = Some(skybox_texture);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            camera.toggle_grab();
        }

        let delta = get_frame_time();
        camera.update(delta);

        clear_background(BLACK);

        camera.apply();
        // set_camera(&Camera3D {
        //     position: vec3(1.0, 1.0, 1.0),
        //     target: vec3(5.0, 0.0, 5.0),
        //     ..Default::default()
        // });
        {
            draw_grid(100, 10.0, WHITE, GRAY);
            draw_cube(vec3(0., 1., -5.), vec3(2., 2., 2.), None, RED); // big red cube ahead
            draw_mesh(&cube);
        }
        set_default_camera();

        draw_fps();
        next_frame().await;
    }
}
