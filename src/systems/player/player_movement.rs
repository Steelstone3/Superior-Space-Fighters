use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, Query, Res, Vec3},
    time::Time,
};

use crate::queries::player_starship_queries::MutablePlayerStarshipTransformQuery;

pub fn player_movement(
    mut players: Query<MutablePlayerStarshipTransformQuery>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut player in &mut players {
        let player_speed = player.player_starship.current_velocity * time.delta_seconds();

        // Forwards
        if input.pressed(KeyCode::KeyW) {
            player.player_starship.current_velocity = (player.player_starship.current_velocity
                + player.player_starship.acceleration)
                .clamp(
                    -player.player_starship.ship.velocity,
                    player.player_starship.ship.velocity,
                );

            let movement_direction = player.transform.rotation * Vec3::Y;
            let translation_delta = movement_direction * player_speed;
            player.transform.translation += translation_delta;
        }
        // Backwards
        else if input.pressed(KeyCode::KeyS) {
            player.player_starship.current_velocity = (player.player_starship.current_velocity
                - player.player_starship.acceleration)
                .clamp(
                    -player.player_starship.ship.velocity,
                    player.player_starship.ship.velocity,
                );

            let movement_direction = player.transform.rotation * Vec3::Y;
            let translation_delta = movement_direction * player_speed / 2.0;
            player.transform.translation += translation_delta;
        }
        // Slow down
        else {
            let movement_direction = player.transform.rotation * Vec3::Y;

            if player.player_starship.current_velocity > 0.0 {
                player.player_starship.current_velocity = (player.player_starship.current_velocity
                    - player.player_starship.acceleration)
                    .clamp(
                        -player.player_starship.ship.velocity,
                        player.player_starship.ship.velocity,
                    );
                let translation_delta = movement_direction * -player_speed;
                player.transform.translation -= translation_delta;
            } else if player.player_starship.current_velocity < 0.0 {
                player.player_starship.current_velocity = (player.player_starship.current_velocity
                    + player.player_starship.acceleration)
                    .clamp(
                        -player.player_starship.ship.velocity / 2.0,
                        player.player_starship.ship.velocity,
                    );
                let translation_delta = movement_direction * player_speed;
                player.transform.translation += translation_delta;
            }
        }

        // Rotate Right
        if input.pressed(KeyCode::KeyD) {
            let reverse_player_rotation = player.player_starship.rotation * -1.0;
            player
                .transform
                .rotate_z(reverse_player_rotation * time.delta_seconds());
        }

        // Rotate Left
        if input.pressed(KeyCode::KeyA) {
            player
                .transform
                .rotate_z(player.player_starship.rotation * time.delta_seconds());
        }
    }
}
