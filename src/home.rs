use crate::components;

use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, NullStorage},
    ecs::Entity,
    prelude::*,
    renderer::{
        PngFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
        TextureMetadata, Transparent,
    },
};

pub struct Home {}
impl Component for Home {
    type Storage = NullStorage<Self>;
}
impl Default for Home {
    fn default() -> Home {
        Home {}
    }
}

pub fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/home.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprites/home.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

pub fn initialise_home(
    world: &mut World,
    sprite_sheet_handle: SpriteSheetHandle,
    x: f32,
    y: f32,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_xyz(x, y, -y);

    const HOME_WIDTH: f32 = 8.;
    const HOME_HEIGHT: f32 = HOME_WIDTH * 352. / 436.;
    transform.set_scale(HOME_WIDTH / 436., HOME_HEIGHT / 352., HOME_HEIGHT / 352.);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    world.register::<Home>();
    world.register::<components::Velocity>();
    world.register::<components::VelocityCmd>();
    world.register::<components::BoundingRect>();
    world.register::<components::ShootCmd>();

    world
        .create_entity()
        .with(Home {})
        .with(sprite_render.clone())
        .with(components::BoundingRect {
            width: HOME_WIDTH,
            height: HOME_HEIGHT,
        })
        .with(transform)
        .with(Transparent)
        .build()
}
