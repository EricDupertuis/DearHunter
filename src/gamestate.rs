extern crate rand;
use crate::hunter;
use crate::tree;
use crate::voronoi;
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

        world.register::<hunter::Hunter>();
        world.register::<tree::Tree>();

        hunter::initialise_hunter(world, hunter_sprite, ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5);

        let mut rng = rand::thread_rng();

        // TODO: Fetch this from RON
        let tree_cnt = 200;
        let centroid_cnt = 30;
        let path_width = 0.03;
        let points = voronoi::generate_voronoi(tree_cnt, centroid_cnt, path_width);

        for p in points.iter() {
            let x = p.x * ARENA_WIDTH;
            let y = p.y * ARENA_HEIGHT;
            tree::initialise_tree(world, tree_sprite.clone(), x, y);
        }

        initialise_camera(world);
    }
}
