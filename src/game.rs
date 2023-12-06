use raylib::prelude::*;

pub const WINDOW_SIZE: (i32, i32) = (
    960,
    840
);
pub const WINDOW_TITLE: &str = "Hello World!";

pub struct Game {
    rl: RaylibHandle,
    thread: RaylibThread
}

impl Game {
    pub fn from(rl: RaylibHandle, th: RaylibThread) -> Self {
        Self {rl, thread: th}
    }
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(WINDOW_SIZE.0, WINDOW_SIZE.1)
            .title(WINDOW_TITLE)
            .build();
        
        Self {rl, thread}
    }
}