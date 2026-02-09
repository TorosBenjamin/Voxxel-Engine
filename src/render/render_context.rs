use nalgebra_glm as glm;
use crate::math::frustum::Frustum;
use crate::render::render_queue::RenderQueue;
use crate::render::render_environment::{RenderEnvironment};

/// Per-frame rendering state holding view/projection matrices, frustum, and render queues.
pub struct RenderContext {
    /// The camera view matrix.
    pub view: glm::Mat4,
    /// The camera projection matrix.
    pub projection: glm::Mat4,
    /// View frustum extracted from the view-projection matrix.
    pub frustum: Frustum,
    /// Queue for opaque geometry (rendered first, no blending).
    pub opaque_queue: RenderQueue,
    /// Queue for transparent geometry (rendered with blending).
    pub transparent_queue: RenderQueue,
    /// Queue for GUI elements (rendered last, no depth test).
    pub gui_queue: RenderQueue,
    pub(crate) gui_projection: glm::Mat4,
    /// Global render variables for the scene
    pub environment: RenderEnvironment,
}

impl RenderContext {
    /// Creates a new render context from view and projection matrices and screen dimensions.
    pub fn new(view: glm::Mat4, projection: glm::Mat4, screen_width: f32, screen_height: f32, environment: RenderEnvironment) -> Self {
        let frustum = Frustum::from_matrix(&(projection * view));
        Self {
            view,
            projection,
            frustum,
            opaque_queue: RenderQueue::new(),
            transparent_queue: RenderQueue::new(),
            gui_queue: RenderQueue::new(),
            gui_projection: glm::ortho(0.0, screen_width, screen_height, 0.0, -1.0, 1.0),
            environment,
        }
    }
}
