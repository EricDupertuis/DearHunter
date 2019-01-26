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

pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

pub struct VelocityCmd {
    pub x: f32,
    pub y: f32,
}

impl Component for VelocityCmd {
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
            format!("{}/resources/sprites/hunter.png", application_root_dir()),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}/resources/sprites/hunter.ron", application_root_dir()), // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the texture we want it to use
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

    world.register::<Velocity>();
    world.register::<VelocityCmd>();

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Hunter::new())
        .with(transform)
        .with(Transparent)
        .with(Velocity {
            x: 0.,
            y: 0.,
            z: 0.,
        })
        .with(VelocityCmd { x: 0., y: 0. })
        .build();
}
