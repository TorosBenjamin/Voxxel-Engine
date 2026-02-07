use gl::types::GLenum;

// Re-export GL data type constants so downstream crates don't need the `gl` crate.
pub const FLOAT: GLenum = gl::FLOAT;
pub const UNSIGNED_INT: GLenum = gl::UNSIGNED_INT;
pub const INT: GLenum = gl::INT;
pub const UNSIGNED_BYTE: GLenum = gl::UNSIGNED_BYTE;

/// Describes one vertex attribute (location, component count, type, offset).
pub struct VertexAttribute {
    pub location: u32,
    pub size: i32,
    pub gl_type: GLenum,
    pub normalized: bool,
    /// When true, uses `glVertexAttribIPointer` (integer, no float conversion).
    pub is_integer: bool,
    pub offset: usize,
}

/// The complete layout of a vertex type (stride and attributes).
pub struct VertexLayout {
    /// Size of one vertex in bytes.
    pub stride: usize,
    /// The attribute descriptors for this vertex layout.
    pub attributes: &'static [VertexAttribute],
}

/// Implemented by vertex types to describe their memory layout for OpenGL.
pub trait Vertex: Sized {
    /// Returns the vertex layout used to configure VAO attributes.
    fn layout() -> VertexLayout;
}

/// Vertex with 3D position and 2D texture coordinates.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VertexPosUv {
    /// XYZ position.
    pub position: [f32; 3],
    /// UV texture coordinates.
    pub uv: [f32; 2],
}

impl Vertex for VertexPosUv {
    fn layout() -> VertexLayout {
        VertexLayout {
            stride: size_of::<Self>(),
            attributes: &[
                VertexAttribute {
                    location: 0,
                    size: 3,
                    gl_type: gl::FLOAT,
                    normalized: false,
                    is_integer: false,
                    offset: 0,
                },
                VertexAttribute {
                    location: 1,
                    size: 2,
                    gl_type: gl::FLOAT,
                    normalized: false,
                    is_integer: false,
                    offset: 12,
                },
            ],
        }
    }
}

/// Vertex with 3D position, normal, and 2D texture coordinates.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VertexPosNormalUv {
    /// XYZ position.
    pub position: [f32; 3],
    /// Surface normal vector.
    pub normal: [f32; 3],
    /// UV texture coordinates.
    pub uv: [f32; 2],
}

impl Vertex for VertexPosNormalUv {
    fn layout() -> VertexLayout {
        VertexLayout {
            stride: size_of::<Self>(),
            attributes: &[
                VertexAttribute {
                    location: 0,
                    size: 3,
                    gl_type: gl::FLOAT,
                    normalized: false,
                    is_integer: false,
                    offset: 0,
                },
                VertexAttribute {
                    location: 1,
                    size: 3,
                    gl_type: gl::FLOAT,
                    normalized: false,
                    is_integer: false,
                    offset: 12,
                },
                VertexAttribute {
                    location: 2,
                    size: 2,
                    gl_type: gl::FLOAT,
                    normalized: false,
                    is_integer: false,
                    offset: 24,
                },
            ],
        }
    }
}
