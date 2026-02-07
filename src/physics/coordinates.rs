/// Integer block position in the voxel world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates {
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
    /// Z coordinate.
    pub z: i32,
}

impl Coordinates {
    /// Returns the six face-adjacent block positions.
    pub fn neighbors(&self) -> [Coordinates; 6] {
        [
            Coordinates { x: self.x + 1, y: self.y,     z: self.z     },
            Coordinates { x: self.x - 1, y: self.y,     z: self.z     },
            Coordinates { x: self.x,     y: self.y + 1, z: self.z     },
            Coordinates { x: self.x,     y: self.y - 1, z: self.z     },
            Coordinates { x: self.x,     y: self.y,     z: self.z + 1 },
            Coordinates { x: self.x,     y: self.y,     z: self.z - 1 },
        ]
    }
}