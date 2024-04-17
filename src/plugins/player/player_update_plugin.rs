use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnExit},
};

use crate::{
    states::core_states::GameState,
    systems::player::{
        player_movement::player_movement, player_pause_resume_game::player_pause_resume,
        spawn_player::spawn_player_ship,
    },
};

pub struct PlayerUpdatePlugin;

impl Plugin for PlayerUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnExit(GameState::MainMenu), spawn_player_ship)
            .add_systems(Update, player_movement.run_if(in_state(GameState::InGame)))
            .add_systems(Update, player_pause_resume);
    }
}
