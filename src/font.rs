const SEP: char = std::path::MAIN_SEPARATOR;

use raylib::prelude::*;
use std::process::exit;

pub struct FontConfig {
    pub martian: Font,
    pub termplus: Font,
}

type FontResult = Result<Font, String>;
type RH = RaylibHandle;
type RT = RaylibThread;

pub fn firacode(rl: &mut RH, th: &RT) -> FontResult {
    rl.load_font(&th, &format!(".{SEP}font{SEP}FiraCodeNF.ttf"))
}
pub fn termplus(rl: &mut RH, th: &RT) -> FontResult {
    rl.load_font(&th, &format!(".{SEP}font{SEP}TermPlusNF.ttf"))
}
pub fn martian_mono(rl: &mut RH, th: &RT) -> FontResult {
    rl.load_font(&th, &format!(".{SEP}font{SEP}MartianMonoNF.ttf"))
}

impl FontConfig {
    pub fn font_size(&self, dr: &RaylibDrawHandle) -> f32 {
        if dr.is_window_fullscreen() {
            25.0
        } else {
            17.0
        }
    }
    pub fn load_all_or_fail(rl: &mut RH, th: &RT) -> Self {
        let fail = |err: String| {
            if let Err(_) = msgbox::create("ERROR", &err, msgbox::IconType::Error) {
                eprintln!("ERROR: {err}");
            }
            exit(1);
        };

        Self {
            martian: martian_mono(rl, th).unwrap_or_else(fail),
            termplus: termplus(rl, th).unwrap_or_else(fail)
        }
    }
}