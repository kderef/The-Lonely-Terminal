use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: "The Lonely Terminal".to_string(),
        window_width: 1920 / 2,
        window_height: 1080 / 2,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    println!("Hello, world!");

    loop {
        draw_fps();
        next_frame().await;
    }
}
