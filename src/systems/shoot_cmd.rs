use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::ShootCmd;
use crate::hunter::Hunter;

pub struct ShootCmdSystem;

const SPEED: f32 = 20.;

impl<'s> System<'s> for ShootCmdSystem {
    type SystemData = (
        WriteStorage<'s, ShootCmd>,
        Read<'s, InputHandler<String, String>>,
        ReadStorage<'s, Hunter>,
    );

    fn run(&mut self, (mut shoot_cmds, input, hunters): Self::SystemData) {
        for (shoot_cmd, _hunter) in (&mut shoot_cmds, &hunters).join() {
            if input.action_is_down("fire") == Some(true) {
                if let Some((mouse_x, mouse_y)) = input.mouse_position() {
                    shoot_cmd.x = mouse_x;
                    shoot_cmd.y = mouse_y;
                    shoot_cmd.speed = SPEED;
                }
            };
        }
    }
}
