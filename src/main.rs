// no terminal window
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod config;
mod rl_util;
mod game;

use config::{game_defaults, win};
use raylib::prelude::*;
use game::*;

fn handle_keys(rl: &mut RaylibHandle, key: Option<KeyboardKey>, state: &mut GameState) {
    if let Some(key) = key {
        match key {
            KeyboardKey::KEY_F11 => {
                rl.toggle_fullscreen();
            },
            KeyboardKey::KEY_ESCAPE => {
                *state = GameState::Paused;
            }
            _ => todo!()
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(win::W, win::H)
        .title(win::TITLE)
        .build();

    // initialization
    let mut camera = game_defaults::default_camera();
    let cam_mode = CameraMode::CAMERA_FIRST_PERSON;

    rl.hide_cursor();
    rl.set_target_fps(game_defaults::TARGET_FPS);
    rl.set_exit_key(None);
    rl.load_font_from_image(_, image, key, first_char)
    
    let mut game_state = GameState::TitleScreen;

    while !rl.window_should_close() {
        let key = rl.get_key_pressed();

        rl.update_camera(&mut camera);
        handle_keys(&mut rl, key, &mut game_state);

        let mut d = rl.begin_drawing(&thread);

        match game_state {
            GameState::TitleScreen => title_screen(&mut d, &mut game_state),
            GameState::Running => {},
            GameState::Paused => {},
        }
    }
}
