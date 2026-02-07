use nalgebra_glm as glm;
use crate::physics::collision_map::CollisionMap;
use crate::physics::physics_entity::{KinematicBody, PhysicsEntity};

/// AABB collision system with gravity and friction.
pub struct PhysicsSystem {
    /// Downward acceleration in units per second squared.
    pub gravity: f32,
}

impl PhysicsSystem {
    /// Advances physics by `dt` seconds: applies gravity, friction, and axis-by-axis collision.
    pub fn step<T: KinematicBody, W: CollisionMap>(&self, body: &mut T, world: &W, dt: f32) {
        let entity = body.get_physics();

        // Apply Gravity
        entity.velocity.y -= self.gravity * dt;

        // Apply Drag (Friction)
        // Horizontal friction (X and Z)
        let friction_coeff = if entity.is_grounded { 0.01f32 } else { 0.5f32 };
        let friction = friction_coeff.powf(dt); // Quick way to handle frame-independent decay
        entity.velocity.x *= friction;
        entity.velocity.z *= friction;

        // Move Axis-by-Axis
        self.move_axis(entity, world, dt, 0); // X
        self.move_axis(entity, world, dt, 1); // Y
        self.move_axis(entity, world, dt, 2); // Z
    }

    fn move_axis<W: CollisionMap>(&self, entity: &mut PhysicsEntity, world: &W, dt: f32, axis: usize) {
        if entity.velocity[axis] == 0.0 { return; }

        let movement = entity.velocity[axis] * dt;
        let mut new_pos = entity.position;
        new_pos[axis] += movement;

        if self.is_colliding(new_pos, entity.size, world) {
            entity.velocity[axis] = 0.0;

            if movement > 0.0 {
                // Hitting a wall in front of us (Positive direction)
                // Snap the MAX side of our AABB to the MIN side of the block.
                // The block's min side is (pos + size).floor()
                let hit_block_edge = (new_pos[axis] + entity.size[axis]).floor();
                entity.position[axis] = hit_block_edge - entity.size[axis] - 0.001;
            } else {
                // Hitting a wall behind us (Negative direction)
                // Snap our MIN side to the MAX side of the block.
                // The block's max side is new_pos.floor() + 1.0
                let hit_block_edge = new_pos[axis].floor() + 1.0;
                entity.position[axis] = hit_block_edge + 0.001;
            }

            if axis == 1 && movement < 0.0 {
                entity.is_grounded = true;
            }
        } else {
            entity.position[axis] = new_pos[axis];
            if axis == 1 && movement != 0.0 {
                entity.is_grounded = false;
            }
        }
    }

    fn is_colliding<W: CollisionMap>(&self, pos: glm::Vec3, size: glm::Vec3, world: &W) -> bool {
        // Calculate the min and max bounds of the AABB
        // Note: We subtract a tiny epsilon from the max so we don't
        // collide with a block we are just "touching" the edge of.
        let min_x = pos.x.floor() as i32;
        let min_y = pos.y.floor() as i32;
        let min_z = pos.z.floor() as i32;

        let max_x = (pos.x + size.x - 0.001).floor() as i32;
        let max_y = (pos.y + size.y - 0.001).floor() as i32;
        let max_z = (pos.z + size.z - 0.001).floor() as i32;

        // 2. Iterate through every block coordinate in that volume
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    // 3. If any block in the range is solid, it's a collision
                    if world.is_solid_at(x as f32, y as f32, z as f32) {
                        return true;
                    }
                }
            }
        }
        false
    }
}