use bevy::{
    app::{Plugin, Update},
    prelude::{in_state, IntoSystemConfigs, OnEnter},
};

use crate::{
    events::user_interface_events::NewGameEvent,
    plugins::run_conditions::event_called,
    states::core_states::GameState,
    systems::starships::{
        despawn_starships::despawn_destoryed_starships, spawn_starships::spawn_random_starships,
        starship_movement::ai_movement,
    },
};

pub struct AIUpdatePlugin;

impl Plugin for AIUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            spawn_random_starships.run_if(event_called::<NewGameEvent>),
        )
        .add_systems(
            Update,
            (ai_movement, despawn_destoryed_starships).run_if(in_state(GameState::InGame)),
        );
    }
}
