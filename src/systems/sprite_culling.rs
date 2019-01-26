use crate::hunter::Hunter;
use crate::tree::Tree;
use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use amethyst::renderer::{Hidden, SpriteVisibility};

pub struct SpriteCullingSystem;

impl<'a> System<'a> for SpriteCullingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        Write<'a, SpriteVisibility>,
        ReadStorage<'a, Hunter>,
        ReadStorage<'a, Tree>,
        WriteStorage<'a, Hidden>,
    );

    fn run(
        &mut self,
        (entities, transforms, mut visibility, hunters, trees, mut hidden): Self::SystemData,
    ) {
        for (hunter_e, hunter_transform, _) in (&*entities, &transforms, &hunters).join() {
            for (e, tree_transform, _) in (&*entities, &transforms, &trees).join() {
                if (tree_transform.translation().x - hunter_transform.translation().x).abs() > 20.
                    || (tree_transform.translation().y - hunter_transform.translation().y).abs()
                        > 20.
                {
                    hidden.insert(e, Hidden);
                } else {
                    hidden.remove(e);
                }
            }
        }
    }
}
