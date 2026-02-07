use nalgebra_glm as glm;
use crate::physics::coordinates::Coordinates;

/// World geometry queries for collision detection.
pub trait CollisionMap {
    /// Returns `true` if the block at the given world position is solid.
    fn is_solid_at(&self, x: f32, y: f32, z: f32) -> bool;
    /// Casts a ray from `origin` in `direction` up to `max_dist` and returns the first hit.
    fn raycast(&self, origin: glm::Vec3, direction: glm::Vec3, max_dist: f32) -> Option<RaycastResult>;
}

/// The result of a successful raycast against the collision map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RaycastResult {
    /// Integer coordinates of the hit block.
    pub block_pos: Coordinates,
    /// Normal of the block face that was hit (useful for block placement).
    pub face_normal: glm::IVec3, // Useful for placing blocks
}