use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        PngFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
        TextureMetadata,
    },
    utils::application_root_dir,
};

pub const HUNTER_HEIGHT: f32 = 16.0;
pub const HUNTER_WIDTH: f32 = 4.0;

pub struct Hunter {
    pub width: f32,
    pub height: f32,
}

impl Hunter {
    fn new() -> Hunter {
        Hunter {
            width: HUNTER_WIDTH,
            height: HUNTER_HEIGHT,
        }
    }
}

impl Component for Hunter {
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

pub fn initialise_hunter(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let mut transform = Transform::default();

    // Correctly position the hunters.
    let y = 20.0;
    transform.set_xyz(HUNTER_WIDTH * 0.5, y, 0.0);

    // Assign the sprites for the hunters
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // hunter is the first sprite in the sprite_sheet
    };

    // Create a hunter entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Hunter::new())
        .with(transform)
        .build();
}
