use crate::core::handle::Handle;
use crate::graphics::shader::Shader;
use crate::graphics::texture::texture::Texture;
use crate::graphics::texture::texture_3d::Texture3D;
use crate::graphics::texture::texture_array::TextureArray;

/// Specifies which texture type is bound to a material slot.
pub enum TextureBinding {
    /// A single 2D texture.
    Texture2D(Handle<Texture>),
    /// A 2D texture array (used for voxel block faces).
    Array(Handle<TextureArray>),
    /// A 3D texture (used for lightmaps).
    Texture3D(Handle<Texture3D>),
}

/// A texture binding assigned to a numbered slot with a shader uniform name.
pub struct TextureSlot {
    /// GL texture unit index (0, 1, 2, ...).
    pub slot: u32,
    /// Sampler uniform name in the shader.
    pub uniform_name: &'static str,
    /// The texture or texture array bound to this slot.
    pub binding: TextureBinding,
}

/// A shader program paired with its texture bindings.
pub struct Material {
    /// Handle to the shader program.
    pub shader: Handle<Shader>,
    /// Texture slots bound when this material is active.
    pub textures: Vec<TextureSlot>,
}

impl Material {
    /// Creates a material with the given shader and no textures.
    pub fn new(shader: Handle<Shader>) -> Self {
        Self {
            shader,
            textures: Vec::new(),
        }
    }

    /// Adds a texture binding to the material (builder pattern).
    pub fn with_texture(mut self, slot: u32, uniform_name: &'static str, binding: TextureBinding) -> Self {
        self.textures.push(TextureSlot { slot, uniform_name, binding });
        self
    }
}
