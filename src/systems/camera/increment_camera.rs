use bevy::{
    prelude::{Input, KeyCode, Query, Res, Transform, Vec3},
    time::Time,
};

use crate::components::player_starship::PlayerStarship;

pub fn increment_camera(
    mut characters: Query<(&mut Transform, &mut PlayerStarship)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut characters {
        let player_speed = player.ship.current_velocity * time.delta_seconds();

        // Forwards
        if input.pressed(KeyCode::W) {
            player.ship.current_velocity = (player.ship.current_velocity
                + player.ship.acceleration)
                .clamp(-player.ship.velocity, player.ship.velocity);

            let movement_direction = transform.rotation * Vec3::Y;
            let translation_delta = movement_direction * player_speed;
            transform.translation += translation_delta;
        }
        // Backwards
    }
}
