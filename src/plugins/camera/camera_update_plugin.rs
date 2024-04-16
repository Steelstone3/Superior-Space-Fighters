use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    plugins::run_conditions::run_if_not_paused,
    systems::camera::{
        camera_movement::camera_movement, increment_camera::increment_camera,
        scroll_camera::scroll_camera,
    },
};

pub struct CameraUpdatePlugin;

impl Plugin for CameraUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, camera_movement.run_if(run_if_not_paused))
            .add_systems(Update, scroll_camera.run_if(run_if_not_paused))
            .add_systems(Update, increment_camera.run_if(run_if_not_paused));
    }
}
