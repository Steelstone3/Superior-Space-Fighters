use crate::resources::camera_settings::CameraSettings;
use bevy::prelude::{Camera, Input, KeyCode, OrthographicProjection, Query, Res, ResMut, With};

pub fn increment_camera(
    input: Res<Input<KeyCode>>,
    mut camera_projection_query: Query<&mut OrthographicProjection, With<Camera>>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    // for (mut transform, mut player) in &mut characters {
    // let player_speed = player.ship.current_velocity * time.delta_seconds();

    // Increment Camera Zoom
    // if input.pressed(KeyCode::Plus) {
    //     player.ship.current_velocity = (player.ship.current_velocity
    //         + player.ship.acceleration)
    //         .clamp(-player.ship.velocity, player.ship.velocity);

    //     let movement_direction = transform.rotation * Vec3::Y;
    //     let translation_delta = movement_direction * player_speed;
    //     transform.translation += translation_delta;
    // }
    // Backwards
    // }
}
