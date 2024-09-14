use bevy::{app::Plugin, prelude::OnEnter};

use crate::{
    states::core_states::GameState, systems::camera::spawn_player_camera::spawn_player_camera,
};

pub struct CameraStartupPlugin;

impl Plugin for CameraStartupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_player_camera);
    }
}
