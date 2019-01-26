use amethyst::{
    assets::Loader,
    ecs::Entity,
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

/// ScoreText contains the ui text components that display the score
pub struct ScoreText {
    pub timer: Entity,
}

pub fn initialise_score(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );
    let transform = UiTransform::new(
        "Timer".to_string(),
        Anchor::TopMiddle,
        -50.,
        -50.,
        1.,
        200.,
        50.,
        0,
    );

    let timer = world
        .create_entity()
        .with(transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    world.add_resource(ScoreText { timer });
}
