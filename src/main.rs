#![cfg_attr(not(debug_assertions), windows_subsytem = "windows")]

use raylib_bindings::{
    rcore::{
        drawing::{begin_drawing, clear_background, end_drawing},
        window::init_window,
    },
    rtextures::color::Color,
};

fn main() {
    let window = init_window(600, 400, "Hello world!");

    while !window.should_close() {
        begin_drawing();

        clear_background(Color::BLACK);

        end_drawing();
    }

    println!("Hello, world!");
}
