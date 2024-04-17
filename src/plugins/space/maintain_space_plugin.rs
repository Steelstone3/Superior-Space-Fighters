use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{states::core_states::GameState, systems::space::move_empty_space::move_empty_space};

pub struct MaintainSpacePlugin;

impl Plugin for MaintainSpacePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, move_empty_space.run_if(in_state(GameState::InGame)));
    }
}
