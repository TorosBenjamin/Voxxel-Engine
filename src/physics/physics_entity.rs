use nalgebra_glm as glm;

/// A movable axis-aligned bounding box with velocity.
pub struct PhysicsEntity {
    /// World-space position of the AABB minimum corner.
    pub position: glm::Vec3,
    /// Current velocity in units per second.
    pub velocity: glm::Vec3,
    /// AABB dimensions (width, height, depth).
    pub size: glm::Vec3, // The AABB dimensions
    /// Whether the entity is resting on a surface below it.
    pub is_grounded: bool,
}

/// Implemented by game objects that participate in physics.
pub trait KinematicBody {
    /// Returns a mutable reference to the underlying physics entity.
    fn get_physics(&mut self) -> &mut PhysicsEntity;
}