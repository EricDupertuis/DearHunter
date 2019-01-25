extern crate rand;
use crate::hunter;
use crate::tree;
use rand::Rng;

use amethyst::{
    core::transform::Transform,
    core::nalgebra::Orthographic3,
    prelude::*,
    renderer::{Camera, Projection},
};

pub struct GameState;

pub const ARENA_WIDTH: f32 = 50.0;
pub const ARENA_HEIGHT: f32 = ARENA_WIDTH * 1080. / 1920.;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::from(Projection::Orthographic(Orthographic3::new(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
            0.0, // near plane
            20., // far plane
        ))))
        .with(transform)
        .build();
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.
        let hunter_sprite = hunter::load_sprite_sheet(world);
        let tree_sprite = tree::load_sprite_sheet(world);

        world.register::<hunter::Hunter>();
        world.register::<tree::Tree>();

        hunter::initialise_hunter(world, hunter_sprite);

        let mut rng = rand::thread_rng();

        for _ in 1..100 {
            let x = (rng.gen::<f32>()) * 30.;
            let y = (rng.gen::<f32>()) * 30.;
            tree::initialise_tree(world, tree_sprite.clone(), x, y);
        }

        initialise_camera(world);
    }
}
