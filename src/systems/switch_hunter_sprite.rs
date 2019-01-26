use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::SpriteRender;

use crate::components::VelocityCmd;
use crate::hunter::Hunter;

pub struct HunterSpriteSwitcher;

impl<'s> System<'s> for HunterSpriteSwitcher {
    type SystemData = (
        ReadStorage<'s, VelocityCmd>,
        ReadStorage<'s, Hunter>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (velocity_cmds, hunters, mut sprites): Self::SystemData) {
        for (sprite, velocity, _) in (&mut sprites, &velocity_cmds, &hunters).join() {
            if velocity.x.abs() < 0.1 && velocity.y.abs() < 0.1 {
                continue;
            }

            let angle = velocity.y.atan2(velocity.x).to_degrees();

            if angle > 65. && angle < 135. {
                // up
                sprite.sprite_number = 0;
            } else if angle < -45. && angle > -135. {
                sprite.sprite_number = 2; // down
            } else if angle < -135. || angle > 135. {
                // left
                sprite.sprite_number = 4;
            } else {
                sprite.sprite_number = 6;
            }
        }
    }
}
