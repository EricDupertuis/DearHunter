mod gamestate;
mod hunter;
mod tree;
mod systems;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA, DepthMode},
    utils::application_root_dir,
    input::InputBundle
};
use gamestate::GameState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir();

    let path = format!("{}/resources/display_config.ron", app_root);
    let config = DisplayConfig::load(&path);

    let binding_path = format!(
        "{}/resources/bindings_config.ron",
        application_root_dir()
    );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 20.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, Some(DepthMode::LessEqualWrite)))
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&["transform_system"]))?
        .with_bundle(input_bundle)?
        .with(systems::HunterSystem, "paddle_system", &["input_system"]);

    let assets_dir = format!("{}/assets/", app_root);

    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();
    Ok(())
}
