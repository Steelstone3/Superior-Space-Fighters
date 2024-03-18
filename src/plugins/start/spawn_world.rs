use bevy::app::{Plugin, Startup};

use crate::systems::{
    camera::spawn_player_camera::spawn_player_camera, planets::spawn_planet::spawn_random_planets,
    space::spawn_space::spawn_random_empty_space_background,
    stations::spawn_station::spawn_random_station,
};

pub struct SpawnWorld;

impl Plugin for SpawnWorld {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player_camera)
            .add_systems(Startup, spawn_random_empty_space_background)
            .add_systems(Startup, spawn_random_station)
            .add_systems(Startup, spawn_random_planets);
    }
}
