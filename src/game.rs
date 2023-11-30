use crate::{font::FontConfig, vec2, rl_util::fail};
use raylib::prelude::*;

#[derive(PartialEq, Debug)]
pub enum GameState {
    TitleScreen,
    Paused,
    Running,
}

impl GameState {
    pub fn handle_keys(&mut self, rl: &mut RaylibHandle, key: Option<KeyboardKey>) {
        if let Some(key) = key {
            match key {
                KeyboardKey::KEY_F11 => {
                    rl.toggle_fullscreen();
                },
                KeyboardKey::KEY_ESCAPE => {
                    *self = if *self == GameState::Paused {
                        GameState::Running
                    } else {
                        GameState::Paused
                    };
                }
                _ => {}
            }
        }
    }
}

pub fn title_screen(rl: &mut RaylibHandle, th: &RaylibThread, fonts: &mut FontConfig) {
    let mut rl_audio = RaylibAudio::init_audio_device();

    let beep = Sound::load_sound(sound!("T_retro_beep.wav")).map_err(fail).unwrap();
    rl_audio.play_sound_multi(&beep);

    let mut spacing = 0;
    let mut dr = rl.begin_drawing(&th);
    
    let echo = |dr: &mut RaylibDrawHandle, msg: &str| {
        dr.draw_text_ex(
            &fonts.termplus,
            msg,
            vec2![0.0, 0.0],
            fonts.font_size(&dr),
            1.0,
            Color::WHITE,
        );
    };
    
    dr.clear_background(Color::BLACK);

    // 1. F11 help
    echo(&mut dr, "Hello World!");

    // 2. Logo text

}
pub fn game_run(rl: &mut RaylibHandle, th: &RaylibThread, fonts: &mut FontConfig) {

}
pub fn pause_screen(rl: &mut RaylibHandle, th: &RaylibThread, fonts: &mut FontConfig) {

}