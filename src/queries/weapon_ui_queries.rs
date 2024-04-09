use bevy::ecs::query::{QueryFilter, With};

use crate::components::user_interface::weapon_selection_parent::WeaponSelectionParent;

#[derive(QueryFilter)]
pub struct WeaponUiUpdateFilter {
    with: With<WeaponSelectionParent>,
}
