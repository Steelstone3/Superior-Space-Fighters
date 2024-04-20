use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter, OnExit},
};

use crate::{
    states::core_states::GameState,
    systems::user_interface::{
        main_menu_user_interface::{
            despawn_main_menu_user_interface, spawn_main_menu_user_interface,
        },
        pause_menu_user_interface::{
            despawn_pause_menu_user_interface, spawn_pause_menu_user_interface,
        },
        user_interface_main_menu_button_update::user_interface_main_menu_button_update,
    },
};

pub struct UserInterfaceMenusUpdatePlugin;

impl Plugin for UserInterfaceMenusUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu_user_interface)
            .add_systems(
                OnExit(GameState::MainMenu),
                despawn_main_menu_user_interface,
            )
            .add_systems(
                OnEnter(GameState::PauseMenu),
                spawn_pause_menu_user_interface,
            )
            .add_systems(
                OnExit(GameState::PauseMenu),
                despawn_pause_menu_user_interface,
            )
            .add_systems(
                Update,
                user_interface_main_menu_button_update.run_if(in_state(GameState::MainMenu)),
            );
    }
}
