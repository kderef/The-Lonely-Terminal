pub mod win {
    pub const TITLE: &str = "The Lonely Terminal";
    pub const W: i32 = 1280;
    pub const H: i32 = 800;
}

pub mod font {
    const SEP: char = std::path::MAIN_SEPARATOR;

    use raylib::prelude::*;

    type FontResult = Result<Font, String>;
    type RH = RaylibHandle;
    type RT = RaylibThread;

    pub fn firacode(rl: &mut RH, th: &RT) -> FontResult {
        rl.load_font(&th, &format!(".{SEP}font{SEP}FiraCodeNF.ttf"))
    }
    pub fn termplus(rl: &mut RH, th: &RT) -> FontResult {
        rl.load_font(&th, &format!(".{SEP}font{SEP}TermPlusNF.ttf"))
    }
}

pub mod game_defaults {
    use raylib::{camera::Camera3D, core::math::Vector3};
    use crate::vec3; // macro

    pub const FOV: f32 = 75.0;
    pub const TARGET_FPS: u32 = 120;

    pub fn default_camera() -> Camera3D {
        Camera3D::perspective(
            vec3![0.0, 2.0, 4.0],
            vec3![0.0, 2.0, 0.0],
            vec3![0.0, 1.0, 0.0],
            self::FOV,
        )
    }
}