use crate::components;
use crate::config::GameConfig;

use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        PngFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
        TextureMetadata, Transparent,
    },
    utils::application_root_dir,
};

#[derive(Default)]
pub struct Beast {
    pub max_speed: f32,
    pub player_detection_radius: f32,
    pub tree_detection_radius: f32,
}
impl Component for Beast {
    type Storage = DenseVecStorage<Self>;
}

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

        const BEAST_SIZE: f32 = 4.;
        let scale = BEAST_SIZE / 256.;
        transform.set_scale(scale, scale, scale);

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0,
        };

        world.register::<Beast>();
        world.register::<components::Velocity>();
        world.register::<components::VelocityCmd>();
        world.register::<components::BoundingRect>();

        let (prey_speed, player_detection_radius, tree_detection_radius) = {
            let beast_config = &world.read_resource::<GameConfig>().beast;
            (
                beast_config.prey_speed,
                beast_config.player_detection_radius,
                beast_config.tree_detection_radius,
            )
        };

        world
            .create_entity()
            .with(Beast {
                max_speed: prey_speed,
                player_detection_radius: player_detection_radius,
                tree_detection_radius: tree_detection_radius,
            })
            .with(sprite_render.clone())
            .with(components::BoundingRect {
                width: BEAST_SIZE * 0.8,
                height: BEAST_SIZE * 0.8,
            })
            .with(transform)
            .with(Transparent)
            .with(components::Velocity {
                x: 0.,
                y: 0.,
                z: 0.,
            })
            .with(components::VelocityCmd { x: 0., y: 0. })
            .with(components::AutoHide {})
            .build();
    }
}
