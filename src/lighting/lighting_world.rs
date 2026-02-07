use crate::physics::coordinates::Coordinates;

/// Unified trait for the engine to interact with game's world data.
pub trait LightingWorld {
    fn is_transparent(&self, cords: Coordinates) -> bool;
    fn get_light(&self, cords: Coordinates) -> [u8; 3];
    fn set_light(&mut self, cords: Coordinates, color: [u8; 3]);
}