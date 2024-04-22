use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, Query, Res, Vec3},
    time::Time,
    utils::tracing,
};

use crate::queries::player_starship_queries::MutablePlayerStarshipTransformQuery;

pub fn player_movement(
    mut player_query_items: Query<MutablePlayerStarshipTransformQuery>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut player_query_item in &mut player_query_items {
        let mut rotation_factor = 0.0;

        if input.pressed(KeyCode::KeyA) {
            rotation_factor += 1.0;
        } else if input.pressed(KeyCode::KeyD) {
            rotation_factor -= 1.0;
        }

        if input.pressed(KeyCode::KeyW) {
            player_query_item.starship.current_velocity += player_query_item.starship.acceleration;
        } else if input.pressed(KeyCode::KeyS) {
            player_query_item.starship.current_velocity -= player_query_item.starship.acceleration;
        } else if player_query_item.starship.current_velocity > 0.0 {
            player_query_item.starship.current_velocity -= player_query_item.starship.acceleration;
        } else if player_query_item.starship.current_velocity < 0.0 {
            player_query_item.starship.current_velocity += player_query_item.starship.acceleration;
        }

        player_query_item.starship.current_velocity =
            player_query_item.starship.current_velocity.clamp(
                -player_query_item.starship.max_velocity,
                player_query_item.starship.max_velocity,
            );

        player_query_item.transform.rotate_z(
            rotation_factor * player_query_item.starship.rotation_speed * time.delta_seconds(),
        );

        let movement_direction = player_query_item.transform.rotation * Vec3::Y;
        let movement_distance = player_query_item.starship.current_velocity * time.delta_seconds();
        let translation_delta = movement_direction * movement_distance;
        player_query_item.transform.translation += translation_delta;
    }
}
