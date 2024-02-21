use crate::assets::images::space::SpaceSprite;
use bevy::{ecs::component::Component, math::Vec3, prelude::Vec2};

#[derive(Component, Default)]
pub struct Space {
    pub asset: SpaceSprite,
    pub size: Vec2,
    pub grid_position: Vec2,
    pub location: Vec3,
}

#[cfg(test)]
mod space_should {
    #[test]
    #[ignore]
    fn create_new() {}
}
