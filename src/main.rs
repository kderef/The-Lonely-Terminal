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

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(win::W, win::H)
        .title(win::TITLE)
        .msaa_4x()
        .build();

    // resize to fit monitor
    if cfg!(target_os = "windows") {
        let monitor = get_monitor_info( get_current_monitor() );
        if let Ok(monitor_info) = monitor {
            println!("{monitor_info:?}");
            rl.set_window_size(monitor_info.width, monitor_info.height);
        }
    }
    rl.toggle_fullscreen();

    rl.set_target_fps(game_defaults::TARGET_FPS);
    rl.set_exit_key(None);
    rl.set_mouse_scale(0.0, 0.0);

    let mut fonts = FontConfig::load_all_or_fail(&mut rl, &thread);
    let mut game_state = GameState::TitleScreen;

    while !rl.window_should_close() {
        let key = rl.get_key_pressed();

        game_state.handle_keys(&mut rl, key);
        match game_state {
            GameState::TitleScreen => title_screen(&mut rl, &thread, &mut fonts),
            GameState::Running => game_run(&mut rl, &thread, &mut fonts),
            GameState::Paused => pause_screen(&mut rl, &thread, &mut fonts)
        }
    }
}
