use amethyst::{
    core::nalgebra::Vector2,
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::beast::Beast;
use crate::components::VelocityCmd;
use crate::hunter::Hunter;
use crate::tree::Tree;

pub struct BehaviorSystem;

impl<'s> System<'s> for BehaviorSystem {
    type SystemData = (
        WriteStorage<'s, VelocityCmd>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Hunter>,
        ReadStorage<'s, Tree>,
        ReadStorage<'s, Beast>,
    );

    fn run(&mut self, (mut commands, transforms, hunter, trees, beasts): Self::SystemData) {
        //Get position of hunter
        let mut x = 0.;
        let mut y = 0.;
        for (trans, _hunter) in (&transforms, &hunter).join() {
            x = trans.translation().x;
            y = trans.translation().y;
        }

        for (cmd, trans, beast) in (&mut commands, &transforms, &beasts).join() {
            let bx = trans.translation().x;
            let by = trans.translation().y;

            let mut speed = Vector2::new(0., 0.);

            // Tree avoidance based on vector fields
            for (_, tree_transform) in (&trees, &transforms).join() {
                let dx = bx - tree_transform.translation().x;
                let dy = by - tree_transform.translation().y;

                let mut d = Vector2::new(dx, dy);

                let dst = d.norm();
                if dst > beast.tree_detection_radius || dst < 0.1 {
                    continue;
                }

                d = d.normalize() * 1. / (dst * dst);
                speed += d;
            }

            let prey_speed = beast.max_speed;

            if ((bx - x) * (bx - x) + (by - y) * (by - y)).sqrt() <= beast.player_detection_radius {
                speed.x += if x > bx { -prey_speed } else { prey_speed };
                speed.y += if y > by { -prey_speed } else { prey_speed };
            }

            speed = speed.normalize() * prey_speed;

            cmd.x = speed.x;
            cmd.y = speed.y;
        }
    }
}
