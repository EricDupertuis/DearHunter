use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{BoundingRect, Velocity, VelocityCmd};
use crate::gamestate::{ARENA_HEIGHT, ARENA_WIDTH};

pub struct CollisionSystem;
impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, BoundingRect>,
        ReadStorage<'s, VelocityCmd>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut velocities, rectangles, commands, transforms): Self::SystemData) {
        for (brect, transform, vel, cmd) in
            (&rectangles, &transforms, &mut velocities, &commands).join()
        {
            // Map commands to velocities
            vel.x = cmd.x;
            vel.y = cmd.y;

            // Perform collision checks
            let x = transform.translation().x;
            let y = transform.translation().y;

            let half_h = brect.height / 2.;
            let half_w = brect.width / 2.;

            if ((y + half_h) >= ARENA_HEIGHT && vel.y > 0.) || ((y - half_h) <= 0. && vel.y < 0.) {
                vel.y = 0.0;
            }

            if ((x + half_w) >= ARENA_WIDTH && vel.x > 0.) || ((x - half_w) <= 0. && vel.x < 0.) {
                vel.x = 0.0;
            }
        }
    }
}
