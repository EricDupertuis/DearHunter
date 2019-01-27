use amethyst::ecs::prelude::{Component, NullStorage};

#[derive(Default)]
pub struct AutoHide {}

impl Component for AutoHide {
    type Storage = NullStorage<Self>;
}
