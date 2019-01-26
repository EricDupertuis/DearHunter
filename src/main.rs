mod gamestate;
mod hunter;
mod beast;
mod tree;
mod systems;
mod velcomp;
mod voronoi;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        ColorMask, DepthMode, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA,
    },
    utils::application_root_dir,
};
use gamestate::GameState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir();

    let path = format!("{}/resources/display_config.ron", app_root);
    let config = DisplayConfig::load(&path);

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 20.0)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            )),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&["transform_system"]),
        )?
        .with_bundle(input_bundle)?
        .with(
            systems::MoveCmdSystem,
            "input_cmd_system",
            &["input_system"],
        )
        .with(
            systems::CollisionSystem,
            "collision_system",
            &["input_cmd_system"],
        )
        .with(
            systems::MoveSystem,
            "movement_system",
            &["collision_system"],
        );

    // Base path where we look for assets/textures/sprites
    let assets_dir = format!("{}/resources/", app_root);

    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();
    Ok(())
}
