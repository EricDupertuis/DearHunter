use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::gamestate::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::hunter::Hunter;

pub struct HunterSystem;

impl<'s> System<'s> for HunterSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Hunter>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, hunters, input): Self::SystemData) {
        for (_hunter, transform) in (&hunters, &mut transforms).join() {
            let mov_x = input.axis_value("leftright");
            let mov_y = input.axis_value("updown");

            if let Some(mv_amount_x) = mov_x {
                if mv_amount_x != 0.0 {
                    let scaled_amount = 1.2 * mv_amount_x as f32;
                    let hunter_x = transform.translation().x;
                    transform.set_x((hunter_x + scaled_amount).min(ARENA_WIDTH - 1.0).max(1.0));
                }
            }

            if let Some(mv_amount_y) = mov_y {
                if mv_amount_y != 0.0 {
                    let scaled_amount = 1.2 * mv_amount_y as f32;
                    let hunter_y = transform.translation().y;
                    transform.set_y((hunter_y + scaled_amount).min(ARENA_HEIGHT - 1.0).max(1.0));
                }
            }
        }
    }
}
