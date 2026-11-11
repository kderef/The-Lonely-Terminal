//! Compression/Encoding functionality

use std::ffi::{CStr, CString};
use std::slice;

/// Copy a raylib-allocated buffer into a Rust-owned Vec, then MemFree it.
/// A null pointer (raylib's failure value) gives an empty Vec.
unsafe fn take_raylib_buffer(raw: *mut u8, len: i32) -> Vec<u8> {
    if raw.is_null() {
        return Vec::new();
    }
    let out = unsafe { slice::from_raw_parts(raw, len as usize) }.to_vec();
    unsafe { raylib_ffi::MemFree(raw as *mut _) };
    out
}

/// Compress data (DEFLATE algorithm).
pub fn compress_data(data: &[u8]) -> Vec<u8> {
    let mut out_size = 0;
    unsafe {
        let raw = raylib_ffi::CompressData(data.as_ptr(), data.len() as i32, &mut out_size);
        take_raylib_buffer(raw, out_size)
    }
}

/// Decompress data (DEFLATE algorithm).
pub fn decompress_data(comp_data: &[u8]) -> Vec<u8> {
    let mut out_size = 0;
    unsafe {
        let raw =
            raylib_ffi::DecompressData(comp_data.as_ptr(), comp_data.len() as i32, &mut out_size);
        take_raylib_buffer(raw, out_size)
    }
}

/// Encode data to a Base64 string.
pub fn encode_data_base64(data: &[u8]) -> String {
    let mut out_size = 0;
    unsafe {
        let raw = raylib_ffi::EncodeDataBase64(data.as_ptr(), data.len() as i32, &mut out_size);
        if raw.is_null() {
            return String::new();
        }
        let s = CStr::from_ptr(raw).to_string_lossy().into_owned();
        raylib_ffi::MemFree(raw as *mut _);
        s
    }
}

/// Decode a Base64 string back to bytes.
pub fn decode_data_base64(text: &str) -> Vec<u8> {
    let c_text = CString::new(text).expect("text contains an interior NUL byte");
    let mut out_size = 0;
    unsafe {
        let raw = raylib_ffi::DecodeDataBase64(c_text.as_ptr() as *const u8, &mut out_size);
        take_raylib_buffer(raw, out_size)
    }
}

/// Compute CRC32 hash code.
pub fn compute_crc32(data: &[u8]) -> u32 {
    unsafe { raylib_ffi::ComputeCRC32(data.as_ptr() as *mut u8, data.len() as i32) }
}

/// Compute MD5 hash code (4 words, 16 bytes).
pub fn compute_md5(data: &[u8]) -> [u32; 4] {
    unsafe {
        *(raylib_ffi::ComputeMD5(data.as_ptr() as *mut u8, data.len() as i32) as *const [u32; 4])
    }
}

/// Compute SHA1 hash code (5 words, 20 bytes).
pub fn compute_sha1(data: &[u8]) -> [u32; 5] {
    unsafe {
        *(raylib_ffi::ComputeSHA1(data.as_ptr() as *mut u8, data.len() as i32) as *const [u32; 5])
    }
}
