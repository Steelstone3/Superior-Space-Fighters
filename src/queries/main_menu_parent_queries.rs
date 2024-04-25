use bevy::ecs::{entity::Entity, query::QueryData};

use crate::components::user_interface::main_menu_parent::MainMenuParent;

#[derive(QueryData)]
pub struct MainMenuParentEntityQuery {
    pub entity: Entity,
    pub weapon_selection_parent: &'static MainMenuParent,
}
