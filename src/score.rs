use amethyst::{
    assets::Loader,
    ecs::{Entity},
    prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub struct GameTimer {
    pub timer: f32,
    pub active: bool,
}
impl Default for GameTimer {
    fn default() -> Self {
        GameTimer {
            timer: 22.,
            active: false,
        }
    }
}

pub fn initialise_score(world: &mut World) {
    world.register::<UiText>();
    world.register::<UiTransform>();

    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );

    let transform = UiTransform::new(
        "Timer".to_string(),
        Anchor::TopLeft,
        50.,
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
            "".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    world.add_resource(ScoreText { timer });
}

pub struct ScoreText {
    pub timer: Entity,
}
