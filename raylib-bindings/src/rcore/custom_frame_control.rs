//! Custom frame control functions
//! NOTE: Those functions are intended for advanced users that want full control over the frame processing
//! By default EndDrawing() does this job: draws everything + SwapScreenBuffer() + manage frame timing + PollInputEvents()
//! To avoid that behaviour and control frame processes manually, enable in config.h: SUPPORT_CUSTOM_FRAME_CONTROL

/// Swap back buffer with front buffer (screen drawing)
pub fn swap_screen_buffer() {
    unsafe { raylib_ffi::SwapScreenBuffer() }
}

/// Register all input events
pub fn poll_input_events() {
    unsafe { raylib_ffi::PollInputEvents() }
}

/// Wait for some time (halt program execution)
pub fn wait_time(seconds: f64) {
    unsafe { raylib_ffi::WaitTime(seconds) }
}
