use crate::math::frustum::*;
use nalgebra_glm as glm;

#[test]
fn test_frustum_culling_behind() {
    // Camera at origin, looking at -Z (default)
    let view = glm::look_at(
        &glm::vec3(0.0, 0.0, 0.0),
        &glm::vec3(0.0, 0.0, -1.0),
        &glm::vec3(0.0, 1.0, 0.0)
    );
    let projection = glm::perspective(1.0, 45.0f32.to_radians(), 0.1, 100.0);
    let frustum = Frustum::from_matrix(&(projection * view));

    // Box behind camera (positive Z)
    let min = glm::vec3(-5.0, -5.0, 5.0);
    let max = glm::vec3(5.0, 5.0, 15.0);
    
    assert!(!frustum.intersects_aabb(&min, &max), "Box behind camera should be culled");
}

#[test]
fn test_frustum_culling_front() {
    // Camera at origin, looking at -Z
    let view = glm::look_at(
        &glm::vec3(0.0, 0.0, 0.0),
        &glm::vec3(0.0, 0.0, -1.0),
        &glm::vec3(0.0, 1.0, 0.0)
    );
    let projection = glm::perspective(1.0, 45.0f32.to_radians(), 0.1, 100.0);
    let frustum = Frustum::from_matrix(&(projection * view));

    // Box in front of camera (negative Z)
    let min = glm::vec3(-1.0, -1.0, -10.0);
    let max = glm::vec3(1.0, 1.0, -5.0);
    
    assert!(frustum.intersects_aabb(&min, &max), "Box in front of camera should NOT be culled");
}

#[test]
fn test_frustum_culling_sideways() {
    // Camera at origin, looking at -Z
    let view = glm::look_at(
        &glm::vec3(0.0, 0.0, 0.0),
        &glm::vec3(0.0, 0.0, -1.0),
        &glm::vec3(0.0, 1.0, 0.0)
    );
    let projection = glm::perspective(1.0, 45.0f32.to_radians(), 0.1, 100.0);
    let frustum = Frustum::from_matrix(&(projection * view));

    // Box to the right, outside FOV
    // FOV is 45 degrees, so at Z=-10, the half-width is 10 * tan(22.5) ~= 4.14
    let min = glm::vec3(10.0, -1.0, -11.0);
    let max = glm::vec3(12.0, 1.0, -9.0);
    
    assert!(!frustum.intersects_aabb(&min, &max), "Box to the far right should be culled");
}
