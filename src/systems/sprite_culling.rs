use crate::components::AutoHide;
use crate::hunter::Hunter;
use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, ReadStorage, System, WriteStorage};
use amethyst::renderer::Hidden;

pub struct SpriteCullingSystem;

impl<'a> System<'a> for SpriteCullingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Hunter>,
        ReadStorage<'a, AutoHide>,
        WriteStorage<'a, Hidden>,
    );

    fn run(&mut self, (entities, transforms, hunters, hideable, mut hidden): Self::SystemData) {
        for (_hunter, hunter_transform, _) in (&*entities, &transforms, &hunters).join() {
            for (e, tree_transform, _) in (&*entities, &transforms, &hideable).join() {
                if (tree_transform.translation().x - hunter_transform.translation().x).abs() > 25.
                    || (tree_transform.translation().y - hunter_transform.translation().y).abs()
                        > 25.
                {
                    hidden.insert(e, Hidden).ok();
                } else {
                    hidden.remove(e);
                }
            }
        }
    }
}
