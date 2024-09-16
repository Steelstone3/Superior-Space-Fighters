use bevy::app::{Plugin, Startup};

use crate::systems::camera::spawn_player_camera::spawn_player_camera;

pub struct CameraStartupPlugin;

impl Plugin for CameraStartupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player_camera);
    }
}
