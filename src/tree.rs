use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, NullStorage},
    prelude::*,
    renderer::{
        PngFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
        TextureMetadata, Transparent,
    },
};

use rand::Rng;

use crate::components;

pub struct Tree {}
impl Component for Tree {
    type Storage = NullStorage<Self>;
}
impl Default for Tree {
    fn default() -> Tree {
        Tree {}
    }
}

pub fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/tree.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprites/tree.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

pub fn initialise_tree(world: &mut World, sprite_sheet_handle: SpriteSheetHandle, x: f32, y: f32) {
    let mut transform = Transform::default();
    transform.set_xyz(x, y, -y);

    const TREE_SIZE: f32 = 4.;
    let scale = TREE_SIZE / 256.;

    transform.set_scale(scale, scale, scale);

    let mut rng = rand::thread_rng();
    let sprite_number = rng.gen_range(0, 4); // TODO: How many trees

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: sprite_number,
    };

    world.register::<Tree>();
    world.register::<components::BoundingRect>();

    world
        .create_entity()
        .with(Tree {})
        .with(sprite_render.clone())
        .with(components::BoundingRect {
            width: TREE_SIZE / 4.,
            height: TREE_SIZE / 2.,
        })
        .with(transform)
        .with(Transparent)
        .with(components::AutoHide {})
        .build();
}
