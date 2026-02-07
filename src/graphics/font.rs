use std::collections::HashMap;
use crate::graphics::texture::Texture;
use crate::graphics::uv_rect::UvRect;
use crate::graphics::gpu_mesh::GpuMesh;
use crate::graphics::vertex::VertexPosUv;
use nalgebra_glm as glm;

/// Metrics and UV data for a single rasterized character.
pub struct Glyph {
    /// UV region of this glyph in the font atlas.
    pub uv_rect: UvRect,
    /// Glyph width in pixels.
    pub width: f32,
    /// Glyph height in pixels.
    pub height: f32,
    /// Horizontal advance to the next character in pixels.
    pub advance: f32,
    /// Horizontal bearing offset in pixels.
    pub offset_x: f32,
    /// Vertical bearing offset in pixels.
    pub offset_y: f32,
}

/// A rasterized font atlas with glyph metrics for text rendering.
pub struct Font {
    /// The grayscale atlas texture containing all rasterized glyphs.
    pub texture: Texture,
    /// Per-character glyph metrics keyed by character.
    pub glyphs: HashMap<char, Glyph>,
    /// Vertical spacing between lines in pixels.
    pub line_height: f32,
}

impl Font {
    /// Creates a font from a pre-built texture and glyph map.
    pub fn new_from_texture(texture: Texture, glyphs: HashMap<char, Glyph>, line_height: f32) -> Self {
        Self {
            texture,
            glyphs,
            line_height,
        }
    }

    /// Rasterizes a TTF font file at the given size into an atlas.
    pub fn new_from_ttf(path: &str, size: f32) -> Self {
        let bytes = std::fs::read(path).expect("Failed to read font file");
        Self::from_ttf_bytes(&bytes, size)
    }

    /// Rasterizes a TTF font from in-memory bytes at the given size into an atlas.
    pub fn from_ttf_bytes(bytes: &[u8], size: f32) -> Self {
        let font = fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default()).unwrap();

        let mut glyphs = HashMap::new();
        let chars: Vec<char> = (32..127).map(|i| std::char::from_u32(i).unwrap()).collect();

        // 1. Calculate atlas size with 1px padding to prevent bleeding
        let mut atlas_width = 0;
        let mut atlas_height = 0;
        for &c in &chars {
            let metrics = font.metrics(c, size);
            atlas_width += metrics.width + 1; // 1px padding
            atlas_height = atlas_height.max(metrics.height);
        }

        let mut atlas_pixels = vec![0u8; atlas_width * atlas_height];
        let mut current_x = 0;

        // 2. Rasterize and copy
        for &c in &chars {
            let (metrics, bitmap) = font.rasterize(c, size);

            for y in 0..metrics.height {
                for x in 0..metrics.width {
                    // Correct indexing for a row-major atlas
                    let dest_idx = y * atlas_width + (current_x + x);
                    let src_idx = y * metrics.width + x;
                    if dest_idx < atlas_pixels.len() {
                        atlas_pixels[dest_idx] = bitmap[src_idx];
                    }
                }
            }

            // 3. UV Mapping
            // We use half-pixel offsets (0.5) to ensure we sample the center of the pixel
            let uv_rect = UvRect {
                min: glm::vec2(current_x as f32 / atlas_width as f32, 0.0),
                max: glm::vec2((current_x + metrics.width) as f32 / atlas_width as f32, metrics.height as f32 / atlas_height as f32),
            };

            let offset_y = metrics.height as f32 + metrics.ymin as f32;

            glyphs.insert(c, Glyph {
                uv_rect,
                width: metrics.width as f32,
                height: metrics.height as f32,
                advance: metrics.advance_width,
                offset_x: metrics.xmin as f32,
                // 4. Correct vertical offset for Top-Left coordinate systems
                offset_y,
            });

            current_x += metrics.width + 1;
        }

        let texture = Texture::from_bytes(&atlas_pixels, atlas_width as u32, atlas_height as u32);

        Self {
            texture,
            glyphs,
            line_height: size,
        }
    }

    /// Returns the glyph metrics for a character, or `None` if not rasterized.
    pub fn get_glyph(&self, c: char) -> Option<&Glyph> {
        self.glyphs.get(&c)
    }

    /// Generates a GPU mesh for the given text string.
    pub fn generate_mesh(&self, text: &str) -> GpuMesh {
        GpuMesh::from_vertices(&self.create_vertices(text))
    }

    /// Updates an existing mesh with new text vertices.
    pub fn update_mesh(&self, mesh: &mut GpuMesh, text: &str) {
        mesh.update_vertices(&self.create_vertices(text));
    }

    fn create_vertices(&self, text: &str) -> Vec<VertexPosUv> {
        let mut vertices = Vec::with_capacity(text.len() * 6);
        let mut cursor_x = 0.0;
        let mut cursor_y = 0.0;

        for c in text.chars() {
            if c == '\n' {
                cursor_x = 0.0;
                cursor_y += self.line_height;
                continue;
            }

            if let Some(glyph) = self.get_glyph(c) {
                let x0 = cursor_x + glyph.offset_x;
                let y0 = cursor_y - glyph.offset_y;
                let x1 = x0 + glyph.width;
                let y1 = y0 + glyph.height;

                let u0 = glyph.uv_rect.min.x;
                let v0 = glyph.uv_rect.min.y;
                let u1 = glyph.uv_rect.max.x;
                let v1 = glyph.uv_rect.max.y;

                // Two triangles (6 vertices)
                vertices.extend_from_slice(&[
                    VertexPosUv { position: [x0, y0, 0.0], uv: [u0, v0] },
                    VertexPosUv { position: [x1, y0, 0.0], uv: [u1, v0] },
                    VertexPosUv { position: [x1, y1, 0.0], uv: [u1, v1] },
                    VertexPosUv { position: [x1, y1, 0.0], uv: [u1, v1] },
                    VertexPosUv { position: [x0, y1, 0.0], uv: [u0, v1] },
                    VertexPosUv { position: [x0, y0, 0.0], uv: [u0, v0] },
                ]);

                cursor_x += glyph.advance;
            }
        }
        vertices
    }
}
