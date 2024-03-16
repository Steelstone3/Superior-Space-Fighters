use crate::systems::{
    camera::spawn_player_camera::spawn_player_camera,
    music::exploration_music::play_exploration_music, planets::spawn_planet::spawn_random_planets,
    player::spawn_player::spawn_player_ship, ships::spawn_ships::spawn_random_ships,
    space::spawn_space::spawn_random_empty_space_background,
    stations::spawn_station::spawn_random_station,
    weapons::targetting::spawn_targetting_settings::spawn_targetting_setting,
};
use bevy::prelude::{App, Plugin, Startup};

pub struct GameStartPlugin;

impl Plugin for GameStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_targetting_setting)
            .add_systems(Startup, play_exploration_music)
            .add_systems(Startup, spawn_random_empty_space_background)
            .add_systems(Startup, spawn_player_camera)
            .add_systems(Startup, spawn_random_planets)
            .add_systems(Startup, spawn_random_station)
            .add_systems(Startup, spawn_random_ships)
            .add_systems(Startup, spawn_player_ship);
    }
}
