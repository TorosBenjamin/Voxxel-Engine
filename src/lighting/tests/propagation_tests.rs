#[cfg(test)]
mod tests {
    use crate::lighting::lighting_world::LightingWorld;
    use crate::lighting::lightmap::Lightmap;
    use crate::lighting::propagation::{propagate, propagate_sky};
    use crate::physics::coordinates::Coordinates;

    /// Test world that wraps a Lightmap with an opacity function.
    /// F returns 0 for transparent, 255 for opaque, or values in between for semi-opaque.
    struct TestWorld<F: Fn(i32, i32, i32) -> u8> {
        lm: Lightmap,
        get_opacity_fn: F,
    }

    impl<F: Fn(i32, i32, i32) -> u8> TestWorld<F> {
        fn new(w: u32, h: u32, d: u32, get_opacity_fn: F) -> Self {
            Self { lm: Lightmap::new(w, h, d), get_opacity_fn }
        }

        fn get(&self, x: u32, y: u32, z: u32) -> [u8; 3] {
            self.lm.get(x, y, z)
        }

        fn in_bounds(&self, c: Coordinates) -> bool {
            c.x >= 0 && c.x < self.lm.width as i32
                && c.y >= 0 && c.y < self.lm.height as i32
                && c.z >= 0 && c.z < self.lm.depth as i32
        }
    }

    impl<F: Fn(i32, i32, i32) -> u8> LightingWorld for TestWorld<F> {
        fn get_opacity(&self, cords: Coordinates) -> u8 {
            if !self.in_bounds(cords) {
                return 255;
            }
            (self.get_opacity_fn)(cords.x, cords.y, cords.z)
        }

        fn get_light(&self, cords: Coordinates) -> [u8; 3] {
            if !self.in_bounds(cords) {
                return [0, 0, 0];
            }
            self.lm.get(cords.x as u32, cords.y as u32, cords.z as u32)
        }

        fn set_light(&mut self, cords: Coordinates, color: [u8; 3]) {
            if self.in_bounds(cords) {
                self.lm.set(cords.x as u32, cords.y as u32, cords.z as u32, color);
            }
        }
    }

    /// Helper to seed light sources and propagate.
    fn seed_and_propagate<F: Fn(i32, i32, i32) -> u8>(
        world: &mut TestWorld<F>,
        sources: &[(u32, u32, u32, [u8; 3])],
        attenuation: u8,
    ) {
        let mut seeds = Vec::new();
        for &(x, y, z, color) in sources {
            let c = Coordinates::new(x as i32, y as i32, z as i32);
            world.set_light(c, color);
            seeds.push(c);
        }
        propagate(world, &seeds, attenuation);
    }

    fn open(_x: i32, _y: i32, _z: i32) -> u8 { 0 }
    fn wall_at_x2(x: i32, _y: i32, _z: i32) -> u8 { if x == 2 { 255 } else { 0 } }

    #[test]
    fn propagate_single_source_spreads() {
        let mut world = TestWorld::new(5, 5, 5, open);
        seed_and_propagate(&mut world, &[(2, 2, 2, [255, 128, 64])], 17);

        assert_eq!(world.get(2, 2, 2), [255, 128, 64]);
        assert_eq!(world.get(3, 2, 2), [238, 111, 47]);
        assert_eq!(world.get(4, 2, 2), [221, 94, 30]);
    }

    #[test]
    fn propagate_blocked_by_wall() {
        let mut world = TestWorld::new(5, 1, 1, wall_at_x2);
        seed_and_propagate(&mut world, &[(0, 0, 0, [255, 255, 255])], 17);

        assert!(world.get(1, 0, 0)[0] > 0);
        assert_eq!(world.get(2, 0, 0), [0, 0, 0]);
        assert_eq!(world.get(3, 0, 0), [0, 0, 0]);
    }

    #[test]
    fn propagate_sky_fills_above_ground() {
        let mut world = TestWorld::new(3, 5, 3, |_x, y, _z| if y == 2 { 255 } else { 0 });
        let min = Coordinates::new(0, 0, 0);
        let max = Coordinates::new(2, 4, 2);
        propagate_sky(&mut world, min, max, [200, 200, 180], 17);

        assert_eq!(world.get(1, 4, 1), [200, 200, 180]);
        assert_eq!(world.get(1, 3, 1), [200, 200, 180]);
        assert_eq!(world.get(1, 2, 1), [0, 0, 0]);
        assert_eq!(world.get(1, 1, 1), [0, 0, 0]);
    }

