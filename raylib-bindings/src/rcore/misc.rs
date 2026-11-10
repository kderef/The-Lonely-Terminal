//! Misc. functions

use crate::rcore::window::ConfigFlags;
use std::ffi::CString;

/// Takes a screenshot of current screen (filename extension defines format)
pub fn take_screenshot(file_name: &str) {
    let file_name = CString::new(file_name).expect("screenshot filename should be valid utf8");

    unsafe { raylib_ffi::TakeScreenshot(file_name.as_ptr()) }
}

/// Set init configuration flags (view FLAGS)
pub fn set_config_flags(flags: ConfigFlags) {
    unsafe {
        raylib_ffi::SetConfigFlags(flags.bits());
    }
}

/// Open URL with default system browser (if available)
pub fn open_url(url: &str) {
    let url = CString::new(url).expect("url should be valid utf8");

    unsafe { raylib_ffi::OpenURL(url.as_ptr()) }
}
