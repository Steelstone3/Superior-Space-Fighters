use bevy::app::{Plugin, Startup};

use crate::systems::{
    player::spawn_player::spawn_player_ship, ships::spawn_ships::spawn_random_ships,
};

pub struct Spawning;

impl Plugin for Spawning {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player_ship)
            .add_systems(Startup, spawn_random_ships);
    }
}
