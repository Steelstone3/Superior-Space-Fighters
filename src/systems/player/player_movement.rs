use bevy::{
    prelude::{Input, KeyCode, Query, Res, Transform, Vec3},
    time::Time,
};

use crate::components::player::Player;

pub fn player_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let player_speed = player.velocity * time.delta_seconds();

        // Forward
        if input.pressed(KeyCode::W) {
            let movement_direction = transform.rotation * Vec3::Y;
            let translation_delta = movement_direction * player_speed;
            transform.translation += translation_delta;
        }
        // Downward
        if input.pressed(KeyCode::S) {
            let movement_direction = transform.rotation * Vec3::Y;
            let translation_delta = movement_direction * (player_speed / 2.0);
            transform.translation -= translation_delta;
        }
        // Rotate Right
        if input.pressed(KeyCode::D) {
            let reverse_player_rotation = player.rotation * -1.0;
            transform.rotate_z(reverse_player_rotation * time.delta_seconds());
        }
        // Rotate Left
        if input.pressed(KeyCode::A) {
            transform.rotate_z(player.rotation * time.delta_seconds());
        }
    }
}
