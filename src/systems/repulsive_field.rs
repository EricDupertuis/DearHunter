use amethyst::{
    core::nalgebra::Vector2,
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::Velocity;
use crate::hunter::Hunter;
use crate::tree::Tree;

const D_RADIUS: f32 = 4.;

pub struct RepulsionSystem;
impl<'s> System<'s> for RepulsionSystem {
    type SystemData = (
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Tree>,
        ReadStorage<'s, Hunter>,
    );

    fn run(&mut self, (mut velocities, transforms, trees, hunter): Self::SystemData) {
        for (_hunter, htrans, hvel) in (&hunter, &transforms, &mut velocities).join() {
            let mut speed = Vector2::new(0., 0.);
            let hunterv = Vector2::new(htrans.translation().x, htrans.translation().y);

            for (_tree, ttrans) in (&trees, &transforms).join() {
                let treev = Vector2::new(ttrans.translation().x, ttrans.translation().y);
                let mut d = hunterv - treev;
                let mut dst = d.norm();
                if dst < 0.01 {
                    dst = 0.01;
                }
                if dst < D_RADIUS {
                    d = d.normalize() * 4. / (dst * dst);
                    speed += d;
                }
            }
            hvel.x += speed.x;
            hvel.y += speed.y;
        }
    }
}
