//! Logging system

use std::ffi::CString;

pub use raylib_ffi::TraceLogCallback;

#[repr(i32)]
#[derive(Debug, Clone, Copy, Default)]
pub enum TraceLogLevel {
    #[default]
    All = 0,
    Trace = 1,
    Debug = 2,
    Info = 3,
    Warning = 4,
    Error = 5,
    Fatal = 6,
    None = 7,
}

/// Set the current threshold (minimum) log level
pub fn set_trace_log_level(log_level: TraceLogLevel) {
    unsafe { raylib_ffi::SetTraceLogLevel(log_level as i32) }
}

/// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...)
pub fn trace_log(log_level: TraceLogLevel, text: &str) {
    let text_c = CString::new(text).expect("text should be valid utf8");
    unsafe { raylib_ffi::TraceLog(log_level as i32, text_c.as_ptr()) }
}

/// Set custom trace log
pub fn set_trace_log_callback(callback: TraceLogCallback) {
    unsafe { raylib_ffi::SetTraceLogCallback(callback) }
}
