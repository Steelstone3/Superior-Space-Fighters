use crate::systems::{
    camera::spawn_player_camera::spawn_player_camera, planets::spawn_planet::spawn_random_planets,
    player::spawn_player::spawn_player_ship, ships::spawn_ships::spawn_random_ships,
    stations::spawn_station::spawn_random_station,
};
use bevy::prelude::{App, Plugin, Startup};

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_camera)
            .add_systems(Startup, spawn_random_planets)
            .add_systems(Startup, spawn_random_station)
            .add_systems(Startup, spawn_random_ships)
            .add_systems(Startup, spawn_player_ship);
    }
}
