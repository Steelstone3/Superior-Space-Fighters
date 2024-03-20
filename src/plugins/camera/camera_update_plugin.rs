use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{
    states::core_states::GameState,
    systems::camera::{
        camera_movement::camera_movement, increment_camera::increment_camera,
        scroll_camera::scroll_camera,
    },
};

pub struct CameraUpdatePlugin;

impl Plugin for CameraUpdatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (camera_movement, scroll_camera, increment_camera).run_if(in_state(GameState::InGame)),
        );
    }
}
