// no terminal window
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod config;
mod rl_util;

use config::game_defaults;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(config::window::W, config::window::H)
        .title(config::window::TITLE)
        .build();

    // initialization
    let camera = Camera3D::perspective(
        vec3![0.0, 2.0, 4.0],
        vec3![0.0, 2.0, 0.0],
        vec3![0.0, 1.0, 0.0],
        game_defaults::FOV,
    );

    let cam_mode = CameraMode::CAMERA_FIRST_PERSON;

    rl.hide_cursor();
    rl.set_target_fps(game_defaults::TARGET_FPS);

    // main game loop
    while !rl.window_should_close() {
        
    }
}
