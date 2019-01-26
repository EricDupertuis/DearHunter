use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

use crate::gamestate::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::hunter::Velocity;

pub struct MoveSystem;

const STEP: f32 = 0.05;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, Velocity>);

    fn run(&mut self, (mut transforms, velocities): Self::SystemData) {
        for (velocity, transform) in (&velocities, &mut transforms).join() {
            transform.set_x(
                (transform.translation().x + velocity.x * STEP)
                    .min(ARENA_WIDTH - 1.0)
                    .max(1.0),
            );

            let y = (transform.translation().y + velocity.y * STEP)
                .min(ARENA_HEIGHT - 1.0)
                .max(1.0);
            transform.set_y(y);
            transform.set_z(-y);
        }
    }
}
