use crate::velcomp;

use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        PngFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
        TextureMetadata, Transparent,
    },
};

pub struct Hunter {
    pub width: f32,
    pub height: f32,
}
impl Hunter {
    fn new() -> Hunter {
        Hunter {
            width: 2.0,
            height: 2.0,
        }
    }
}

impl Component for Hunter {
    type Storage = DenseVecStorage<Self>;
}

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

    let scale = 2. / 16.;
    transform.set_scale(scale, scale, scale);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    world.register::<velcomp::Velocity>();
    world.register::<velcomp::VelocityCmd>();

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Hunter::new())
        .with(transform)
        .with(Transparent)
        .with(velcomp::Velocity{x: 0., y: 0., z: 0.})
        .with(velcomp::VelocityCmd{x: 0., y: 0.})
        .build();
}
