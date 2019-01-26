use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::VelocityCmd;

pub struct MoveCmdSystem;

const SPEED: f32 = 10.;

impl<'s> System<'s> for MoveCmdSystem {
    type SystemData = (
        WriteStorage<'s, VelocityCmd>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut velocity_cmds, input): Self::SystemData) {
        for (velocity_cmd) in (&mut velocity_cmds).join() {
            if let Some(mov_x) = input.axis_value("leftright") {
                velocity_cmd.x = mov_x as f32 * SPEED;
            }
            if let Some(mov_y) = input.axis_value("updown") {
                velocity_cmd.y = mov_y as f32 * SPEED;
            }
        }
    }
}
