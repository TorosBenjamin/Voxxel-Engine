use crate::graphics::texture::texture_3d::Texture3D;

/// CPU-side 3D light data for a chunk.
/// Stores Block Light (RGB) and Sky Light Accessibility (A) per voxel.
pub struct Lightmap {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    /// R, G, B = Block Light (Torches/Lava)
    /// A = Sky Light Accessibility (0-255)
    data: Vec<[u8; 4]>,
}

impl Lightmap {
    /// Creates a new lightmap initialized to black (no light) and zero sky access.
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        let size = (width * height * depth) as usize;
        Self {
            width,
            height,
            depth,
            // Initialize with [0, 0, 0, 0] (No block light, No sky access)
            data: vec![[0, 0, 0, 0]; size],
        }
    }

    /// Sets the BLOCK light (RGB) only. Preserves the current SKY light (A).
    pub fn set_block_light(&mut self, x: u32, y: u32, z: u32, color: [u8; 3]) {
        let idx = self.index(x, y, z);
        self.data[idx][0] = color[0];
        self.data[idx][1] = color[1];
        self.data[idx][2] = color[2];
    }

    /// Gets the BLOCK light (RGB) only.
    pub fn get_block_light(&self, x: u32, y: u32, z: u32) -> [u8; 3] {
        let val = self.data[self.index(x, y, z)];
        [val[0], val[1], val[2]]
    }

    /// Sets the SKY light accessibility (Alpha) only. Preserves current BLOCK light.
    pub fn set_sky_light(&mut self, x: u32, y: u32, z: u32, intensity: u8) {
        let idx = self.index(x, y, z);
        self.data[idx][3] = intensity;
    }

    /// Gets the SKY light accessibility (Alpha).
    pub fn get_sky_light(&self, x: u32, y: u32, z: u32) -> u8 {
        self.data[self.index(x, y, z)][3]
    }

    /// Helper to get the full RGBA value (useful for mesh generation/debugging).
    pub fn get_raw(&self, x: u32, y: u32, z: u32) -> [u8; 4] {
        self.data[self.index(x, y, z)]
    }

    /// Resets all voxels to zero (black and dark).
    pub fn clear(&mut self) {
        self.data.fill([0, 0, 0, 0]);
    }

    /// Creates a new GPU 3D texture from this lightmap data.
    pub fn to_texture_3d(&self) -> Texture3D {
        // IMPORTANT: Ensure your Texture3D::new implementation accepts RGBA8
        let tex = Texture3D::new(self.width, self.height, self.depth);
        tex.update(self.as_bytes());
        tex
    }

    /// Re-uploads this lightmap's data to an existing GPU texture.
    pub fn upload_to(&self, texture: &Texture3D) {
        texture.update(self.as_bytes());
    }

    /// Returns the raw data as a flat byte slice (for direct GL upload).
    pub fn as_bytes(&self) -> &[u8] {
        // Safety: [u8; 4] has no padding, so the cast is valid.
        unsafe {
            std::slice::from_raw_parts(
                self.data.as_ptr() as *const u8,
                self.data.len() * 4, // 4 bytes per voxel now!
            )
        }
    }

    pub(crate) fn index(&self, x: u32, y: u32, z: u32) -> usize {
        (x + y * self.width + z * self.width * self.height) as usize
    }
}