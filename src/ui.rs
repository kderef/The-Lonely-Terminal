use egui_macroquad::egui::Context;
use egui_macroquad::egui::{self, Widget};
use macroquad::prelude::*;

pub struct UI {
    pub enabled: bool,
    pub toggle_button: KeyCode,
}

pub trait DebugWindow {
    fn title(&self) -> &str {
        "debug window"
    }
    fn render(&mut self, ui: &mut egui::Ui);
}

impl DebugWindow for Camera3D {
    fn title(&self) -> &str {
        "Camera3D"
    }
    fn render(&mut self, ui: &mut egui::Ui) {
        egui::Slider::new(&mut self.fovy, -0.0..=10.0)
            .min_decimals(1)
            .text("fovy")
            .ui(ui);

        ui.horizontal_centered(|ui| {
            if ui.button("reset to default").clicked() {
                *self = Default::default();
            }
        });
    }
}

impl UI {
    pub fn new(toggle_button: KeyCode) -> Self {
        Self {
            enabled: false,
            toggle_button,
        }
    }

    pub fn render(&mut self, windows: &mut [&mut impl DebugWindow]) {
        if !self.enabled {
            return;
        }

        egui_macroquad::ui(|ctx| {
            for window in &mut *windows {
                egui::Window::new(window.title()).show(ctx, |ui| {
                    //
                    window.render(ui);
                });
            }
        });
    }
    pub fn draw(&self) {
        if self.enabled {
            egui_macroquad::draw();
        }
    }
}
