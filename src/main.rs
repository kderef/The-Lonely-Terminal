// no terminal window
//#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

mod config;
mod rl_util;
mod game;

use std::process::exit;

use config::{game_defaults, win, font};
use raylib::prelude::*;
use game::*;
use rl_util::*;

fn handle_keys(rl: &mut RaylibHandle, key: Option<KeyboardKey>, state: &mut GameState) {
    if let Some(key) = key {
        match key {
            KeyboardKey::KEY_F11 => {
                rl.toggle_fullscreen();
            },
            KeyboardKey::KEY_ESCAPE => {
                *state = if *state == GameState::Paused {
                    GameState::Running
                } else {
                    GameState::Paused
                };
            }
            _ => {}
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(win::W, win::H)
        .title(win::TITLE)
        .msaa_4x()
        .build();

    let monitor = get_monitor_info( get_current_monitor() );
    if let Ok(monitor_info) = monitor {
        rl.set_window_size(monitor_info.width, monitor_info.height);
    }
    rl.toggle_fullscreen();

    // initialization
    let mut camera = game_defaults::default_camera();
    let cam_mode = CameraMode::CAMERA_FIRST_PERSON;

    println!("{:?}", rl.get_window_state());
    rl.set_target_fps(game_defaults::TARGET_FPS);
    rl.set_exit_key(None);
    rl.set_mouse_scale(0.0, 0.0);

    let font = rl.load_font(&thread, font::FIRACODE).unwrap_or_else(|err| {
        if let Err(_) = msgbox::create("ERROR", &err, msgbox::IconType::Error) {
            eprintln!("ERROR: {err}");
        }
        exit(1);
    });
    
    let mut game_state = GameState::TitleScreen;

    while !rl.window_should_close() {
        let key = rl.get_key_pressed();

        handle_keys(&mut rl, key, &mut game_state);

        let mut d = rl.begin_drawing(&thread);

        match game_state {
            GameState::TitleScreen => title_screen(&mut d, &mut game_state, &font),
            GameState::Running => {},
            GameState::Paused => {},
        }
    }
}
