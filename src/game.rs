use crate::{font::FontConfig, rl_util::fail, vec2};
use raylib::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum GameState {
    TitleScreen,
    MainMenu,
    Paused,
    Running,
}

pub struct Game<'a> {
    pub rl: &'a mut RaylibHandle,
    th: &'a RaylibThread,
    fonts: &'a FontConfig,
    state: GameState,
    rl_audio: RaylibAudio,
    startup_beep_played: bool,
    sounds: HashMap<&'static str, Sound>
}

impl<'a> Game<'a> {
    pub fn new(rl: &'a mut RaylibHandle, th: &'a RaylibThread, fonts: &'a FontConfig) -> Self {
        Self {
            rl,
            th,
            fonts,
            state: GameState::TitleScreen,
            rl_audio: RaylibAudio::init_audio_device(),
            startup_beep_played: false,
            sounds: HashMap::new()
        }
    }

    pub fn load_all_sounds_or_fail(&mut self) {
        let beep = Sound::load_sound(sound!("T_retro_beep.wav")).map_err(fail).unwrap();

        self.sounds.insert("retro_beep", beep);
    }

    pub fn is_running(&self) -> bool {
        !self.rl.window_should_close()
    }

    pub fn get_key(&mut self) -> Option<KeyboardKey> {
        self.rl.get_key_pressed()
    }
    pub fn handle_key(&mut self, key: Option<KeyboardKey>) {
        if let Some(key) = key {
            match key {
                KeyboardKey::KEY_F11 => {
                    self.rl.toggle_fullscreen();
                },
                KeyboardKey::KEY_ESCAPE => {
                    self.state = if self.state == GameState::Paused {
                        GameState::Running
                    } else {
                        GameState::Paused
                    };
                }
                _ => {}
            }
        }
    }
    pub fn fps(&self) -> u32 {
        self.rl.get_fps()
    }

    // game states

    /// `GameState::TitleScreen`
    pub fn show_title_screen(&mut self) {
        if !self.startup_beep_played {
            self.rl_audio.play_sound(&self.sounds.get("retro_beep").unwrap());
            self.startup_beep_played = true;
        }

        let mut spacing = 0;
        let mut dr = self.rl.begin_drawing(self.th);

        let echo = |dr: &mut RaylibDrawHandle, msg: &str| {
            dr.draw_text_ex(
                &self.fonts.termplus,
                msg,
                vec2![0.0, 0.0],
                self.fonts.font_size(&dr),
                1.0,
                Color::WHITE,
            );
        };

        dr.clear_background(Color::BLACK);

        // 1. F11 help
        echo(&mut dr, "Hello World!");

        // 2. Logo text
    }
    /// `GameState::MainMenu`
    pub fn show_main_menu(&mut self) {}
    /// `GameState::Running`
    pub fn show_running(&mut self) {}
    /// `GameState::Paused`
    pub fn show_pause_screen(&mut self) {}
}
