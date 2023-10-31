use bevy::{
    prelude::{Input, KeyCode, Query, Res, Transform, Vec3},
    time::Time,
};

use crate::components::player::Player;

pub fn player_movement(
    mut characters: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut characters {
        let player_speed = player.current_velocity * time.delta_seconds();

        // Forward
        if input.pressed(KeyCode::W) {
            if player.current_velocity >= 0.0 {
                player.current_velocity += player.acceleration.clamp(0.0, player.velocity);

                let movement_direction = transform.rotation * Vec3::Y;
                let translation_delta = movement_direction * player_speed;
                transform.translation += translation_delta;
            }
        }
        
        // Downward
        if input.pressed(KeyCode::S) {
            if player.current_velocity <= 0.01 {
                player.current_velocity -= player.acceleration.clamp(-player.velocity / 2.0, 0.01);

                let movement_direction = transform.rotation * Vec3::Y;
                let translation_delta = movement_direction * -(player_speed / 2.0);
                transform.translation -= translation_delta;
            }
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
