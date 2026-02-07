use crate::core::handle::Handle;
use crate::graphics::gpu_mesh::GpuMesh;
use crate::graphics::material::{Material, TextureBinding, TextureSlot};
use nalgebra_glm as glm;

/// A shader uniform value.
pub enum UniformValue {
    Float(f32),
    Int(i32),
    Vec2(glm::Vec2),
    Vec3(glm::Vec3),
    Vec4(glm::Vec4),
    Mat4(glm::Mat4),
}

/// A named shader uniform to set before drawing.
pub struct Uniform {
    /// The uniform variable name in the shader.
    pub name: &'static str,
    /// The value to upload.
    pub value: UniformValue,
}

/// A single draw call submitted to a render queue.
pub struct RenderCommand {
    /// Handle to the GPU mesh to draw.
    pub mesh: Handle<GpuMesh>,
    /// Handle to the material (shader + textures).
    pub material: Handle<Material>,
    /// Model transform matrix.
    pub transform: glm::Mat4,
    /// Additional per-draw uniforms.
    pub uniforms: Vec<Uniform>,
    /// Per-draw texture bindings (e.g. per-chunk lightmaps).
    pub textures: Vec<TextureSlot>,
}

impl RenderCommand {
    /// Creates a render command with no extra uniforms.
    pub fn new(mesh: Handle<GpuMesh>, material: Handle<Material>, transform: glm::Mat4) -> Self {
        Self {
            mesh,
            material,
            transform,
            uniforms: Vec::new(),
            textures: Vec::new(),
        }
    }

    /// Adds a per-draw uniform (builder pattern).
    pub fn with_uniform(mut self, name: &'static str, value: UniformValue) -> Self {
        self.uniforms.push(Uniform { name, value });
        self
    }

    /// Adds a per-draw texture binding (builder pattern).
    pub fn with_texture(mut self, slot: u32, uniform_name: &'static str, binding: TextureBinding) -> Self {
        self.textures.push(TextureSlot { slot, uniform_name, binding });
        self
    }
}
