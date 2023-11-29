use raylib::prelude::*;
use crate::vec2;

#[derive(PartialEq)]
pub enum GameState {
    TitleScreen,
    Paused,
    Running,
}

pub fn title_screen(dr: &mut RaylibDrawHandle, state: &mut GameState, font: &Font) {
    dr.clear_background(Color::BLACK);
    dr.draw_text_ex(font, "F11 - toggle Fullscreen", vec2![100.0, 100.0], 25.0, 1.0, Color::WHITE);
}