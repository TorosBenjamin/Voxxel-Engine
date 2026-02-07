use nalgebra_glm as glm;

/// A rectangular region in UV texture space.
#[derive(Debug, Clone, Copy)]
pub struct UvRect {
    /// Bottom-left UV coordinate.
    pub min: glm::Vec2,
    /// Top-right UV coordinate.
    pub max: glm::Vec2,
}

impl UvRect {
    /// Returns a UV rectangle covering the entire texture (0,0 to 1,1).
    pub fn full() -> Self {
        Self {
            min: glm::vec2(0.0, 0.0),
            max: glm::vec2(1.0, 1.0),
        }
    }
}