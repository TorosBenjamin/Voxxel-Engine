use crate::graphics::texture::Texture3D;

/// A point light source placed at a specific voxel position.
pub struct LightSource {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    /// RGB color of the light.
    pub color: [u8; 3],
}

/// CPU-side 3D light data for a chunk, stored as RGB per voxel.
pub struct Lightmap {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    data: Vec<[u8; 3]>,
}

impl Lightmap {
    /// Creates a new lightmap initialized to black (no light).
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        let size = (width * height * depth) as usize;
        Self {
            width,
            height,
            depth,
            data: vec![[0, 0, 0]; size],
        }
    }

    /// Sets the light color at the given voxel position.
    pub fn set(&mut self, x: u32, y: u32, z: u32, color: [u8; 3]) {
        let idx = self.index(x, y, z);
        self.data[idx] = color;
    }

    /// Gets the light color at the given voxel position.
    pub fn get(&self, x: u32, y: u32, z: u32) -> [u8; 3] {
        self.data[self.index(x, y, z)]
    }

    /// Resets all voxels to black.
    pub fn clear(&mut self) {
        self.data.fill([0, 0, 0]);
    }

    /// Creates a new GPU 3D texture from this lightmap data.
    pub fn to_texture_3d(&self) -> Texture3D {
        let tex = Texture3D::new(self.width, self.height, self.depth);
        tex.update(&self.data);
        tex
    }

    /// Re-uploads this lightmap's data to an existing GPU texture.
    pub fn upload_to(&self, texture: &Texture3D) {
        texture.update(&self.data);
    }

    /// Returns the raw data as a flat byte slice (for direct GL upload).
    pub fn as_bytes(&self) -> &[u8] {
        // Safety: [u8; 3] has no padding, so the cast is valid.
        unsafe {
            std::slice::from_raw_parts(
                self.data.as_ptr() as *const u8,
                self.data.len() * 3,
            )
        }
    }

    pub(crate) fn index(&self, x: u32, y: u32, z: u32) -> usize {
        (x + y * self.width + z * self.width * self.height) as usize
    }
}