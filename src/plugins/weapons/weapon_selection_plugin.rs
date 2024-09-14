use crate::{
    states::core_states::GameState, systems::player::player_weapon_select::player_weapon_select,
};
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
    prelude::in_state,
};

pub struct WeaponSelectionPlugin;

impl Plugin for WeaponSelectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            player_weapon_select.run_if(in_state(GameState::InGame)),
        );
    }
}
