use bevy::app::{Plugin, Startup};

use crate::systems::player::spawn_player::spawn_player_ship;

pub struct PlayerStartPlugin;

impl Plugin for PlayerStartPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player_ship);
    }
}
