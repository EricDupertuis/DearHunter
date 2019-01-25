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

const TREE_HEIGHT: f32 = 64.;
const TREE_WIDTH: f32 = 64.;

pub struct Tree {
    pub width: f32,
    pub height: f32,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            width: TREE_WIDTH,
            height: TREE_HEIGHT,
        }
    }
}

impl Component for Tree {
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
            format!("{}/resources/sprites/tree.png", application_root_dir()),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}/resources/sprites/tree.ron", application_root_dir()), // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

pub fn initialise_tree(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let mut transform = Transform::default();

    transform.set_xyz(20., 20., 0.);

    let scale = 8. / 138.;
    transform.set_scale(scale, scale, scale);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Tree::new())
        .with(transform)
        .build();
}
