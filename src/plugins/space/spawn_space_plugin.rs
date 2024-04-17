use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{
    states::core_states::GameState,
    systems::{
        planets::spawn_planet::spawn_random_planets,
        space::spawn_space::spawn_random_empty_space_background,
        stations::spawn_station::spawn_random_station,
    },
};

pub struct SpawnSpacePlugin;

impl Plugin for SpawnSpacePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                spawn_random_empty_space_background,
                spawn_random_station,
                spawn_random_planets,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
