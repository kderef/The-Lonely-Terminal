use raylib::prelude::*;

#[macro_use]
mod rl_util;
mod game;
mod rl_port;

use game::Game;
use rl_port::*;

fn handler(state: &mut Game, key: Option<KeyboardKey>) {
    if let Some(key) = key {
        match key {
            KeyboardKey::KEY_F11 => state.rl.toggle_fullscreen(),
            KeyboardKey::KEY_A => {
                let beep = load_sound!("../audio/T_retro_beep.wav", ".wav");
                state.rl_audio.play_sound_multi(&beep);
            }
            _ => {}
        }
    }

    let mut dr = state.rl.begin_drawing(&state.thread);

    dr.clear_background(Color::BLACK);

    dr.draw_text_ex(
        &state.fonts.termplus,
        &format!(
            "{}x{} @ {}FPS",
            dr.get_screen_width(),
            dr.get_screen_height(),
            dr.get_fps()
        ),
        vec2![0.0, 0.0],
        20.0,
        1.0,
        Color::WHITE,
    );
}

fn main() {
    let mut game = Game::new();
    game.run(handler);
}
