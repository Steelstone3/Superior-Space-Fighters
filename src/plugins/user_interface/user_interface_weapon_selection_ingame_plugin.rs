use bevy::{app::Plugin, ecs::schedule::OnEnter};

use crate::{
    states::core_states::GameState,
    systems::user_interface::initialize_ingame_interface::initialize_ingame_interface,
};

pub struct UserInterfaceWeaponSelectionInGamePlugin;

impl Plugin for UserInterfaceWeaponSelectionInGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::InGame), initialize_ingame_interface);
    }
}
