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

/// An OpenGL 2D texture array for layered textures (e.g. voxel block faces).
pub struct TextureArray {
    pub(crate) id: u32,
    /// Width of each layer in pixels.
    pub width: u32,
    /// Height of each layer in pixels.
    pub height: u32,
    /// Number of layers in the array.
    pub layers: u32,
}

impl TextureArray {
    /// Creates an empty texture array with the given dimensions and layer count.
    pub fn new(width: u32, height: u32, layers: u32) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, id);

            gl::TexImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                gl::RGBA8 as i32,
                width as i32,
                height as i32,
                layers as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
        }
        Self { id, width, height, layers }
    }

    /// Loads an image file into a specific layer.
    pub fn set_layer(&self, layer: u32, path: &str) {
        let img = image::open(path)
            .expect("Failed to load texture for array")
            .flipv();
        let (w, h) = img.dimensions();
        if w != self.width || h != self.height {
            panic!("Texture size mismatch for array layer {}: expected {}x{}, got {}x{}", layer, self.width, self.height, w, h);
        }
        let data = img.to_rgba8();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.id);
            gl::TexSubImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                0, 0, layer as i32,
                self.width as i32,
                self.height as i32,
                1,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
        }
    }

    /// Copies a tile from a texture atlas into a specific layer.
    pub fn set_layer_from_atlas(
        &self,
        layer: u32,
        path: &str,
        tile_size: (u32, u32),
        tile_x: u32,
        tile_y: u32,
    ) {
        if tile_size.0 != self.width || tile_size.1 != self.height {
            panic!(
                "Tile size mismatch for array layer {}: expected {}x{}, got {}x{}",
                layer,
                self.width,
                self.height,
                tile_size.0,
                tile_size.1
            );
        }

        let img = image::open(path).expect("Failed to load texture atlas for array");
        let (atlas_w, atlas_h) = img.dimensions();
        let x = tile_x * tile_size.0;
        let y = tile_y * tile_size.1;

        if x + tile_size.0 > atlas_w || y + tile_size.1 > atlas_h {
            panic!(
                "Atlas tile out of bounds for layer {}: tile ({}, {}), size {}x{}, atlas {}x{}",
                layer,
                tile_x,
                tile_y,
                tile_size.0,
                tile_size.1,
                atlas_w,
                atlas_h
            );
        }

        let tile = img
            .crop_imm(x, y, tile_size.0, tile_size.1)
            .flipv()
            .to_rgba8();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.id);
            gl::TexSubImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                0, 0, layer as i32,
                self.width as i32,
                self.height as i32,
                1,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                tile.as_ptr() as *const _,
            );
        }
    }

    /// Generates mipmaps for the entire texture array.
    pub fn generate_mipmaps(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.id);
            gl::GenerateMipmap(gl::TEXTURE_2D_ARRAY);
        }
    }

    /// Binds this texture array to the given texture unit slot.
    pub fn bind(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.id);
        }
    }
}

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
                gl::RGB8 as i32,
                width as i32,
                height as i32,
                depth as i32,
                0,
                gl::RGB,
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

    /// Re-uploads the full 3D texture data. Each element is an RGB triplet.
    pub fn update(&self, data: &[[u8; 3]]) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_3D, self.id);
            gl::TexSubImage3D(
                gl::TEXTURE_3D,
                0,
                0, 0, 0,
                self.width as i32,
                self.height as i32,
                self.depth as i32,
                gl::RGB,
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

/// A 2D texture subdivided into uniform tiles.
pub struct TextureAtlas {
    /// The underlying texture.
    pub texture: Texture,
    /// Width and height of each tile in pixels.
    pub tile_size: (u32, u32),
    /// Total width and height of the atlas in pixels.
    pub atlas_size: (u32, u32),
}

impl TextureAtlas {
    /// Loads an atlas texture from an image file with the given tile size.
    pub fn from_file(path: &str, tile_size: (u32, u32)) -> Self {
        let tex = Texture::from_file(path);
        let atlas_size = (tex.width, tex.height);

        Self {
            texture: tex,
            tile_size,
            atlas_size,
        }
    }

    /// Binds the atlas texture to the given texture unit slot.
    pub fn bind(&self, slot: u32) {
        self.texture.bind(slot);
    }

    /// Returns the UV rectangle for the tile at grid position `(x, y)`.
    pub fn uv_rect(&self, x: u32, y: u32) -> crate::graphics::uv_rect::UvRect {
        let u0 = (x * self.tile_size.0) as f32 / self.atlas_size.0 as f32;
        let v0 = (y * self.tile_size.1) as f32 / self.atlas_size.1 as f32;
        let u1 = ((x + 1) * self.tile_size.0) as f32 / self.atlas_size.0 as f32;
        let v1 = ((y + 1) * self.tile_size.1) as f32 / self.atlas_size.1 as f32;

        crate::graphics::uv_rect::UvRect {
            min: glm::vec2(u0, v0),
            max: glm::vec2(u1, v1),
        }
    }
}

