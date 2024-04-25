use bevy::ecs::{entity::Entity, query::QueryData};

use crate::components::user_interface::pause_menu_parent::PauseMenuParent;

#[derive(QueryData)]
pub struct PauseMenuParentEntityQuery {
    pub entity: Entity,
    pub weapon_selection_parent: &'static PauseMenuParent,
}
