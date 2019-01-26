use crate::components;

use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{
        PngFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
        TextureMetadata, Transparent,
    },
};

pub fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/hunter.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprites/hunter.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

pub fn initialise_hunter(
    world: &mut World,
    sprite_sheet_handle: SpriteSheetHandle,
    x: f32,
    y: f32,
) {
    let mut transform = Transform::default();
    transform.set_xyz(x, y, -y);

    const HUNTER_SIZE: f32 = 2.;
    let scale = HUNTER_SIZE / 16.;
    transform.set_scale(scale, scale, scale);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    world.register::<components::Velocity>();
    world.register::<components::VelocityCmd>();
    world.register::<components::BoundingRect>();

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(components::BoundingRect {
            width: HUNTER_SIZE,
            height: HUNTER_SIZE,
        })
        .with(transform)
        .with(Transparent)
        .with(components::Velocity {
            x: 0.,
            y: 0.,
            z: 0.,
        })
        .with(components::VelocityCmd { x: 0., y: 0. })
        .build();
}
