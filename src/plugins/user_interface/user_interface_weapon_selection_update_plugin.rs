use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{
    plugins::run_conditions::resource_available,
    resources::selected_weapon::SelectedWeaponResource, states::core_states::GameState,
    systems::user_interface::weapon_selection_update::weapon_selection_update,
};

pub struct UserInterfaceWeaponSelectionUpdatePlugin;

impl Plugin for UserInterfaceWeaponSelectionUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (weapon_selection_update)
                .run_if(resource_available::<SelectedWeaponResource>)
                .run_if(in_state(GameState::InGame)),
        );
    }
}
