use crate::components::player_starship::PlayerStarship;
use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, Query, Res, Transform, Vec3},
    time::Time,
};

pub fn player_movement(
    mut players: Query<(&mut Transform, &mut PlayerStarship)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut players {
        let player_speed = player.current_velocity * time.delta_seconds();

        // Forwards
        if input.pressed(KeyCode::KeyW) {
            player.current_velocity = (player.current_velocity + player.acceleration)
                .clamp(-player.ship.velocity, player.ship.velocity);

            let movement_direction = transform.rotation * Vec3::Y;
            let translation_delta = movement_direction * player_speed;
            transform.translation += translation_delta;
        }
        // Backwards
        else if input.pressed(KeyCode::KeyS) {
            player.current_velocity = (player.current_velocity - player.acceleration)
                .clamp(-player.ship.velocity, player.ship.velocity);

            let movement_direction = transform.rotation * Vec3::Y;
            let translation_delta = movement_direction * player_speed / 2.0;
            transform.translation += translation_delta;
        }
        // Slow down
        else {
            let movement_direction = transform.rotation * Vec3::Y;

            if player.current_velocity > 0.0 {
                player.current_velocity = (player.current_velocity - player.acceleration)
                    .clamp(-player.ship.velocity, player.ship.velocity);
                let translation_delta = movement_direction * -player_speed;
                transform.translation -= translation_delta;
            } else if player.current_velocity < 0.0 {
                player.current_velocity = (player.current_velocity + player.acceleration)
                    .clamp(-player.ship.velocity / 2.0, player.ship.velocity);
                let translation_delta = movement_direction * player_speed;
                transform.translation += translation_delta;
            }
        }

        // Rotate Right
        if input.pressed(KeyCode::KeyD) {
            let reverse_player_rotation = player.rotation * -1.0;
            transform.rotate_z(reverse_player_rotation * time.delta_seconds());
        }

        // Rotate Left
        if input.pressed(KeyCode::KeyA) {
            transform.rotate_z(player.rotation * time.delta_seconds());
        }
    }
}
