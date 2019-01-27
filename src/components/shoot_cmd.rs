use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct ShootCmd {
    pub x: f64,
    pub y: f64,
    pub speed: f32,
}

impl Component for ShootCmd {
    type Storage = DenseVecStorage<Self>;
}
