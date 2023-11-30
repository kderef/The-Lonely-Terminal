// no terminal window
//#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

mod config;
#[macro_use]
mod rl_util;
mod game;
mod font;

use config::{game_defaults, win};
use raylib::prelude::*;
use game::*;
use font::FontConfig;
use std::ops::Div;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(win::W, win::H)
        .title(win::TITLE)
        .msaa_4x()
        .build();

    let size: (i32, i32);
    // resize to fit monitor
    if cfg!(target_os = "windows") {
        let monitor = get_monitor_info( get_current_monitor() );
        if let Ok(monitor_info) = monitor {
            rl.set_window_size(monitor_info.width, monitor_info.height);
            size = (monitor_info.width, monitor_info.height);
        } else {
            size = (win::W, win::H);
        }
    } else {
        size = (win::W, win::H);
    }
    rl.toggle_fullscreen();

    rl.set_target_fps(game_defaults::TARGET_FPS);
    rl.set_exit_key(None);
    rl.set_mouse_scale(0.0, 0.0);

    let mut fonts = FontConfig::load_all_or_fail(&mut rl, &thread);
    let mut game_state = GameState::TitleScreen;

    let mut game = Game::new(&mut rl, &thread, &fonts);
    game.load_all_sounds_or_fail();

    while game.is_running() {
        let key = game.get_key();
        let fps = game.fps();

        let mut dr = game.rl.begin_drawing(&thread);
        dr.draw_text_ex(
            &fonts.termplus,
            &format!("FPS: {fps}"),
            vec2![0.0, size.1.div(2) as f32],
            20.0, 1.0, Color::GREEN
        );

        drop(dr);

        game.handle_key(key);

        match game_state {
            GameState::TitleScreen => game.show_title_screen(),
            GameState::MainMenu => game.show_main_menu(),
            GameState::Running => game.show_running(),
            GameState::Paused => game.show_pause_screen(),
        }
    }
}
