use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entities, Join, System, ReadStorage, WriteStorage, Read};
use amethyst::core::timing::Time;

use crate::components::Bullet;

pub struct BulletSystem;

impl<'s> System<'s> for BulletSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Bullet>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, bullets, mut transforms, time): Self::SystemData) {
        // Scan through the list of bullets and move them forward.
        for (bullet_entity, _bullet_component, bullet_transform) in (&*entities, &bullets, &mut transforms).join() {
            bullet_transform.translate_y( * time.delta_seconds());

            // Delete when off the screen
            if bullet_transform.translation()[1] > 1024. {
                let _result = entities.delete(bullet_entity);
            }
        }
    }
}
