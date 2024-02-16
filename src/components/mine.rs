use super::weapon::Weapon;
use crate::assets::{
    images::starships::weapons::mines::MineSprite, sounds::starships::weapons::mines::MineSound,
};
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Mine {
    pub mine: MineSprite,
    pub sound: MineSound,
    pub weapon: Weapon,
}
