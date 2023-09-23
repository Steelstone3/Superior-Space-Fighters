use bevy::{
    prelude::{Input, KeyCode, Query, Res, Transform},
    sprite::Sprite,
    time::Time,
};

pub fn player_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let player_speed = 400.0;
    let player_diagonal_speed = calculate_diagonal_speed(player_speed);

    for (mut transform, _) in &mut characters {
        // Forward right diagonal
        if input.pressed(KeyCode::W) && input.pressed(KeyCode::D) {
            transform.translation.y += player_diagonal_speed * time.delta_seconds();
            transform.translation.x += player_diagonal_speed * time.delta_seconds();
        }
        // Forward left diagonal
        else if input.pressed(KeyCode::W) && input.pressed(KeyCode::A) {
            transform.translation.y += player_diagonal_speed * time.delta_seconds();
            transform.translation.x -= player_diagonal_speed * time.delta_seconds();
        }
        // Downward right diagonal
        else if input.pressed(KeyCode::S) && input.pressed(KeyCode::D) {
            transform.translation.y -= player_diagonal_speed * time.delta_seconds();
            transform.translation.x += player_diagonal_speed * time.delta_seconds();
        }
        // Downward left diagonal
        else if input.pressed(KeyCode::S) && input.pressed(KeyCode::A) {
            transform.translation.y -= player_diagonal_speed * time.delta_seconds();
            transform.translation.x -= player_diagonal_speed * time.delta_seconds();
        }
        // Forward
        else if input.pressed(KeyCode::W) {
            transform.translation.y += player_speed * time.delta_seconds();
        }
        // Downward
        else if input.pressed(KeyCode::S) {
            transform.translation.y -= player_speed * time.delta_seconds();
        }
        // Right
        else if input.pressed(KeyCode::D) {
            transform.translation.x += player_speed * time.delta_seconds();
        }
        // Left
        else if input.pressed(KeyCode::A) {
            transform.translation.x -= player_speed * time.delta_seconds();
        }
    }
}

fn calculate_diagonal_speed(speed: f32) -> f32 {
    speed / 4.0 * 3.0
}
