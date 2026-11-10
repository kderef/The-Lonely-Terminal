//! Random values generation functions

/// Set the seed for the random number generator
pub fn set_random_seed(seed: u32) {
    unsafe { raylib_ffi::SetRandomSeed(seed) }
}

/// Get a random value between min and max (both included)
pub fn get_random_value(min: i32, max: i32) -> i32 {
    unsafe { raylib_ffi::GetRandomValue(min, max) }
}

/// Load random values sequence, no values repeated
pub unsafe fn load_random_sequence(count: u32, min: i32, max: i32) -> *mut i32 {
    unsafe { raylib_ffi::LoadRandomSequence(count, min, max) }
}

/// Unload random values sequence
pub unsafe fn unload_random_sequence(sequence: *mut i32) {
    unsafe {
        raylib_ffi::UnloadRandomSequence(sequence);
    }
}
