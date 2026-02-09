use nalgebra_glm as glm;
use crate::graphics::texture::texture::Texture;

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