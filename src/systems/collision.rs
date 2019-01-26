use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
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
        for (hunter, transform, vel, cmd) in
            (&hunters, &transforms, &mut velocities, &commands).join()
        {
            // Map commands to velocities
            vel.x = cmd.x;
            vel.y = cmd.y;
        }
    }
}
