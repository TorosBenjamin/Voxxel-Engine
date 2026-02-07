use crate::graphics::shader::Shader;
use crate::graphics::texture::Texture;
use crate::graphics::uv_rect::UvRect;

/// A shader and texture pair for immediate-mode GUI rendering.
pub struct GuiMaterial {
    /// The GUI shader program.
    pub shader: Shader,
    /// The GUI texture.
    pub texture: Texture,
}

/// A reference to a GUI material with a specific UV region to draw.
pub struct GuiInstance<'a> {
    /// The GUI material to render with.
    pub material: &'a GuiMaterial,
    /// UV region of the texture to sample.
    pub uv_rect: UvRect,
}

impl<'a> GuiInstance<'a> {
    /// Creates a new GUI instance from a material and UV rectangle.
    pub fn new(material: &'a GuiMaterial, uv_rect: UvRect) -> Self {
        Self {
            material,
            uv_rect,
        }
    }
}
