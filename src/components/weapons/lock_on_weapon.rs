use super::ranged_weapon::RangedWeapon;
use crate::components::starships::starship::Starship;
use bevy::{ecs::component::Component, transform::components::Transform};

// TODO idea for targetting system
#[derive(Component, Debug, PartialEq)]
pub struct LockOnWeapon {
    pub target: (Transform, Starship),
    pub weapon: RangedWeapon,
}
