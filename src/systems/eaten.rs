use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, Write},
};

use crate::beast::Beast;
use crate::components::BoundingRect;
use crate::game_termination::GameTermination;
use crate::hunter::Hunter;

pub struct EatenSystem;

impl<'s> System<'s> for EatenSystem {
    type SystemData = (
        ReadStorage<'s, BoundingRect>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Beast>,
        ReadStorage<'s, Hunter>,
        Write<'s, GameTermination>,
    );

    fn run(
        &mut self,
        (rectangles, transforms, beasts, hunters, mut game_termination): Self::SystemData,
    ) {
        for (_hunter, hunter_brect, hunter_transform) in (&hunters, &rectangles, &transforms).join()
        {
            for (_beast, beast_brect, beast_transform) in (&beasts, &rectangles, &transforms).join()
            {
                let x = hunter_transform.translation().x;
                let y = hunter_transform.translation().y;

                let beast_x = beast_transform.translation().x;
                let beast_y = beast_transform.translation().y;

                let beast_left = beast_x - 0.5 * (beast_brect.width);
                let beast_right = beast_x + 0.5 * (beast_brect.width);
                let beast_top = beast_y + 0.5 * (beast_brect.height);
                let beast_bottom = beast_y - 0.5 * (beast_brect.height);

                if point_in_rect(x, y, beast_left, beast_bottom, beast_right, beast_top) {
                    println!("You are dead!");
                    game_termination.eaten = true;
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
