use std::collections::VecDeque;
use crate::lighting::lightmap::{LightSource, Lightmap};

/// Trait for querying block transparency in the world.
pub trait LightGrid {
    /// Returns `true` if the block at the given position allows light to pass through.
    fn is_transparent(&self, x: i32, y: i32, z: i32) -> bool;
}

/// Propagates block light from point sources using BFS flood fill.
///
/// Each RGB channel is attenuated independently by `attenuation` per block of distance.
/// Light only spreads through transparent blocks (as reported by `grid`).
pub fn propagate(
    lightmap: &mut Lightmap,
    sources: &[LightSource],
    grid: &dyn LightGrid,
    attenuation: u8,
) {
    let (w, h, d) = (lightmap.width, lightmap.height, lightmap.depth);
    let mut queue: VecDeque<(u32, u32, u32)> = VecDeque::new();

    // Seed sources
    for src in sources {
        if src.x < w && src.y < h && src.z < d {
            let existing = lightmap.get(src.x, src.y, src.z);
            let merged = [
                existing[0].max(src.color[0]),
                existing[1].max(src.color[1]),
                existing[2].max(src.color[2]),
            ];
            lightmap.set(src.x, src.y, src.z, merged);
            queue.push_back((src.x, src.y, src.z));
        }
    }

    // BFS flood fill
    let neighbors: [(i32, i32, i32); 6] = [
        (1, 0, 0), (-1, 0, 0),
        (0, 1, 0), (0, -1, 0),
        (0, 0, 1), (0, 0, -1),
    ];

    while let Some((x, y, z)) = queue.pop_front() {
        let current = lightmap.get(x, y, z);

        for (dx, dy, dz) in &neighbors {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            let nz = z as i32 + dz;

            if nx < 0 || ny < 0 || nz < 0 {
                continue;
            }
            let (nx, ny, nz) = (nx as u32, ny as u32, nz as u32);
            if nx >= w || ny >= h || nz >= d {
                continue;
            }

            if !grid.is_transparent(nx as i32, ny as i32, nz as i32) {
                continue;
            }

            let attenuated = [
                current[0].saturating_sub(attenuation),
                current[1].saturating_sub(attenuation),
                current[2].saturating_sub(attenuation),
            ];

            // Skip if fully attenuated
            if attenuated[0] == 0 && attenuated[1] == 0 && attenuated[2] == 0 {
                continue;
            }

            let neighbor = lightmap.get(nx, ny, nz);
            if attenuated[0] > neighbor[0] || attenuated[1] > neighbor[1] || attenuated[2] > neighbor[2] {
                let merged = [
                    neighbor[0].max(attenuated[0]),
                    neighbor[1].max(attenuated[1]),
                    neighbor[2].max(attenuated[2]),
                ];
                lightmap.set(nx, ny, nz, merged);
                queue.push_back((nx, ny, nz));
            }
        }
    }
}

