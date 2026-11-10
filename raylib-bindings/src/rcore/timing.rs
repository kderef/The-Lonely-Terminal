//! Timing-related functions

/// Set target FPS (maximum)
pub fn set_target_fps(fps: i32) {
    unsafe { raylib_ffi::SetTargetFPS(fps) }
}

/// Get time in seconds for last frame drawn (delta time)
pub fn get_frame_time() -> f32 {
    unsafe { raylib_ffi::GetFrameTime() }
}

/// Get elapsed time in seconds since InitWindow()
pub fn get_time() -> f64 {
    unsafe { raylib_ffi::GetTime() }
}

/// Get current FPS
pub fn get_fps() -> i32 {
    unsafe { raylib_ffi::GetFPS() }
}
