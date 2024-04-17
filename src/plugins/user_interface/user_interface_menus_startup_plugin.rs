use bevy::{
    app::{Plugin, Startup},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{
    states::core_states::GameState, systems::user_interface::draw_main_menu::draw_main_menu,
};

pub struct UserInterfaceMenusStartupPlugin;

impl Plugin for UserInterfaceMenusStartupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Startup,
            draw_main_menu.run_if(in_state(GameState::MainMenu)),
        );
    }
}
