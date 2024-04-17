use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnExit},
};

use crate::{
    states::core_states::GameState,
    systems::starships::{
        despawn_starships::despawn_destoryed_starships, spawn_starships::spawn_random_starships,
        starship_movement::ai_movement,
    },
};

pub struct AIUpdatePlugin;

impl Plugin for AIUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnExit(GameState::MainMenu), (spawn_random_starships,))
            .add_systems(
                Update,
                (ai_movement, despawn_destoryed_starships).run_if(in_state(GameState::InGame)),
            );
    }
}
