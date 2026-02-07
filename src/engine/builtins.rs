use crate::core::handle::Handle;
use crate::graphics::font::Font;
use crate::graphics::shader::Shader;

/// Handles to built-in resources auto-registered by the engine at startup.
pub struct BuiltinResources {
    /// The default voxel shader (vertex.glsl + fragment.glsl).
    pub voxel_shader: Handle<Shader>,
    /// The text rendering shader (text_vertex.glsl + text_fragment.glsl).
    pub text_shader: Handle<Shader>,
    /// The UI shader (ui_vertex.glsl + ui_fragment.glsl).
    pub ui_shader: Handle<Shader>,
    /// The wireframe/debug line shader (wireframe_vertex.glsl + wireframe_fragment.glsl).
    pub wireframe_shader: Handle<Shader>,
    /// The default font (Pix32, 24px).
    pub default_font: Handle<Font>,
}
