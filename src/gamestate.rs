extern crate rand;
use crate::hunter;
use crate::tree;
use crate::beast;
use rand::Rng;

use amethyst::{
    core::nalgebra::Orthographic3,
    core::transform::Transform,
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
            0.0,          // near plane
            ARENA_HEIGHT, // far plane. z_depth = height since we're using an inclined plane to show depth in 2D.
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
        let beast_sprite = beast::load_sprite_sheet(world);

        world.register::<hunter::Hunter>();
        world.register::<tree::Tree>();
        world.register::<beast::Beast>();

        hunter::initialise_hunter(world, hunter_sprite, ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5);

        beast::initialise_beast(
            world,
            beast_sprite,
            &[ARENA_WIDTH * 0.2, ARENA_WIDTH * 0.4, ARENA_WIDTH * 0.8],
            &[ARENA_HEIGHT * 0.2, ARENA_HEIGHT * 0.4, ARENA_HEIGHT * 0.8]
            );



        let mut rng = rand::thread_rng();

        for _ in 1..100 {
            let x = (rng.gen::<f32>()) * ARENA_WIDTH;
            let y = (rng.gen::<f32>()) * ARENA_HEIGHT;
            tree::initialise_tree(world, tree_sprite.clone(), x, y);
        }

        initialise_camera(world);
    }
}
