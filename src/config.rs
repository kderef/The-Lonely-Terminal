pub mod win {
    pub const TITLE: &str = "The Lonely Terminal";
    pub const W: i32 = 1920;
    pub const H: i32 = 1080;
}

pub mod font {
    #[cfg(target_os = "windows")]
    pub const FIRACODE: &str = ".\\font\\FiraCodeNerdFont-Regular.ttf";

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub const FIRACODE: &str = "./font/FiraCodeNerdFont-Regular.ttf";
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