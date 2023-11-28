// no terminal window
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod config;
mod rl_util;

use config::{game_defaults, win};

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(win::W, win::H)
        .title(win::TITLE)
        .build();

    // initialization
    let camera = game_defaults::default_camera();
    let cam_mode = CameraMode::CAMERA_FIRST_PERSON;

    rl.hide_cursor();
    rl.set_target_fps(game_defaults::TARGET_FPS);

    // main game loop
    while !rl.window_should_close() {
        
    }
}
