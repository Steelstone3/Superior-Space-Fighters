use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
    prelude::OnExit,
};

use crate::{
    events::user_interface_events::NewGameEvent,
    plugins::run_conditions::event_called,
    states::core_states::GameState,
    systems::{
        planets::spawn_planet::spawn_random_planets,
        space::{
            spawn_space::spawn_random_empty_space_background,
            spawn_space_sprite_on_load::spawn_space_sprite_on_load,
        },
        stations::{
            spawn_station::spawn_random_station,
            spawn_station_sprite_on_load::spawn_station_sprite_on_load,
        },
    },
};

pub struct SpawnSpacePlugin;

impl Plugin for SpawnSpacePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            OnExit(GameState::MainMenu),
            (
                spawn_random_empty_space_background,
                spawn_random_station,
                spawn_random_planets,
            )
                .run_if(event_called::<NewGameEvent>),
        )
        .add_systems(
            Update,
            (spawn_space_sprite_on_load, spawn_station_sprite_on_load),
        );
    }
}
