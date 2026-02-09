use crate::physics::coordinates::Coordinates;

/// Unified trait for the engine to interact with game's world data.
pub trait LightingWorld {
    /// Returns the opacity of the block at the given coordinates.
    /// 0 = fully transparent (air, glass), 255 = fully opaque (stone, dirt).
    fn get_opacity(&self, cords: Coordinates) -> u8;
    fn get_light(&self, cords: Coordinates) -> [u8; 3];
    fn set_light(&mut self, cords: Coordinates, color: [u8; 3]);
}