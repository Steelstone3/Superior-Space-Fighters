use bevy::{ecs::component::Component, transform::components::Transform};
use crate::components::starships::starship::Starship;
use super::ranged_weapon::RangedWeapon;

// TODO idea for targetting system
#[derive(Component, Debug, PartialEq)]
pub struct LockOnWeapon {
    pub target: (Transform, Starship),
    pub weapon: RangedWeapon,
}