/// Propagates sky light downward and then spreads it via BFS.
///
/// For each (x, z) column, all voxels above the highest opaque block receive full `sky_color`.
/// Below that, light spreads through transparent blocks with `attenuation` per step.
pub fn propagate_sky(
    lightmap: &mut Lightmap,
    grid: &dyn LightGrid,
    sky_color: [u8; 3],
    attenuation: u8,
) {
    let (w, h, d) = (lightmap.width, lightmap.height, lightmap.depth);
    let mut queue: VecDeque<(u32, u32, u32)> = VecDeque::new();

    for x in 0..w {
        for z in 0..d {
            // Find the highest opaque block in this column
            let mut top_opaque: i32 = -1;
            for y in (0..h).rev() {
                if !grid.is_transparent(x as i32, y as i32, z as i32) {
                    top_opaque = y as i32;
                    break;
                }
            }

            // All voxels above the highest opaque block get full sky light
            for y in (top_opaque + 1) as u32..h {
                let existing = lightmap.get(x, y, z);
                let merged = [
                    existing[0].max(sky_color[0]),
                    existing[1].max(sky_color[1]),
                    existing[2].max(sky_color[2]),
                ];
                lightmap.set(x, y, z, merged);
                queue.push_back((x, y, z));
            }
        }
    }

    // BFS to spread sky light into caves/overhangs
    let neighbors: [(i32, i32, i32); 6] = [
        (1, 0, 0), (-1, 0, 0),
        (0, 1, 0), (0, -1, 0),
        (0, 0, 1), (0, 0, -1),
    ];

    while let Some((x, y, z)) = queue.pop_front() {
        let current = lightmap.get(x, y, z);

        for (dx, dy, dz) in &neighbors {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            let nz = z as i32 + dz;

            if nx < 0 || ny < 0 || nz < 0 {
                continue;
            }
            let (nx, ny, nz) = (nx as u32, ny as u32, nz as u32);
            if nx >= w || ny >= h || nz >= d {
                continue;
            }

            if !grid.is_transparent(nx as i32, ny as i32, nz as i32) {
                continue;
            }

            let attenuated = [
                current[0].saturating_sub(attenuation),
                current[1].saturating_sub(attenuation),
                current[2].saturating_sub(attenuation),
            ];

            if attenuated[0] == 0 && attenuated[1] == 0 && attenuated[2] == 0 {
                continue;
            }

            let neighbor = lightmap.get(nx, ny, nz);
            if attenuated[0] > neighbor[0] || attenuated[1] > neighbor[1] || attenuated[2] > neighbor[2] {
                let merged = [
                    neighbor[0].max(attenuated[0]),
                    neighbor[1].max(attenuated[1]),
                    neighbor[2].max(attenuated[2]),
                ];
                lightmap.set(nx, ny, nz, merged);
                queue.push_back((nx, ny, nz));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct OpenGrid;
    impl LightGrid for OpenGrid {
        fn is_transparent(&self, _x: i32, _y: i32, _z: i32) -> bool {
            true
        }
    }

    struct WallGrid;
    impl LightGrid for WallGrid {
        fn is_transparent(&self, x: i32, _y: i32, _z: i32) -> bool {
            // Wall at x == 2
            x != 2
        }
    }

    #[test]
    fn propagate_single_source_spreads() {
        let mut lm = Lightmap::new(5, 5, 5);
        let sources = [LightSource { x: 2, y: 2, z: 2, color: [255, 128, 64] }];

        propagate(&mut lm, &sources, &OpenGrid, 17);

        // Source should have full color
        assert_eq!(lm.get(2, 2, 2), [255, 128, 64]);

        // Adjacent should be attenuated
        let adj = lm.get(3, 2, 2);
        assert_eq!(adj, [238, 111, 47]);

        // Two steps away should be further attenuated
        let far = lm.get(4, 2, 2);
        assert_eq!(far, [221, 94, 30]);
    }

    #[test]
    fn propagate_blocked_by_wall() {
        let mut lm = Lightmap::new(5, 1, 1);
        let sources = [LightSource { x: 0, y: 0, z: 0, color: [255, 255, 255] }];

        propagate(&mut lm, &sources, &WallGrid, 17);

        // Light should reach x=1
        assert!(lm.get(1, 0, 0)[0] > 0);
        // Wall at x=2 should block light — it stays black
        assert_eq!(lm.get(2, 0, 0), [0, 0, 0]);
        // Beyond wall should also be dark
        assert_eq!(lm.get(3, 0, 0), [0, 0, 0]);
    }

    #[test]
    fn propagate_sky_fills_above_ground() {
        let mut lm = Lightmap::new(3, 5, 3);

        // Ground at y=2 (opaque), everything else transparent
        struct GroundGrid;
        impl LightGrid for GroundGrid {
            fn is_transparent(&self, _x: i32, y: i32, _z: i32) -> bool {
                y != 2
            }
        }

        propagate_sky(&mut lm, &GroundGrid, [200, 200, 180], 17);

        // Above ground (y=3, y=4) should have full sky light
        assert_eq!(lm.get(1, 4, 1), [200, 200, 180]);
        assert_eq!(lm.get(1, 3, 1), [200, 200, 180]);

        // Ground itself (y=2) stays dark (opaque blocks light)
        assert_eq!(lm.get(1, 2, 1), [0, 0, 0]);

        // Below ground (y=1) stays dark — no path around the opaque layer
        assert_eq!(lm.get(1, 1, 1), [0, 0, 0]);
    }

    #[test]
    fn propagate_sky_leaks_through_gap() {
        // 5-wide chunk, ground at y=2 except a gap at x=0 z=0
        let mut lm = Lightmap::new(5, 5, 5);

        struct GapGrid;
        impl LightGrid for GapGrid {
            fn is_transparent(&self, x: i32, y: i32, z: i32) -> bool {
                // Opaque ground at y=2, except a gap at (0,2,0)
                if y == 2 && !(x == 0 && z == 0) {
                    return false;
                }
                true
            }
        }

        propagate_sky(&mut lm, &GapGrid, [200, 200, 180], 17);

        // Sky light comes through the gap at (0,2,0) and reaches y=1
        let below_gap = lm.get(0, 1, 0);
        assert!(below_gap[0] > 0, "Light should leak through gap");

        // Light should spread underground from the gap with attenuation
        let further = lm.get(1, 1, 0);
        assert!(further[0] > 0, "Light should spread from gap");
        assert!(further[0] < below_gap[0], "Light should attenuate with distance");
    }

    #[test]
    fn clear_resets_to_black() {
        let mut lm = Lightmap::new(2, 2, 2);
        lm.set(0, 0, 0, [100, 200, 50]);
        lm.clear();
        assert_eq!(lm.get(0, 0, 0), [0, 0, 0]);
    }

    #[test]
    fn propagate_no_sources() {
        let mut lm = Lightmap::new(4, 4, 4);
        propagate(&mut lm, &[], &OpenGrid, 17);
        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    assert_eq!(lm.get(x, y, z), [0, 0, 0]);
                }
            }
        }
    }

    struct OpaqueGrid;
    impl LightGrid for OpaqueGrid {
        fn is_transparent(&self, _x: i32, _y: i32, _z: i32) -> bool {
            false
        }
    }

    #[test]
    fn propagate_fully_opaque_grid() {
        let mut lm = Lightmap::new(5, 5, 5);
        let sources = [LightSource { x: 2, y: 2, z: 2, color: [255, 255, 255] }];

        propagate(&mut lm, &sources, &OpaqueGrid, 17);

        // Source voxel still gets its color (seeded before BFS)
        assert_eq!(lm.get(2, 2, 2), [255, 255, 255]);
        // But nothing else — BFS can't spread through opaque blocks
        assert_eq!(lm.get(3, 2, 2), [0, 0, 0]);
        assert_eq!(lm.get(1, 2, 2), [0, 0, 0]);
        assert_eq!(lm.get(2, 3, 2), [0, 0, 0]);
    }

    #[test]
    fn propagate_source_at_corner() {
        let mut lm = Lightmap::new(4, 4, 4);
        let sources = [LightSource { x: 0, y: 0, z: 0, color: [255, 255, 255] }];

        propagate(&mut lm, &sources, &OpenGrid, 17);

        // Light should spread in +x, +y, +z only (no negative neighbors)
        assert_eq!(lm.get(0, 0, 0), [255, 255, 255]);
        assert_eq!(lm.get(1, 0, 0), [238, 238, 238]);
        assert_eq!(lm.get(0, 1, 0), [238, 238, 238]);
        assert_eq!(lm.get(0, 0, 1), [238, 238, 238]);
    }

    #[test]
    fn propagate_multiple_sources_merge_max() {
        let mut lm = Lightmap::new(5, 1, 1);
        // Red light at x=0, blue light at x=4
        let sources = [
            LightSource { x: 0, y: 0, z: 0, color: [255, 0, 0] },
            LightSource { x: 4, y: 0, z: 0, color: [0, 0, 255] },
        ];

        propagate(&mut lm, &sources, &OpenGrid, 51);

        // Sources get merged with attenuated light from the other source.
        // Blue travels 4 steps to x=0: 255 - 4*51 = 51
        let at_red = lm.get(0, 0, 0);
        assert_eq!(at_red[0], 255); // red channel stays at source value
        assert_eq!(at_red[2], 51);  // blue channel from attenuated blue source

        // Middle voxel should have both colors merged (max of each channel)
        let mid = lm.get(2, 0, 0);
        assert!(mid[0] > 0, "Red should reach the middle");
        assert!(mid[2] > 0, "Blue should reach the middle");
        // Red: 255 - 2*51 = 153, Blue: 255 - 2*51 = 153
        assert_eq!(mid[0], 153);
        assert_eq!(mid[2], 153);
    }

    #[test]
    fn propagate_high_attenuation_dies_fast() {
        let mut lm = Lightmap::new(5, 1, 1);
        let sources = [LightSource { x: 0, y: 0, z: 0, color: [100, 100, 100] }];

        // Attenuation of 128 means after 1 step: 100 - 128 = 0 (saturating)
        propagate(&mut lm, &sources, &OpenGrid, 128);

        assert_eq!(lm.get(0, 0, 0), [100, 100, 100]);
        // All attenuated values are 0, so nothing propagates
        assert_eq!(lm.get(1, 0, 0), [0, 0, 0]);
    }

    #[test]
    fn propagate_no_diagonal_spread() {
        let mut lm = Lightmap::new(3, 3, 3);
        let sources = [LightSource { x: 1, y: 1, z: 1, color: [51, 51, 51] }];

        // With attenuation=51, one step gives 0, so only the 6 face-adjacent
        // neighbors can be reached, and they get exactly 0 (won't propagate).
        // Use attenuation=50 so one step = 1, two steps = 0.
        propagate(&mut lm, &sources, &OpenGrid, 50);

        // Face-adjacent should get light (51 - 50 = 1)
        assert_eq!(lm.get(2, 1, 1)[0], 1);
        assert_eq!(lm.get(0, 1, 1)[0], 1);
        assert_eq!(lm.get(1, 2, 1)[0], 1);
        assert_eq!(lm.get(1, 0, 1)[0], 1);
        assert_eq!(lm.get(1, 1, 2)[0], 1);
        assert_eq!(lm.get(1, 1, 0)[0], 1);

        // Diagonal neighbor (edge-adjacent) should NOT get direct light.
        // It could only reach via two 1-step BFS hops, but 1-50 saturates to 0.
        assert_eq!(lm.get(2, 2, 1), [0, 0, 0]);
        assert_eq!(lm.get(0, 0, 1), [0, 0, 0]);
        assert_eq!(lm.get(2, 1, 2), [0, 0, 0]);

        // Corner-diagonal should also be 0
        assert_eq!(lm.get(0, 0, 0), [0, 0, 0]);
        assert_eq!(lm.get(2, 2, 2), [0, 0, 0]);
    }

    #[test]
    fn propagate_source_out_of_bounds_ignored() {
        let mut lm = Lightmap::new(3, 3, 3);
        let sources = [
            LightSource { x: 10, y: 0, z: 0, color: [255, 255, 255] },
            LightSource { x: 0, y: 99, z: 0, color: [255, 255, 255] },
        ];

        propagate(&mut lm, &sources, &OpenGrid, 17);

        // Lightmap should remain all black — out-of-bounds sources are skipped
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    assert_eq!(lm.get(x, y, z), [0, 0, 0]);
                }
            }
        }
    }

    #[test]
    fn propagate_symmetric_in_open_grid() {
        let mut lm = Lightmap::new(5, 5, 5);
        let sources = [LightSource { x: 2, y: 2, z: 2, color: [255, 255, 255] }];

        propagate(&mut lm, &sources, &OpenGrid, 17);

        // All 6 face-adjacent voxels should have the same value
        let expected = lm.get(3, 2, 2);
        assert_eq!(lm.get(1, 2, 2), expected);
        assert_eq!(lm.get(2, 3, 2), expected);
        assert_eq!(lm.get(2, 1, 2), expected);
        assert_eq!(lm.get(2, 2, 3), expected);
        assert_eq!(lm.get(2, 2, 1), expected);

        // All 12 edge-adjacent voxels should be equal
        let edge = lm.get(3, 3, 2);
        assert_eq!(lm.get(1, 1, 2), edge);
        assert_eq!(lm.get(3, 2, 3), edge);
        assert_eq!(lm.get(2, 3, 3), edge);
    }

    #[test]
    fn propagate_independent_rgb_channels() {
        let mut lm = Lightmap::new(3, 1, 1);
        // Source with very different channel values
        let sources = [LightSource { x: 0, y: 0, z: 0, color: [255, 100, 17] }];

        propagate(&mut lm, &sources, &OpenGrid, 17);

        let one_step = lm.get(1, 0, 0);
        assert_eq!(one_step[0], 238); // 255 - 17
        assert_eq!(one_step[1], 83);  // 100 - 17
        assert_eq!(one_step[2], 0);   // 17 - 17

        let two_step = lm.get(2, 0, 0);
        assert_eq!(two_step[0], 221); // 238 - 17
        assert_eq!(two_step[1], 66);  // 83 - 17
        assert_eq!(two_step[2], 0);   // already 0, can't propagate from 0
    }

    #[test]
    fn propagate_sky_no_opaque_blocks() {
        let mut lm = Lightmap::new(3, 4, 3);

        propagate_sky(&mut lm, &OpenGrid, [200, 200, 180], 17);

        // Entire volume should have full sky color — no opaque blocks anywhere
        for x in 0..3 {
            for y in 0..4 {
                for z in 0..3 {
                    assert_eq!(lm.get(x, y, z), [200, 200, 180],
                        "voxel ({x},{y},{z}) should have full sky color");
                }
            }
        }
    }

    #[test]
    fn propagate_sky_fully_opaque_ceiling() {
        // Opaque layer at the very top (y = height-1)
        let mut lm = Lightmap::new(3, 4, 3);

        struct CeilingGrid;
        impl LightGrid for CeilingGrid {
            fn is_transparent(&self, _x: i32, y: i32, _z: i32) -> bool {
                y != 3 // opaque at y=3 (top)
            }
        }

        propagate_sky(&mut lm, &CeilingGrid, [200, 200, 180], 17);

        // Opaque ceiling blocks all sky light — nothing above it
        assert_eq!(lm.get(1, 3, 1), [0, 0, 0]);

        // Below ceiling should get attenuated BFS light (no direct sky)
        // Actually: top_opaque = 3, so sky fills y > 3, which is nothing (h=4).
        // BFS has nothing to start from, so everything stays black.
        for y in 0..4 {
            assert_eq!(lm.get(1, y, 1), [0, 0, 0],
                "y={y} should be dark with opaque ceiling at top");
        }
    }

    #[test]
    fn propagate_sky_and_block_light_combine() {
        let mut lm = Lightmap::new(5, 5, 5);

        // Ground at y=2, transparent gap at (2,2,2)
        struct PartialGround;
        impl LightGrid for PartialGround {
            fn is_transparent(&self, x: i32, y: i32, z: i32) -> bool {
                if y == 2 && !(x == 2 && z == 2) {
                    return false;
                }
                true
            }
        }

        // First run sky light
        propagate_sky(&mut lm, &PartialGround, [100, 100, 80], 17);

        // Then add a red block light underground
        let sources = [LightSource { x: 2, y: 0, z: 2, color: [200, 0, 0] }];
        propagate(&mut lm, &sources, &PartialGround, 17);

        // The source voxel should merge: sky light leaked + block light
        let at_source = lm.get(2, 0, 2);
        assert_eq!(at_source[0], 200); // max(sky_leaked_r, 200)
        assert!(at_source[1] > 0);     // some sky green leaked through gap

        // Above ground should still have sky light
        assert_eq!(lm.get(0, 4, 0), [100, 100, 80]);
    }

    #[test]
    fn propagate_light_wraps_around_obstacle() {
        // L-shaped corridor: wall at x=2 for y=0, but open at y=1
        let mut lm = Lightmap::new(5, 2, 1);

        struct LShapeGrid;
        impl LightGrid for LShapeGrid {
            fn is_transparent(&self, x: i32, y: i32, _z: i32) -> bool {
                // Wall at x=2, y=0 only
                !(x == 2 && y == 0)
            }
        }

        let sources = [LightSource { x: 0, y: 0, z: 0, color: [255, 255, 255] }];
        propagate(&mut lm, &sources, &LShapeGrid, 17);

        // Direct path blocked at x=2, y=0
        assert_eq!(lm.get(2, 0, 0), [0, 0, 0]);

        // But light can go: (0,0) -> (0,1) -> (1,1) -> (2,1) -> (3,1) -> (3,0)
        // or (0,0) -> (1,0) -> (1,1) -> (2,1) -> (3,1) -> (3,0)
        assert!(lm.get(3, 0, 0)[0] > 0, "Light should wrap around the wall");
        assert!(lm.get(4, 0, 0)[0] > 0, "Light should reach beyond the wall");
    }

    #[test]
    fn propagate_attenuation_zero_fills_entire_grid() {
        let mut lm = Lightmap::new(3, 3, 3);
        let sources = [LightSource { x: 0, y: 0, z: 0, color: [200, 150, 100] }];

        // No attenuation — light should fill the entire open grid at full strength
        propagate(&mut lm, &sources, &OpenGrid, 0);

        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    assert_eq!(lm.get(x, y, z), [200, 150, 100],
                        "voxel ({x},{y},{z}) should have full light with 0 attenuation");
                }
            }
        }
    }

    #[test]
    fn propagate_sky_varying_terrain_height() {
        // Terrain: ground at different heights per column
        let mut lm = Lightmap::new(3, 6, 1);

        struct HillyGrid;
        impl LightGrid for HillyGrid {
            fn is_transparent(&self, x: i32, y: i32, _z: i32) -> bool {
                // Column heights: x=0 -> ground at y=1, x=1 -> y=3, x=2 -> y=0
                match x {
                    0 => y > 1,
                    1 => y > 3,
                    2 => y > 0,
                    _ => true,
                }
            }
        }

        propagate_sky(&mut lm, &HillyGrid, [255, 255, 255], 17);

        // x=0: sky from y=2..5
        assert_eq!(lm.get(0, 5, 0), [255, 255, 255]);
        assert_eq!(lm.get(0, 2, 0), [255, 255, 255]);
        assert_eq!(lm.get(0, 1, 0), [0, 0, 0]); // opaque

        // x=1: sky from y=4..5 only
        assert_eq!(lm.get(1, 5, 0), [255, 255, 255]);
        assert_eq!(lm.get(1, 4, 0), [255, 255, 255]);
        assert_eq!(lm.get(1, 3, 0), [0, 0, 0]); // opaque

        // x=2: sky from y=1..5
        assert_eq!(lm.get(2, 1, 0), [255, 255, 255]);
        assert_eq!(lm.get(2, 0, 0), [0, 0, 0]); // opaque
    }
}
