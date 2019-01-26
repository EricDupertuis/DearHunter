extern crate amethyst;
use crate::gamestate::GameState;
use amethyst::{
    assets::{AssetStorage, Loader},
    core::nalgebra::Orthographic3,
    core::transform::Transform,
    ecs::prelude::{Component, NullStorage},
    input::is_key_down,
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata, Transparent, VirtualKeyCode,
    },
};

pub struct StartState;

#[derive(Default)]
pub struct Title;

impl Component for Title {
    type Storage = NullStorage<Self>;
}

pub const CAMERA_WIDTH: f32 = 1920.;
pub const CAMERA_HEIGHT: f32 = 1080.;

pub fn initialise_title(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let transform = Transform::default();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world.register::<Title>();

    world
        .create_entity()
        .with(Title {})
        .with(sprite_render.clone())
        .with(transform)
        .with(Transparent)
        .build();
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();

    // In case of disappearing sprites, check far plane and z camera transform
    transform.set_xyz(-(CAMERA_WIDTH / 2.), -(CAMERA_HEIGHT / 2.), 1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::Orthographic(Orthographic3::new(
            0.0,
            CAMERA_WIDTH,
            0.0,
            CAMERA_HEIGHT,
            0.0,
            10.,
        ))))
        .with(transform)
        .build();
}

pub fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/title.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprites/title.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

impl SimpleState for StartState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite = load_sprite_sheet(world);
        initialise_title(world, sprite);
        initialise_camera(world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Space) {
                data.world.delete_all();
                return Trans::Switch(Box::new(GameState));
            }
        }
        Trans::None
    }
}
