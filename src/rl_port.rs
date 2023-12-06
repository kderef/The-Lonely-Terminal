use std::ffi::CString;

use raylib::prelude::*;
pub fn load_font_from_memory(file_type: &'static str, data: &[u8], font_size: i32) -> Font {
    unsafe {
        let raw = raylib::ffi::LoadFontFromMemory(
            file_type.as_ptr() as *const i8,
            data.as_ptr(),
            data.len() as i32,
            font_size,
            std::ptr::null_mut(),
            0
        );
        Font::from_raw(raw)
    }
}

pub fn set_config_flags(flags: u32) {
    unsafe {
        raylib::ffi::SetConfigFlags(flags);
    }
}

pub fn get_current_monitor() -> i32 {
    unsafe {
        raylib::ffi::GetCurrentMonitor()
    }
}

pub fn get_monitor_refresh_rate(monitor: i32) -> i32 {
    unsafe {
        raylib::ffi::GetMonitorRefreshRate(monitor)
    }
}

pub fn load_sound_from_memory(filetype_: &'static str, data: &[u8]) -> Sound {
    let filetype = CString::new(filetype_).unwrap();
    unsafe {
        let raw_wave = raylib::ffi::LoadWaveFromMemory(
            filetype.as_ptr(),
            data.as_ptr(),
            data.len() as i32
        );

        let raw_sound = raylib::ffi::LoadSoundFromWave(raw_wave);
        Sound::from_raw(raw_sound)
    }
}