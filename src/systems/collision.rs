use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Velocity, VelocityCmd};
use crate::gamestate::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::hunter::Hunter;
pub struct CollisionSystem;
impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Hunter>,
        ReadStorage<'s, VelocityCmd>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut velocities, hunters, commands, transforms): Self::SystemData) {
        for (hunter, transform, vel, cmd) in
            (&hunters, &transforms, &mut velocities, &commands).join()
        {
            // Map commands to velocities
            vel.x = cmd.x;
            vel.y = cmd.y;

            // Perform collision checks
            let x = transform.translation().x;
            let y = transform.translation().y;

            let hunter_half_height = hunter.height / 2.;
            let hunter_half_width = hunter.width / 2.;

            if ((y + hunter_half_height) >= ARENA_HEIGHT && vel.y > 0.)
                || ((y - hunter_half_height) <= 0. && vel.y < 0.)
            {
                vel.y = 0.0;
            }

            if ((x + hunter_half_width) >= ARENA_WIDTH && vel.x > 0.)
                || ((x - hunter_half_width) <= 0. && vel.x < 0.)
            {
                vel.x = 0.0;
            }
        }
    }
}
