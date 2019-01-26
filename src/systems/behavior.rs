use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{VelocityCmd};
use crate::tree::Tree;
use crate::beast::Beast;
use crate::hunter::Hunter;



pub struct BehaviorSystem;

impl<'s> System<'s> for BehaviorSystem {
    type SystemData = (
        WriteStorage<'s, VelocityCmd>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Hunter>,
        ReadStorage<'s, Tree>,
        ReadStorage<'s, Beast>,
        );

    fn run(&mut self, (mut commands, transforms, hunter, _trees, beasts): Self::SystemData) {
        //Get position of hunter
        let mut x = 0.;
        let mut y = 0.;
        for (trans, _hunter) in (&transforms, &hunter).join() {
            x = trans.translation().x;
            y = trans.translation().y;
        }

        for (cmd, trans, _beast) in (&mut commands, &transforms, &beasts).join() {
            let bx = trans.translation().x;
            let by = trans.translation().y;

            cmd.x = if x > bx {
                -1.
            } else {
                1.
            };
            cmd.y = if y > by {
                -1.
            } else {
                1.
            };
        }
    }
}
            
