use image::GenericImageView;
use nalgebra_glm as glm;

/// A 2D OpenGL texture.
#[derive(Clone, Copy)]
pub struct Texture {
    pub(crate) id: u32,
    /// Texture width in pixels.
    pub width: u32,
    /// Texture height in pixels.
    pub height: u32,
    pub(crate) target: u32,
}

impl Texture {
    /// Binds this texture to the given texture unit slot.
    pub fn bind(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(self.target, self.id);
        }
    }

    /// Loads an RGBA texture from an image file.
    pub fn from_file(path: &str) -> Self {
        let img = image::open(path)
            .expect("Failed to load texture")
            .flipv();

        let (width, height) = img.dimensions();
        let data = img.to_rgba8();

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }

        Self { id, width, height, target: gl::TEXTURE_2D }
    }

    /// Creates a single-channel (RED) texture from raw pixel bytes.
    pub fn from_bytes(pixels: &[u8], width: u32, height: u32) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            // fontdue gives us 1 byte per pixel (Grayscale).
            // We upload it as RED so the shader can use .r as the alpha/intensity.
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as i32,
                width as i32,
                height as i32,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const _,
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }
        Self { id, width, height, target: gl::TEXTURE_2D }
    }
}

