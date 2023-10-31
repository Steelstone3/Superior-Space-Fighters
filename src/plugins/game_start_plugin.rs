use crate::systems::{camera::add_camera, player::spawn_player::spawn_player_ship};
use bevy::prelude::{App, Plugin, Startup};

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera)
            .add_systems(Startup, spawn_player_ship);
    }
}
