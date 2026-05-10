#![cfg_attr(
    any(not(debug_assertions), feature = "nocon"),
    windows_subsystem = "windows"
)]

mod player_camera;
mod skybox;

use player_camera::update_camera;

use raylib::prelude::*;

use crate::skybox::Skybox;

fn main() {
    let (mut rl, thr) = raylib::init()
        .size(1920, 1080)
        .msaa_4x()
        .title("The Lonely Terminal")
        .build();

    rl.set_exit_key(None);
    rl.set_target_fps(200);

    let mut camera = Camera3D::perspective(
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(5.0, 0.0, 5.0),
        Vector3::new(0., 1.0, 0.),
        80.0,
    );

    let mut grabbed = false;

    // skybox
    let image = Image::load_image("textures/sky06_cube.png").unwrap();
    let skybox = Skybox::new(&mut rl, &thr, &image);
    drop(image);

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            player_camera::toggle_grab(&mut rl, &mut grabbed);
        }

        if grabbed {
            update_camera(&rl, &mut camera);
        }

        let mut d = rl.begin_drawing(&thr);

        d.clear_background(Color::BLACK);

        {
            let mut d3d = d.begin_mode3D(camera);

            skybox.draw(&mut d3d);

            d3d.draw_grid(100, 10.0);
        }

        d.draw_fps(0, 0);
    }
    println!("EXITING GAME LOOP");
}
