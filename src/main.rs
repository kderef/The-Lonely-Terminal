#![cfg_attr(not(debug_assertions), windows_subsytem = "windows")]

use raylib_bindings::window::init_window;

fn main() {
    let window = init_window(600, 400, "Hello world!");

    println!("Hello, world!");
}
