mod beast;
mod components;
mod config;
mod states;
mod home;
mod hunter;
mod score;
mod systems;
mod tree;
mod voronoi;

use config::GameConfig;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        ColorMask, DepthMode, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA,
    },
    ui::{DrawUi, UiBundle},
    utils::application_root_dir,
};
use states::StartState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir();

    let path = format!("{}/resources/display_config.ron", app_root);
    let config = DisplayConfig::load(&path);

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());

    let game_config = {
        let file = format!("{}/resources/game_config.ron", application_root_dir());
        GameConfig::load(&file)
    };

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 20.0)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            ))
            .with_pass(DrawUi::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(
            systems::SpriteCullingSystem,
            "sprite_culling",
            &["transform_system"],
        )
        .with(
            systems::MoveCmdSystem,
            "input_cmd_system",
            &["input_system"],
        )
        .with(
            systems::HunterSpriteSwitcher,
            "hunter_sprite_system",
            &["input_cmd_system"],
        )
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[
                    "hunter_sprite_system",
                    "transform_system",
                    "sprite_culling",
                ]),
        )?
        .with(
            systems::CollisionSystem,
            "collision_system",
            &["input_cmd_system"],
        )
        .with(
            systems::RepulsionSystem,
            "repulsion_system",
            &["collision_system"],
        )
        .with(
            systems::MoveSystem,
            "movement_system",
            &["collision_system"],
        )
        .with(systems::BehaviorSystem, "behavior_system", &[])
        .with(
            systems::ShootCmdSystem,
            "shoot_cmd_system",
            &["input_system"],
        )
        .with(
            systems::GoingHomeSystem,
            "going_home_system",
            &["movement_system"],
        )
        .with(systems::TimerSystem, "timer_system", &[]);

    // Base path where we look for assets/textures/sprites
    let assets_dir = format!("{}/resources/", app_root);

    let mut game = Application::build(assets_dir, StartState)?
        .with_resource(game_config)
        .build(game_data)?;

    game.run();
    Ok(())
}
