/// A 3D OpenGL texture (used for lightmaps).
pub struct Texture3D {
    pub(crate) id: u32,
    /// Texture width in texels.
    pub width: u32,
    /// Texture height in texels.
    pub height: u32,
    /// Texture depth in texels.
    pub depth: u32,
}

impl Texture3D {
    /// Creates an empty 3D texture with RGB8 format and linear filtering.
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_3D, id);

            gl::TexImage3D(
                gl::TEXTURE_3D,
                0,
                gl::RGBA8 as i32,
                width as i32,
                height as i32,
                depth as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
        }
        Self { id, width, height, depth }
    }

    /// Re-uploads the full 3D texture data.
    /// Data should be a flat slice of bytes in RGBA order.
    pub fn update(&self, data: &[u8]) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_3D, self.id);
            gl::TexSubImage3D(
                gl::TEXTURE_3D,
                0,
                0, 0, 0,
                self.width as i32,
                self.height as i32,
                self.depth as i32,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
        }
    }

    /// Binds this 3D texture to the given texture unit slot.
    pub fn bind(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_3D, self.id);
        }
    }
}

impl Drop for Texture3D {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
