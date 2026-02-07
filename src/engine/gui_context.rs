use nalgebra_glm as glm;
use crate::graphics::gui_material::GuiInstance;
use crate::graphics::gpu_mesh::GpuMesh;
use crate::graphics::font::Font;
use crate::graphics::shader::Shader;

/// Immediate-mode GUI rendering context with an orthographic projection.
pub struct GuiContext {
    /// Screen width in pixels.
    pub width: f32,
    /// Screen height in pixels.
    pub height: f32,
    /// Orthographic projection matrix for Y-down UI coordinates.
    pub projection: glm::Mat4,
}

impl GuiContext {
    /// Creates a new GUI context for the given screen dimensions.
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            projection: glm::ortho(
                0.0, width,
                height, 0.0, // Y-down UI coordinates
                -1.0, 1.0,
            ),
        }
    }
}

impl GuiContext {
    /// Draws a mesh using a GUI material and model transform.
    pub fn draw(
        &self,
        mesh: &GpuMesh,
        instance: &GuiInstance,
        model: &glm::Mat4,
    ) {
        instance.material.shader.use_program();
        instance.material.texture.bind(0);

        let shader = &instance.material.shader;
        shader.set_int("uTexture", 0);
        shader.set_mat4("projection", &self.projection);
        shader.set_mat4("model", model);

        mesh.draw();
    }

    /// Draws text using a font atlas, shader, model transform, and color.
    pub fn draw_text(
        &self,
        mesh: &GpuMesh,
        font: &Font,
        shader: &Shader,
        model: &glm::Mat4,
        color: &glm::Vec4,
    ) {
        shader.use_program();
        font.texture.bind(0);

        shader.set_int("uTexture", 0);
        shader.set_mat4("projection", &self.projection);
        shader.set_mat4("model", model);
        shader.set_vec4("uColor", color);

        mesh.draw();
    }
}

