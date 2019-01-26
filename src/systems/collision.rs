use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::gamestate::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::hunter::{Hunter, Velocity, VelocityCmd};

pub struct CollisionSystem;
impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Hunter>,
        ReadStorage<'s, VelocityCmd>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut velocities, hunters, commands, transforms): Self::SystemData) {
        for (_hunter, transform, vel, cmd) in
            (&hunters, &transforms, &mut velocities, &commands).join()
        {
            // Map commands to velocities
            vel.x = cmd.x;
            vel.y = cmd.y;

            // Perform collision checks
            let x = transform.translation().x;
            let y = transform.translation().y;

            let hunter_height = 2.0;
            let hunter_width = 2.0;

            if ((y + hunter_height / 2.) >= ARENA_HEIGHT && vel.y > 0.)
                || ((y - hunter_height / 2.) <= 0. && vel.y < 0.)
            {
                vel.y = 0.0;
            }

            if ((x + hunter_width / 2.) >= ARENA_WIDTH && vel.x > 0.)
                || ((x - hunter_width / 2.) <= 0. && vel.x < 0.)
            {
                vel.x = 0.0;
            }
        }
    }
}
