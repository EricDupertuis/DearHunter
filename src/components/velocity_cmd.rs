use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct VelocityCmd {
    pub x: f32,
    pub y: f32,
}

impl Component for VelocityCmd {
    type Storage = DenseVecStorage<Self>;
}