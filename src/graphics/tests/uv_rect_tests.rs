use crate::graphics::uv_rect::UvRect;
use nalgebra_glm as glm;

#[test]
fn test_uv_rect_full() {
    let rect = UvRect::full();
    assert_eq!(rect.min, glm::vec2(0.0, 0.0));
    assert_eq!(rect.max, glm::vec2(1.0, 1.0));
}
