use bevy::{app::Plugin, ecs::schedule::OnExit};

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
            OnExit(GameState::MainMenu),
            (
                spawn_random_empty_space_background,
                spawn_random_station,
                spawn_random_planets,
            ),
        );
    }
}
