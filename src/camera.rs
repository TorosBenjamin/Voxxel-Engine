use nalgebra_glm as glm;

/// Camera projection mode.
pub enum Projection {
    /// Perspective projection with field-of-view (radians), near and far clip planes.
    Perspective { fov: f32, near: f32, far: f32 },
    /// Orthographic projection with explicit view bounds and clip planes.
    Orthographic { left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32 },
}

/// First-person camera with configurable projection.
pub struct Camera {
    /// World-space position.
    pub position: glm::Vec3,
    /// Normalized forward direction vector.
    pub front: glm::Vec3,
    /// Up direction vector.
    pub up: glm::Vec3,
    yaw: f32,
    pitch: f32,
    projection: Projection,
}

impl Camera {
    /// Creates a camera at `position` with default perspective projection (45deg FOV, 0.1-100 clip range).
    pub fn new(position: glm::Vec3) -> Self {
        Self {
            position,
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            projection: Projection::Perspective {
                fov: 45.0f32.to_radians(),
                near: 0.1,
                far: 100.0,
            },
        }
    }

    /// Replaces the current projection mode.
    pub fn set_projection(&mut self, projection: Projection) {
        self.projection = projection;
    }

    /// Returns a reference to the current projection.
    pub fn projection(&self) -> &Projection {
        &self.projection
    }

    /// Computes the view matrix from position, front, and up.
    pub fn view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    /// Computes the projection matrix. For perspective, `aspect_ratio` controls width/height.
    /// For orthographic, `aspect_ratio` is ignored.
    pub fn projection_matrix(&self, aspect_ratio: f32) -> glm::Mat4 {
        match &self.projection {
            Projection::Perspective { fov, near, far } => {
                glm::perspective(aspect_ratio, *fov, *near, *far)
            }
            Projection::Orthographic { left, right, bottom, top, near, far } => {
                glm::ortho(*left, *right, *bottom, *top, *near, *far)
            }
        }
    }

    /// Translates the camera by an offset in world space.
    pub fn translate(&mut self, offset: glm::Vec3) {
        self.position += offset;
    }

    /// Sets yaw and pitch (degrees) and recalculates the front vector.
    pub fn set_yaw_and_pitch(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw;
        self.pitch = pitch;
        self.update_front_from_angles()
    }

    /// Returns the current yaw in degrees.
    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    /// Returns the current pitch in degrees.
    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    fn update_front_from_angles(&mut self) {
        let yaw_r = self.yaw.to_radians();
        let pitch_r = self.pitch.to_radians();

        let direction = glm::vec3(
            yaw_r.cos() * pitch_r.cos(),
            pitch_r.sin(),
            yaw_r.sin() * pitch_r.cos(),
        );

        if glm::length(&direction) > 0.0 {
            self.front = glm::normalize(&direction);
        }
    }
}
