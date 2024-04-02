use bevy::app::{Plugin, Startup, Update};

use crate::systems::{
    player::spawn_player::spawn_player_ship,
    starships::{
        despawn_starships::despawn_destoryed_starships, spawn_starships::spawn_random_starships,
    },
};

pub struct Spawning;

impl Plugin for Spawning {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player_ship)
            .add_systems(Startup, spawn_random_starships)
            .add_systems(Update, despawn_destoryed_starships);
    }
}
