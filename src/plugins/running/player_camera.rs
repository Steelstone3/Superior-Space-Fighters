use bevy::app::{Plugin, Update};

use crate::systems::camera::{
    camera_movement::camera_movement, increment_camera::increment_camera,
    scroll_camera::scroll_camera,
};

pub struct PlayerCamera;

impl Plugin for PlayerCamera {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, camera_movement)
            .add_systems(Update, scroll_camera)
            .add_systems(Update, increment_camera);
    }
}