    #[test]
    fn propagate_sky_leaks_through_gap() {
        let mut world = TestWorld::new(5, 5, 5, |x, y, z| {
            if y == 2 && !(x == 0 && z == 0) { return 255; }
            0
        });
        let min = Coordinates::new(0, 0, 0);
        let max = Coordinates::new(4, 4, 4);
        propagate_sky(&mut world, min, max, [200, 200, 180], 17);

        let below_gap = world.get(0, 1, 0);
        assert!(below_gap[0] > 0, "Light should leak through gap");

        let further = world.get(1, 1, 0);
        assert!(further[0] > 0, "Light should spread from gap");
        assert!(further[0] < below_gap[0], "Light should attenuate with distance");
    }

    #[test]
    fn clear_resets_to_black() {
        let mut world = TestWorld::new(2, 2, 2, open);
        world.lm.set(0, 0, 0, [100, 200, 50]);
        world.lm.clear();
        assert_eq!(world.get(0, 0, 0), [0, 0, 0]);
    }

    #[test]
    fn propagate_no_sources() {
        let mut world = TestWorld::new(4, 4, 4, open);
        propagate(&mut world, &[], 17);
        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    assert_eq!(world.get(x, y, z), [0, 0, 0]);
                }
            }
        }
    }

    #[test]
    fn propagate_fully_opaque_grid() {
        let mut world = TestWorld::new(5, 5, 5, |_, _, _| 255);
        seed_and_propagate(&mut world, &[(2, 2, 2, [255, 255, 255])], 17);

        assert_eq!(world.get(2, 2, 2), [255, 255, 255]);
        assert_eq!(world.get(3, 2, 2), [0, 0, 0]);
        assert_eq!(world.get(1, 2, 2), [0, 0, 0]);
        assert_eq!(world.get(2, 3, 2), [0, 0, 0]);
    }

    #[test]
    fn propagate_source_at_corner() {
        let mut world = TestWorld::new(4, 4, 4, open);
        seed_and_propagate(&mut world, &[(0, 0, 0, [255, 255, 255])], 17);

        assert_eq!(world.get(0, 0, 0), [255, 255, 255]);
        assert_eq!(world.get(1, 0, 0), [238, 238, 238]);
        assert_eq!(world.get(0, 1, 0), [238, 238, 238]);
        assert_eq!(world.get(0, 0, 1), [238, 238, 238]);
    }

    #[test]
    fn propagate_multiple_sources_merge_max() {
        let mut world = TestWorld::new(5, 1, 1, open);
        seed_and_propagate(&mut world, &[
            (0, 0, 0, [255, 0, 0]),
            (4, 0, 0, [0, 0, 255]),
        ], 51);

        let at_red = world.get(0, 0, 0);
        assert_eq!(at_red[0], 255);
        assert_eq!(at_red[2], 51);

        let mid = world.get(2, 0, 0);
        assert!(mid[0] > 0, "Red should reach the middle");
        assert!(mid[2] > 0, "Blue should reach the middle");
        assert_eq!(mid[0], 153);
        assert_eq!(mid[2], 153);
    }

    #[test]
    fn propagate_high_attenuation_dies_fast() {
        let mut world = TestWorld::new(5, 1, 1, open);
        seed_and_propagate(&mut world, &[(0, 0, 0, [100, 100, 100])], 128);

        assert_eq!(world.get(0, 0, 0), [100, 100, 100]);
        assert_eq!(world.get(1, 0, 0), [0, 0, 0]);
    }

    #[test]
    fn propagate_no_diagonal_spread() {
        let mut world = TestWorld::new(3, 3, 3, open);
        seed_and_propagate(&mut world, &[(1, 1, 1, [51, 51, 51])], 50);

        assert_eq!(world.get(2, 1, 1)[0], 1);
        assert_eq!(world.get(0, 1, 1)[0], 1);
        assert_eq!(world.get(1, 2, 1)[0], 1);
        assert_eq!(world.get(1, 0, 1)[0], 1);
        assert_eq!(world.get(1, 1, 2)[0], 1);
        assert_eq!(world.get(1, 1, 0)[0], 1);

        assert_eq!(world.get(2, 2, 1), [0, 0, 0]);
        assert_eq!(world.get(0, 0, 1), [0, 0, 0]);
        assert_eq!(world.get(2, 1, 2), [0, 0, 0]);
        assert_eq!(world.get(0, 0, 0), [0, 0, 0]);
        assert_eq!(world.get(2, 2, 2), [0, 0, 0]);
    }

    #[test]
    fn propagate_source_out_of_bounds_ignored() {
        let mut world = TestWorld::new(3, 3, 3, open);
        // Seeds that are out of bounds — set_light will be no-ops
        let oob1 = Coordinates::new(10, 0, 0);
        let oob2 = Coordinates::new(0, 99, 0);
        world.set_light(oob1, [255, 255, 255]);
        world.set_light(oob2, [255, 255, 255]);
        propagate(&mut world, &[oob1, oob2], 17);

        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    assert_eq!(world.get(x, y, z), [0, 0, 0]);
                }
            }
        }
    }

    #[test]
    fn propagate_symmetric_in_open_grid() {
        let mut world = TestWorld::new(5, 5, 5, open);
        seed_and_propagate(&mut world, &[(2, 2, 2, [255, 255, 255])], 17);

        let expected = world.get(3, 2, 2);
        assert_eq!(world.get(1, 2, 2), expected);
        assert_eq!(world.get(2, 3, 2), expected);
        assert_eq!(world.get(2, 1, 2), expected);
        assert_eq!(world.get(2, 2, 3), expected);
        assert_eq!(world.get(2, 2, 1), expected);

        let edge = world.get(3, 3, 2);
        assert_eq!(world.get(1, 1, 2), edge);
        assert_eq!(world.get(3, 2, 3), edge);
        assert_eq!(world.get(2, 3, 3), edge);
    }

    #[test]
    fn propagate_independent_rgb_channels() {
        let mut world = TestWorld::new(3, 1, 1, open);
        seed_and_propagate(&mut world, &[(0, 0, 0, [255, 100, 17])], 17);

        let one_step = world.get(1, 0, 0);
        assert_eq!(one_step[0], 238);
        assert_eq!(one_step[1], 83);
        assert_eq!(one_step[2], 0);

        let two_step = world.get(2, 0, 0);
        assert_eq!(two_step[0], 221);
        assert_eq!(two_step[1], 66);
        assert_eq!(two_step[2], 0);
    }

    #[test]
    fn propagate_sky_no_opaque_blocks() {
        let mut world = TestWorld::new(3, 4, 3, open);
        let min = Coordinates::new(0, 0, 0);
        let max = Coordinates::new(2, 3, 2);
        propagate_sky(&mut world, min, max, [200, 200, 180], 17);

        for x in 0..3 {
            for y in 0..4 {
                for z in 0..3 {
                    assert_eq!(world.get(x, y, z), [200, 200, 180],
                               "voxel ({x},{y},{z}) should have full sky color");
                }
            }
        }
    }

    #[test]
    fn propagate_sky_fully_opaque_ceiling() {
        let mut world = TestWorld::new(3, 4, 3, |_x, y, _z| if y == 3 { 255 } else { 0 });
        let min = Coordinates::new(0, 0, 0);
        let max = Coordinates::new(2, 3, 2);
        propagate_sky(&mut world, min, max, [200, 200, 180], 17);

        assert_eq!(world.get(1, 3, 1), [0, 0, 0]);
        for y in 0..4 {
            assert_eq!(world.get(1, y, 1), [0, 0, 0],
                       "y={y} should be dark with opaque ceiling at top");
        }
    }

    #[test]
    fn propagate_sky_and_block_light_combine() {
        let mut world = TestWorld::new(5, 5, 5, |x, y, z| {
            if y == 2 && !(x == 2 && z == 2) { return 255; }
            0
        });
        let min = Coordinates::new(0, 0, 0);
        let max = Coordinates::new(4, 4, 4);
        propagate_sky(&mut world, min, max, [100, 100, 80], 17);

        // Then add a red block light underground
        let source_pos = Coordinates::new(2, 0, 2);
        world.set_light(source_pos, [200, world.get(2, 0, 2)[1], world.get(2, 0, 2)[2]].map(|v| v.max(world.get(2, 0, 2)[0])));
        // Actually: set the source, then propagate
        let current = world.get_light(source_pos);
        let merged = [current[0].max(200), current[1], current[2]];
        world.set_light(source_pos, merged);
        propagate(&mut world, &[source_pos], 17);

        let at_source = world.get(2, 0, 2);
        assert_eq!(at_source[0], 200);
        assert!(at_source[1] > 0);

        assert_eq!(world.get(0, 4, 0), [100, 100, 80]);
    }

    #[test]
    fn propagate_light_wraps_around_obstacle() {
        let mut world = TestWorld::new(5, 2, 1, |x, y, _z| if x == 2 && y == 0 { 255 } else { 0 });
        seed_and_propagate(&mut world, &[(0, 0, 0, [255, 255, 255])], 17);

        assert_eq!(world.get(2, 0, 0), [0, 0, 0]);
        assert!(world.get(3, 0, 0)[0] > 0, "Light should wrap around the wall");
        assert!(world.get(4, 0, 0)[0] > 0, "Light should reach beyond the wall");
    }

    #[test]
    fn propagate_attenuation_zero_fills_entire_grid() {
        let mut world = TestWorld::new(3, 3, 3, open);
        seed_and_propagate(&mut world, &[(0, 0, 0, [200, 150, 100])], 0);

        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    assert_eq!(world.get(x, y, z), [200, 150, 100],
                               "voxel ({x},{y},{z}) should have full light with 0 attenuation");
                }
            }
        }
    }

    #[test]
    fn propagate_sky_varying_terrain_height() {
        let mut world = TestWorld::new(3, 6, 1, |x, y, _z| {
            match x {
                0 => if y > 1 { 0 } else { 255 },
                1 => if y > 3 { 0 } else { 255 },
                2 => if y > 0 { 0 } else { 255 },
                _ => 0,
            }
        });
        let min = Coordinates::new(0, 0, 0);
        let max = Coordinates::new(2, 5, 0);
        propagate_sky(&mut world, min, max, [255, 255, 255], 17);

        assert_eq!(world.get(0, 5, 0), [255, 255, 255]);
        assert_eq!(world.get(0, 2, 0), [255, 255, 255]);
        assert_eq!(world.get(0, 1, 0), [0, 0, 0]);

        assert_eq!(world.get(1, 5, 0), [255, 255, 255]);
        assert_eq!(world.get(1, 4, 0), [255, 255, 255]);
        assert_eq!(world.get(1, 3, 0), [0, 0, 0]);

        assert_eq!(world.get(2, 1, 0), [255, 255, 255]);
        assert_eq!(world.get(2, 0, 0), [0, 0, 0]);
    }

    #[test]
    fn propagate_semi_opaque_block_attenuates() {
        // A semi-opaque block with opacity=30 should add to attenuation
        let mut world = TestWorld::new(5, 1, 1, |x, _, _| {
            if x == 2 { 30 } else { 0 }
        });
        seed_and_propagate(&mut world, &[(0, 0, 0, [255, 255, 255])], 17);

        // At x=1: attenuated by 17 (distance) = 238
        assert_eq!(world.get(1, 0, 0), [238, 238, 238]);
        // At x=2: attenuated by 17 (distance) + 30 (opacity) = 47 total from x=1's value
        // 238 - 47 = 191
        assert_eq!(world.get(2, 0, 0), [191, 191, 191]);
        // At x=3: attenuated by 17 (distance) from x=2's value = 191 - 17 = 174
        assert_eq!(world.get(3, 0, 0), [174, 174, 174]);
    }

    #[test]
    fn propagate_sky_through_semi_opaque() {
        // Semi-opaque blocks at y=3 with opacity=50 should dim sky light
        let mut world = TestWorld::new(1, 5, 1, |_x, y, _z| {
            if y == 3 { 50 } else { 0 }
        });
        let min = Coordinates::new(0, 0, 0);
        let max = Coordinates::new(0, 4, 0);
        propagate_sky(&mut world, min, max, [200, 200, 200], 17);

        // y=4: full sky (opacity 0)
        assert_eq!(world.get(0, 4, 0), [200, 200, 200]);
        // y=3: attenuated by opacity 50 (no distance penalty in column)
        assert_eq!(world.get(0, 3, 0), [150, 150, 150]);
        // y=2: no further attenuation in column (opacity 0)
        assert_eq!(world.get(0, 2, 0), [150, 150, 150]);
    }
}
