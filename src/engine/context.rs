use crate::camera::Camera;
use crate::input::input::Input;

/// Per-frame context passed to the game during the update phase.
pub struct EngineContext<'a> {
    /// Current input state (keyboard and mouse).
    pub input: &'a Input,
    /// Seconds elapsed since the previous frame.
    pub delta_time: f32,
    /// Mutable reference to the engine-owned camera.
    pub camera: &'a mut Camera,
    /// Current window width in pixels.
    pub screen_width: f32,
    /// Current window height in pixels.
    pub screen_height: f32,
}
