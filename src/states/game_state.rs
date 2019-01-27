extern crate rand;
use crate::audio;
use crate::beast;
use crate::config::GameConfig;
use crate::game_termination::GameTermination;
use crate::home;
use crate::hunter;
use crate::score;
use crate::states::{LoseState, WinState};
use crate::tree;
use crate::voronoi;

use rand::Rng;

use amethyst::{
    core::nalgebra::{Orthographic3, Point2},
    core::transform::Transform,
    core::Parent,
    ecs::Entity,
    prelude::*,
    renderer::{Camera, Projection},
};

pub struct GameState;

pub const ARENA_WIDTH: f32 = 500.0;
pub const ARENA_HEIGHT: f32 = ARENA_WIDTH * 1080. / 1920.;

pub const CAMERA_WIDTH: f32 = 50.0;
pub const CAMERA_HEIGHT: f32 = CAMERA_WIDTH * 1080. / 1920.;

fn initialise_camera(world: &mut World, parent: Entity) {
    let mut transform = Transform::default();

    // Keep it synced with the hunter
    let scale = 256. / 2.;

    // In case of disappearing sprites, check far plane and z camera transform
    transform.set_xyz(
        -(CAMERA_WIDTH / 2.) * scale,
        -(CAMERA_HEIGHT / 2.) * scale,
        CAMERA_HEIGHT * scale,
    );
    transform.set_scale(scale, scale, scale);

    world
        .create_entity()
        .with(Camera::from(Projection::Orthographic(Orthographic3::new(
            0.0,
            CAMERA_WIDTH,
            0.0,
            CAMERA_HEIGHT,
            0.0,                 // near plane
            CAMERA_HEIGHT * 10., // far plane. z_depth = height since we're using an inclined plane to show depth in 2D.
        ))))
        .with(Parent { entity: parent })
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
        let home_sprite = home::load_sprite_sheet(world);

        let (home_x, home_y) = {
            let home_config = &world.read_resource::<GameConfig>().home;
            (home_config.x, home_config.y)
        };
        let _home = home::initialise_home(
            world,
            home_sprite,
            ARENA_WIDTH * home_x,
            ARENA_HEIGHT * home_y,
        );

        let (hunter_start_x, hunter_start_y) = {
            let hunter_config = &world.read_resource::<GameConfig>().hunter;
            (hunter_config.start_x, hunter_config.start_y)
        };
        let hunter = hunter::initialise_hunter(
            world,
            hunter_sprite,
            ARENA_WIDTH * hunter_start_x,
            ARENA_HEIGHT * hunter_start_y,
        );

        let mut rng = rand::thread_rng();

        let nb_beast = {
            let beast_config = &world.read_resource::<GameConfig>().beast;
            beast_config.count
        };
        let bw: Vec<f32> = (1..nb_beast)
            .map(|_| ARENA_WIDTH * rng.gen_range(0., 1.))
            .collect();
        let bh: Vec<f32> = (1..nb_beast)
            .map(|_| ARENA_HEIGHT * rng.gen_range(0., 1.))
            .collect();
        beast::initialise_beast(world, beast_sprite, &bw, &bh);

        let (tree_count, centroid_count, path_width, start_radius) = {
            let forest_config = &world.read_resource::<GameConfig>().forest;
            (
                forest_config.tree_count,
                forest_config.centroid_count,
                forest_config.path_width,
                forest_config.start_zone_radius,
            )
        };

        let points = voronoi::generate_voronoi(
            tree_count,
            centroid_count,
            path_width,
            vec![
                voronoi::ClearRegion {
                    center: Point2::new(0.5, 0.5),
                    radius: start_radius,
                },
                voronoi::ClearRegion {
                    center: Point2::new(1.0, 1.0),
                    radius: start_radius,
                },
            ],
        );

        for p in points.iter() {
            let x = p.x * ARENA_WIDTH;
            let y = p.y * ARENA_HEIGHT;
            tree::initialise_tree(world, tree_sprite.clone(), x, y);
        }

        initialise_camera(world, hunter);
        audio::change_track(world, audio::MusicTracks::Game);
        score::initialise_score(world);

        let time_to_go_home = {
            let hunter_config = &world.read_resource::<GameConfig>().hunter;
            hunter_config.time_to_go_home
        };
        world.write_resource::<score::GameTimer>().timer = time_to_go_home;
        world.write_resource::<score::GameTimer>().active = true;
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Disable the timer
        world.write_resource::<score::GameTimer>().active = false;

        // Stop the music
        audio::change_track(world, audio::MusicTracks::Silent);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let (won, eaten, timeout) = {
            let termination = &data.world.read_resource::<GameTermination>();
            (
                termination.reached_home,
                termination.eaten,
                termination.timeout,
            )
        };

        if won {
            data.world.delete_all();
            return Trans::Switch(Box::new(WinState));
        }

        if eaten || timeout {
            data.world.delete_all();
            return Trans::Switch(Box::new(LoseState));
        }

        Trans::None
    }
}
