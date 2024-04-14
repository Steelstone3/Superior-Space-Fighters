use bevy::app::{Plugin, Startup};

use crate::systems::player::spawn_player::spawn_player_ship;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player_ship);
    }
}
