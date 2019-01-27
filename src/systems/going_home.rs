use amethyst::{
    core::shrev::EventChannel,
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, Resources, System, Write},
};

use crate::components::BoundingRect;
use crate::events::GameEndEvent;
use crate::home::Home;
use crate::hunter::Hunter;

pub struct GoingHomeSystem;

impl<'s> System<'s> for GoingHomeSystem {
    type SystemData = (
        ReadStorage<'s, BoundingRect>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Home>,
        ReadStorage<'s, Hunter>,
        Write<'s, EventChannel<GameEndEvent>>,
    );

    fn run(&mut self, (rectangles, transforms, homes, hunters, mut game_end): Self::SystemData) {
        for (_hunter, hunter_brect, hunter_transform) in (&hunters, &rectangles, &transforms).join()
        {
            for (_home, home_brect, home_transform) in (&homes, &rectangles, &transforms).join() {
                let x = hunter_transform.translation().x;
                let y = hunter_transform.translation().y;

                let home_x = home_transform.translation().x;
                let home_y = home_transform.translation().y;

                let home_left = home_x - 0.5 * (hunter_brect.width + home_brect.width);
                let home_right = home_x + 0.5 * (hunter_brect.width + home_brect.width);
                let home_top = home_y + 0.5 * (hunter_brect.height + home_brect.height);
                let home_bottom = home_y - 0.5 * (hunter_brect.height + home_brect.height);

                if point_in_rect(x, y, home_left, home_bottom, home_right, home_top) {
                    println!("You are home!");
                    game_end.single_write(GameEndEvent::Win);
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
