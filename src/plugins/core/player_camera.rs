use bevy::{
    app::{Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::camera::{
        camera_movement::camera_movement, increment_camera::increment_camera,
        scroll_camera::scroll_camera, spawn_player_camera::spawn_player_camera,
    },
};

pub struct PlayerCamera;

impl Plugin for PlayerCamera {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player_camera)
            .add_systems(Update, camera_movement.run_if(run_if_not_paused))
            .add_systems(Update, scroll_camera.run_if(run_if_not_paused))
            .add_systems(Update, increment_camera.run_if(run_if_not_paused));
    }
}
