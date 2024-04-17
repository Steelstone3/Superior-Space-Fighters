use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{
    states::core_states::GameState,
    systems::user_interface::{
        main_menu_user_interface::main_menu_user_interface,
        pause_menu_user_interface::pause_menu_user_interface,
        user_interface_main_menu_button_update::user_interface_main_menu_button_update,
    },
};

pub struct UserInterfaceMenusUpdatePlugin;

impl Plugin for UserInterfaceMenusUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (main_menu_user_interface,).run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            (
                pause_menu_user_interface,
                user_interface_main_menu_button_update,
            ),
        );
    }
}
