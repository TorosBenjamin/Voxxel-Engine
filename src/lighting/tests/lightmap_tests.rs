#[cfg(test)]
mod tests {
    use crate::lighting::lightmap::Lightmap;
    use super::*;

    #[test]
    fn new_initializes_to_black() {
        let lm = Lightmap::new(4, 3, 2);
        assert_eq!(lm.width, 4);
        assert_eq!(lm.height, 3);
        assert_eq!(lm.depth, 2);
        for z in 0..2 {
            for y in 0..3 {
                for x in 0..4 {
                    assert_eq!(lm.get(x, y, z), [0, 0, 0]);
                }
            }
        }
    }

    #[test]
    fn set_get_roundtrip() {
        let mut lm = Lightmap::new(4, 4, 4);
        lm.set(1, 2, 3, [10, 20, 30]);
        assert_eq!(lm.get(1, 2, 3), [10, 20, 30]);
        // Other voxels should be untouched
        assert_eq!(lm.get(0, 0, 0), [0, 0, 0]);
        assert_eq!(lm.get(3, 3, 3), [0, 0, 0]);
    }

    #[test]
    fn set_get_all_corners() {
        let mut lm = Lightmap::new(3, 4, 5);
        let corners = [
            (0, 0, 0), (2, 0, 0), (0, 3, 0), (0, 0, 4),
            (2, 3, 0), (2, 0, 4), (0, 3, 4), (2, 3, 4),
        ];
        for (i, &(x, y, z)) in corners.iter().enumerate() {
            let v = (i as u8 + 1) * 30;
            lm.set(x, y, z, [v, v, v]);
        }
        for (i, &(x, y, z)) in corners.iter().enumerate() {
            let v = (i as u8 + 1) * 30;
            assert_eq!(lm.get(x, y, z), [v, v, v], "corner ({x},{y},{z})");
        }
    }

    #[test]
    fn index_is_row_major() {
        let lm = Lightmap::new(4, 3, 2);
        // index = x + y * width + z * width * height
        assert_eq!(lm.index(0, 0, 0), 0);
        assert_eq!(lm.index(1, 0, 0), 1);
        assert_eq!(lm.index(0, 1, 0), 4);     // y=1 -> offset by width=4
        assert_eq!(lm.index(0, 0, 1), 12);    // z=1 -> offset by width*height=4*3=12
        assert_eq!(lm.index(3, 2, 1), 3 + 2 * 4 + 1 * 4 * 3); // 3+8+12=23
    }

    #[test]
    fn set_overwrites_previous_value() {
        let mut lm = Lightmap::new(2, 2, 2);
        lm.set(1, 1, 1, [100, 200, 50]);
        assert_eq!(lm.get(1, 1, 1), [100, 200, 50]);
        lm.set(1, 1, 1, [5, 10, 15]);
        assert_eq!(lm.get(1, 1, 1), [5, 10, 15]);
    }

    #[test]
    fn clear_resets_all_voxels() {
        let mut lm = Lightmap::new(3, 3, 3);
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    lm.set(x, y, z, [255, 128, 64]);
                }
            }
        }
        lm.clear();
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    assert_eq!(lm.get(x, y, z), [0, 0, 0]);
                }
            }
        }
    }

    #[test]
    fn as_bytes_length() {
        let lm = Lightmap::new(4, 3, 2);
        assert_eq!(lm.as_bytes().len(), 4 * 3 * 2 * 3); // w*h*d * 3 bytes per voxel
    }

    #[test]
    fn as_bytes_matches_data() {
        let mut lm = Lightmap::new(2, 1, 1);
        lm.set(0, 0, 0, [10, 20, 30]);
        lm.set(1, 0, 0, [40, 50, 60]);
        let bytes = lm.as_bytes();
        assert_eq!(bytes, &[10, 20, 30, 40, 50, 60]);
    }

    #[test]
    fn adjacent_voxels_independent() {
        let mut lm = Lightmap::new(4, 4, 4);
        lm.set(1, 1, 1, [255, 0, 0]);
        lm.set(2, 1, 1, [0, 255, 0]);
        lm.set(1, 2, 1, [0, 0, 255]);
        assert_eq!(lm.get(1, 1, 1), [255, 0, 0]);
        assert_eq!(lm.get(2, 1, 1), [0, 255, 0]);
        assert_eq!(lm.get(1, 2, 1), [0, 0, 255]);
        // Untouched neighbor
        assert_eq!(lm.get(1, 1, 2), [0, 0, 0]);
    }

    #[test]
    fn dimensions_1x1x1() {
        let mut lm = Lightmap::new(1, 1, 1);
        assert_eq!(lm.get(0, 0, 0), [0, 0, 0]);
        lm.set(0, 0, 0, [42, 43, 44]);
        assert_eq!(lm.get(0, 0, 0), [42, 43, 44]);
        assert_eq!(lm.as_bytes(), &[42, 43, 44]);
    }
}
