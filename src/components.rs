use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

pub struct VelocityCmd {
    pub x: f32,
    pub y: f32,
}

impl Component for VelocityCmd {
    type Storage = DenseVecStorage<Self>;
}

pub struct BoundingRect {
    pub width: f32,
    pub height: f32,
}

impl Component for BoundingRect {
    type Storage = DenseVecStorage<Self>;
}

pub struct ShootCmd {
    pub x: f64,
    pub y: f64,
    pub speed: f32,
}

impl Component for ShootCmd {
    type Storage = DenseVecStorage<Self>;
}
