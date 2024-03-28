use crate::components::user_interface::weapon_selection_parent::WeaponSelectionParent;
use bevy::ecs::{entity::Entity, query::QueryData};

#[derive(QueryData)]
pub struct WeaponSelectionParentEntityQuery {
    pub entity: Entity,
    pub weapon_selection_parent: &'static WeaponSelectionParent,
}
