use crate::components;

use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{
        PngFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
        TextureMetadata, Transparent,
    },
    utils::application_root_dir,
};

pub fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}/resources/sprites/beast.png", application_root_dir()),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}/resources/sprites/beast.ron", application_root_dir()), // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

pub fn initialise_beast(
    world: &mut World,
    sprite_sheet_handle: SpriteSheetHandle,
    xs: &[f32],
    ys: &[f32],
) {
    for (x, y) in xs.iter().zip(ys.iter()) {
        let mut transform = Transform::default();
        transform.set_xyz(*x, *y, -*y);

        const BEAST_SIZE: f32 = 2.;
        let scale = BEAST_SIZE / 16.;
        transform.set_scale(scale, scale, scale);

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0,
        };

        world.register::<components::Velocity>();
        world.register::<components::VelocityCmd>();

        world
            .create_entity()
            .with(sprite_render.clone())
            .with(components::BoundingRect {
                width: BEAST_SIZE,
                height: BEAST_SIZE,
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
}
