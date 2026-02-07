use nalgebra_glm as glm;

/// A normalized plane in 3D space defined by a normal and signed distance.
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    /// Unit normal vector of the plane.
    pub normal: glm::Vec3,
    /// Signed distance from the origin along the normal.
    pub distance: f32,
}

impl Plane {
    /// Creates a plane and normalizes the normal and distance.
    pub fn new(normal: glm::Vec3, distance: f32) -> Self {
        let length = glm::length(&normal);
        Self {
            normal: normal / length,
            distance: distance / length,
        }
    }

    /// Returns the signed distance from a point to this plane.
    pub fn distance_to_point(&self, point: &glm::Vec3) -> f32 {
        glm::dot(&self.normal, point) + self.distance
    }
}

/// Six-plane view frustum for visibility culling.
#[derive(Debug, Clone, Copy)]
pub struct Frustum {
    /// The six frustum planes: left, right, bottom, top, near, far.
    pub planes: [Plane; 6],
}

impl Frustum {
    /// Extracts frustum planes from a view-projection matrix using the Gribb-Hartmann method.
    pub fn from_matrix(mat: &glm::Mat4) -> Self {
        // Gribb-Hartmann extraction for OpenGL (where NDC Z is [-1, 1])
        let left = Plane::new(
            glm::vec3(mat[(3, 0)] + mat[(0, 0)], mat[(3, 1)] + mat[(0, 1)], mat[(3, 2)] + mat[(0, 2)]),
            mat[(3, 3)] + mat[(0, 3)],
        );
        let right = Plane::new(
            glm::vec3(mat[(3, 0)] - mat[(0, 0)], mat[(3, 1)] - mat[(0, 1)], mat[(3, 2)] - mat[(0, 2)]),
            mat[(3, 3)] - mat[(0, 3)],
        );
        let bottom = Plane::new(
            glm::vec3(mat[(3, 0)] + mat[(1, 0)], mat[(3, 1)] + mat[(1, 1)], mat[(3, 2)] + mat[(1, 2)]),
            mat[(3, 3)] + mat[(1, 3)],
        );
        let top = Plane::new(
            glm::vec3(mat[(3, 0)] - mat[(1, 0)], mat[(3, 1)] - mat[(1, 1)], mat[(3, 2)] - mat[(1, 2)]),
            mat[(3, 3)] - mat[(1, 3)],
        );
        let near = Plane::new(
            glm::vec3(mat[(3, 0)] + mat[(2, 0)], mat[(3, 1)] + mat[(2, 1)], mat[(3, 2)] + mat[(2, 2)]),
            mat[(3, 3)] + mat[(2, 3)],
        );
        let far = Plane::new(
            glm::vec3(mat[(3, 0)] - mat[(2, 0)], mat[(3, 1)] - mat[(2, 1)], mat[(3, 2)] - mat[(2, 2)]),
            mat[(3, 3)] - mat[(2, 3)],
        );

        Self {
            planes: [left, right, bottom, top, near, far],
        }
    }

    /// Returns `true` if the axis-aligned bounding box is at least partially inside the frustum.
    pub fn intersects_aabb(&self, min: &glm::Vec3, max: &glm::Vec3) -> bool {
        for plane in &self.planes {
            let mut p = *min;
            if plane.normal.x >= 0.0 { p.x = max.x; }
            if plane.normal.y >= 0.0 { p.y = max.y; }
            if plane.normal.z >= 0.0 { p.z = max.z; }

            if plane.distance_to_point(&p) < 0.0 {
                return false;
            }
        }
        true
    }
}