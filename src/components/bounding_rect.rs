use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct BoundingRect {
    pub width: f32,
    pub height: f32,
}

impl Component for BoundingRect {
    type Storage = DenseVecStorage<Self>;
}
