use image::GenericImageView;

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