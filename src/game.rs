use raylib::prelude::*;

pub enum GameState {
    TitleScreen,
    Paused,
    Running,
}

pub fn title_screen(dr: &mut RaylibDrawHandle, state: &mut GameState) {
    dr.clear_background(Color::BLACK);
    dr.draw_text("F11 - toggle Fullscreen", 100, 100, 10, Color::WHITE);
}