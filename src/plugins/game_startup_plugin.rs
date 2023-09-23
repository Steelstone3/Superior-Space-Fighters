use crate::systems::{camera::add_camera, spawn_sprite::spawn_sprite};
use bevy::prelude::{App, Plugin, Startup};

pub struct GameStartupPlugin;

impl Plugin for GameStartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera)
            .add_systems(Startup, spawn_sprite);
    }
}
