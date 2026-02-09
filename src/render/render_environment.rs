use nalgebra_glm as glm;

/// Global scene render variables
pub struct RenderEnvironment {
    pub sky_color: glm::Vec3,
    pub sky_intensity: f32,
    pub ambient: f32,
}