use crate::{rl_util::FontConfig, vec2, config};
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
    pub fn run(&mut self, rl: &mut RaylibHandle, th: &RaylibThread, fonts: &mut FontConfig) {
        match *self {
            Self::TitleScreen => {
                let mut dr = rl.begin_drawing(&th);
                dr.clear_background(Color::BLACK);
                dr.draw_text_ex(
                    &fonts.title_screen,
                    "F11 - Toggle Fullscreen",
                    vec2![0.0, 0.0],
                    fonts.font_size(&dr),
                    1.0,
                    Color::WHITE,
                );
            },
            Self::Paused => {

            },
            Self::Running => {

            }
        }
    }
}