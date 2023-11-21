use bevy::{
    ecs::component::Component,
    prelude::Vec2,
};

use crate::assets::images::space::SpaceSprite;

#[derive(Component)]
pub struct Space {
    pub asset: SpaceSprite,
    pub size: Vec2,
}
