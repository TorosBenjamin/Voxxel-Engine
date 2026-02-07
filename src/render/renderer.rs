use nalgebra_glm as glm;
use crate::render::render_command::UniformValue;
use crate::render::render_context::RenderContext;
use crate::render::render_queue::RenderQueue;
use crate::resource::resource_manager::ResourceAccess;
use crate::graphics::material::TextureBinding;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, ctx: &mut RenderContext, resources: &impl ResourceAccess) {
        // Opaque pass
        ctx.opaque_queue.sort_by_material();
        self.render_queue(&ctx.opaque_queue, &ctx.view, &ctx.projection, resources);

        // Transparent pass (blend on, depth writes off to avoid transparent-on-transparent occlusion)
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DepthMask(gl::FALSE);
        }
        ctx.transparent_queue.sort_by_material();
        self.render_queue(&ctx.transparent_queue, &ctx.view, &ctx.projection, resources);
        unsafe {
            gl::DepthMask(gl::TRUE);
        }

        // GUI pass (blend already on, disable depth test)
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
        let identity = glm::identity::<f32, 4>();
        ctx.gui_queue.sort_by_material();
        self.render_queue(&ctx.gui_queue, &identity, &ctx.gui_projection, resources);
        // NOTE: Blend stays enabled and depth test stays disabled here.
        // The engine restores GL state after render_ui() so that immediate-mode
        // GUI drawing (crosshair, text) also benefits from alpha blending.
    }

    fn render_queue(
        &self,
        queue: &RenderQueue,
        view: &glm::Mat4,
        projection: &glm::Mat4,
        resources: &impl ResourceAccess,
    ) {
        let mut last_shader_id: u32 = 0;
        let mut last_material_id: u32 = u32::MAX;

        for cmd in queue {
            let material = match resources.get(cmd.material) {
                Some(m) => m,
                None => continue,
            };
            let shader = match resources.get(material.shader) {
                Some(s) => s,
                None => continue,
            };

            // Only rebind shader if it changed
            if shader.id != last_shader_id {
                shader.use_program();
                shader.set_mat4("view", view);
                shader.set_mat4("projection", projection);
                last_shader_id = shader.id;
                // Force material rebind since shader changed
                last_material_id = u32::MAX;
            }

            // Only rebind textures if material changed
            if cmd.material.id != last_material_id {
                for tex_slot in &material.textures {
                    shader.set_int(tex_slot.uniform_name, tex_slot.slot as i32);

                    match &tex_slot.binding {
                        TextureBinding::Texture2D(handle) => {
                            if let Some(tex) = resources.get(*handle) {
                                tex.bind(tex_slot.slot);
                            }
                        }
                        TextureBinding::Array(handle) => {
                            if let Some(arr) = resources.get(*handle) {
                                arr.bind(tex_slot.slot);
                            }
                        }
                        TextureBinding::Texture3D(handle) => {
                            if let Some(tex3d) = resources.get(*handle) {
                                tex3d.bind(tex_slot.slot);
                            }
                        }
                    }
                }
                last_material_id = cmd.material.id;
            }

            // Standard per-draw uniforms
            shader.set_mat4("model", &cmd.transform);

            // Custom per-draw uniforms
            for uniform in &cmd.uniforms {
                match &uniform.value {
                    UniformValue::Float(v) => shader.set_f32(uniform.name, *v),
                    UniformValue::Int(v) => shader.set_int(uniform.name, *v),
                    UniformValue::Vec2(v) => shader.set_vec2(uniform.name, v),
                    UniformValue::Vec3(v) => shader.set_vec3(uniform.name, v),
                    UniformValue::Vec4(v) => shader.set_vec4(uniform.name, v),
                    UniformValue::Mat4(v) => shader.set_mat4(uniform.name, v),
                }
            }

            // Per-draw textures (e.g. per-chunk lightmaps)
            for tex_slot in &cmd.textures {
                shader.set_int(tex_slot.uniform_name, tex_slot.slot as i32);

                match &tex_slot.binding {
                    TextureBinding::Texture2D(handle) => {
                        if let Some(tex) = resources.get(*handle) {
                            tex.bind(tex_slot.slot);
                        }
                    }
                    TextureBinding::Array(handle) => {
                        if let Some(arr) = resources.get(*handle) {
                            arr.bind(tex_slot.slot);
                        }
                    }
                    TextureBinding::Texture3D(handle) => {
                        if let Some(tex3d) = resources.get(*handle) {
                            tex3d.bind(tex_slot.slot);
                        }
                    }
                }
            }

            // Draw
            if let Some(mesh) = resources.get(cmd.mesh) {
                mesh.draw();
            }
        }
    }
}
