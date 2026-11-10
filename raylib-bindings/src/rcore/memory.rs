//! Memory management, using internal allocators

use std::ffi::c_void;

/// Internal memory allocator
pub unsafe fn mem_alloc(size: u32) -> *mut c_void {
    unsafe { raylib_ffi::MemAlloc(size) }
}

/// Internal memory reallocator
pub unsafe fn mem_realloc(ptr: *mut c_void, size: u32) -> *mut c_void {
    unsafe { raylib_ffi::MemRealloc(ptr, size) }
}

/// Internal memory free
pub fn mem_free(ptr: *mut c_void) {
    unsafe { raylib_ffi::MemFree(ptr) }
}
