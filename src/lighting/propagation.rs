use std::collections::VecDeque;
use crate::lighting::lighting_world::LightingWorld;
use crate::physics::coordinates::Coordinates;

/// Propagates block light from point sources using BFS flood fill.
///
/// Each RGB channel is attenuated independently by `attenuation` per block of distance,
/// plus the opacity of the destination block. Light cannot enter fully opaque blocks (opacity 255).
pub fn propagate(
    world: &mut dyn LightingWorld,
    seeds: &[Coordinates],
    attenuation: u8,
) {
    let mut queue: VecDeque<Coordinates> = VecDeque::from(seeds.to_vec());

    while let Some(source_cords) = queue.pop_front() {
        let current = world.get_light(source_cords);

        for neighbour_cords in source_cords.neighbors() {
            let opacity = world.get_opacity(neighbour_cords);
            // Fully opaque blocks block all light
            if opacity == 255 {
                continue;
            }

            let total_att = attenuation.saturating_add(opacity);
            let attenuated = [
                current[0].saturating_sub(total_att),
                current[1].saturating_sub(total_att),
                current[2].saturating_sub(total_att),
            ];

            // If the light level is 0 return
            if attenuated == [0, 0, 0] {
                continue;
            }

            let neighbor = world.get_light(neighbour_cords);

            // Check if the new light is actually brighter
            if attenuated[0] > neighbor[0] || attenuated[1] > neighbor[1] || attenuated[2] > neighbor[2] {
                let merged = [
                    neighbor[0].max(attenuated[0]),
                    neighbor[1].max(attenuated[1]),
                    neighbor[2].max(attenuated[2]),
                ];
                world.set_light(neighbour_cords, merged);
                queue.push_back(neighbour_cords);
            }
        }
    }
}

/// Removes light starting from the given coordinates.
///
/// This will zero out light that originated from these seeds,
/// then trigger a re-propagation from surrounding light sources.
pub fn unpropagate(
    world: &mut dyn LightingWorld,
    seeds: &[Coordinates],
    attenuation: u8,
) {
    // Queue stores (Position, OldLightValue)
    let mut removal_queue: VecDeque<(Coordinates, [u8; 3])> = VecDeque::new();
    let mut refill_seeds: Vec<Coordinates> = Vec::new();

    // 1. Initial Seeding
    for &coords in seeds {
        let old_light = world.get_light(coords);
        world.set_light(coords, [0, 0, 0]);
        removal_queue.push_back((coords, old_light));
    }

    // 2. The Siphon Pass
    while let Some((pos, old_light)) = removal_queue.pop_front() {
        for neighbor_pos in pos.neighbors() {
            let neighbor_light = world.get_light(neighbor_pos);

            // If the neighbor has light, we need to check if it came from the block we're removing
            if neighbor_light != [0, 0, 0] {
                // Is this neighbor's light an attenuated version of our old light?
                // Account for opacity of the neighbor block
                let neighbor_opacity = world.get_opacity(neighbor_pos);
                let total_att = attenuation.saturating_add(neighbor_opacity);
                let is_derived = neighbor_light[0] == old_light[0].saturating_sub(total_att) &&
                    neighbor_light[1] == old_light[1].saturating_sub(total_att) &&
                    neighbor_light[2] == old_light[2].saturating_sub(total_att);

                if is_derived {
                    // It's part of the same light "tree", so remove it
                    world.set_light(neighbor_pos, [0, 0, 0]);
                    removal_queue.push_back((neighbor_pos, neighbor_light));
                } else {
                    // This neighbor has light that didn't come from us!
                    // It will help refill the vacuum.
                    refill_seeds.push(neighbor_pos);
                }
            }
        }
    }

    // 3. The Refill Pass
    // Reuse your existing propagation logic to fill the area back in
    // using the light from neighbors that were unaffected.
    if !refill_seeds.is_empty() {
        propagate(world, &refill_seeds, attenuation);
    }
}

/// Propagates sky light downward and then spreads it via BFS.
///
/// For each (x, z) column, sky light starts at full `sky_color` and attenuates
/// by each block's opacity as it passes through. Fully opaque blocks (opacity 255)
/// stop the column entirely.
pub fn propagate_sky(
    world: &mut dyn LightingWorld,
    min: Coordinates,
    max: Coordinates,
    sky_color: [u8; 3],
    attenuation: u8,
) {
    let mut seeds = Vec::new();
    // We scan the XZ area provided and drop light from the top
    for x in min.x..=max.x {
        for z in min.z..=max.z {
            let mut sky = sky_color;
            // Start from the highest Y and work down
            for y in (min.y..=max.y).rev() {
                let coords = Coordinates::new(x, y, z);
                let opacity = world.get_opacity(coords);

                if opacity == 255 {
                    // Hit something fully opaque; the rest of this column is in shadow
                    break;
                }

                // Attenuate sky by block opacity (no distance penalty vertically)
                sky = [
                    sky[0].saturating_sub(opacity),
                    sky[1].saturating_sub(opacity),
                    sky[2].saturating_sub(opacity),
                ];

                if sky == [0, 0, 0] {
                    break;
                }

                world.set_light(coords, sky);
                seeds.push(coords);
            }
        }
    }

    // BFS Spread
    // Instead of duplicating the BFS code, reuse the propagate function
    propagate(world, &seeds, attenuation);
}

/// Casts a shadow downward from a newly placed opaque block and
/// removes the associated sky light.
pub fn unpropagate_sky(
    world: &mut dyn LightingWorld,
    placed_pos: Coordinates,
    _sky_color: [u8; 3],
    attenuation: u8,
) {
    let mut removal_seeds = Vec::new();

    // 1. Trace the vertical shadow column
    // Everything directly below the placed block that had sky light
    // and is not fully opaque must be marked for removal.
    let mut current_y = placed_pos.y - 1;

    loop {
        let current_pos = Coordinates::new(placed_pos.x, current_y, placed_pos.z);
        let opacity = world.get_opacity(current_pos);

        if opacity == 255 {
            break;
        }

        let light = world.get_light(current_pos);
        if light != [0, 0, 0] {
            removal_seeds.push(current_pos);
        } else {
            break;
        }

        if current_y == 0 { break; }
        current_y -= 1;
    }

    // 2. Reuse the generic unpropagate logic
    // This handles the "Siphon" (clearing horizontal bleed)
    // and the "Refill" (if another hole in the roof provides light).
    unpropagate(world, &removal_seeds, attenuation);
}
