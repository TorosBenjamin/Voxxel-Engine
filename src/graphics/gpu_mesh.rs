use gl::types::*;
use crate::graphics::vertex::Vertex;

// Re-export GL draw mode constants so downstream crates don't need the `gl` crate.
pub const DRAW_TRIANGLES: u32 = gl::TRIANGLES;
pub const DRAW_LINES: u32 = gl::LINES;
pub const DRAW_POINTS: u32 = gl::POINTS;

/// A vertex buffer uploaded to the GPU, ready for drawing.
pub struct GpuMesh {
    vao: GLuint,
    vbo: GLuint,
    vertex_count: i32,
    draw_mode: u32,
}

impl GpuMesh {
    /// Uploads vertices to a new VAO/VBO using the vertex layout from the [`Vertex`] trait.
    pub fn from_vertices<V: Vertex>(vertices: &[V]) -> Self {
        if vertices.is_empty() {
            return Self {
                vao: 0,
                vbo: 0,
                vertex_count: 0,
                draw_mode: gl::TRIANGLES,
            };
        }

        let layout = V::layout();

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * layout.stride) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            for attr in layout.attributes {
                gl::EnableVertexAttribArray(attr.location);
                if attr.is_integer {
                    gl::VertexAttribIPointer(
                        attr.location,
                        attr.size,
                        attr.gl_type,
                        layout.stride as i32,
                        attr.offset as *const _,
                    );
                } else {
                    gl::VertexAttribPointer(
                        attr.location,
                        attr.size,
                        attr.gl_type,
                        attr.normalized as u8,
                        layout.stride as i32,
                        attr.offset as *const _,
                    );
                }
            }

            gl::BindVertexArray(0);
        }

        Self {
            vao,
            vbo,
            vertex_count: vertices.len() as i32,
            draw_mode: gl::TRIANGLES,
        }
    }

    /// Sets the OpenGL draw mode (e.g. `gl::LINES`, `gl::TRIANGLES`).
    pub fn with_draw_mode(mut self, mode: u32) -> Self {
        self.draw_mode = mode;
        self
    }

    /// Re-uploads vertex data to the existing VBO, replacing the previous contents.
    pub fn update_vertices<V: Vertex>(&mut self, vertices: &[V]) {
        self.vertex_count = vertices.len() as i32;
        if vertices.is_empty() {
            return;
        }
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * V::layout().stride) as isize,
                vertices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    /// Issues a `glDrawArrays` call for this mesh.
    pub fn draw(&self) {
        if self.vertex_count == 0 {
            return;
        }

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(self.draw_mode, 0, self.vertex_count);
        }
    }
}

// Un allocate mesh from gpu memory
impl Drop for GpuMesh {
    fn drop(&mut self) {
        unsafe {
            if self.vbo != 0 {
                gl::DeleteBuffers(1, &self.vbo);
            }
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &self.vao);
            }
        }
    }
}
