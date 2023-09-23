use bevy::prelude::{Commands, Camera2dBundle};

pub fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}