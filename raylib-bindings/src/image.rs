#[repr(transparent)]
pub struct Image(pub(crate) raylib_ffi::Image);

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            raylib_ffi::UnloadImage(self.0);
        }
    }
}
