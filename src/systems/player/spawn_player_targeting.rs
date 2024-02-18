use bevy::{
    ecs::{
        query::Without,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
    log,
    math::Vec2,
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

use crate::components::{player_starship::PlayerStarship, starship::Starship};

pub fn spawn_player_targeting(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut other_ships: Query<(&Transform, &Starship), Without<PlayerStarship>>,
) {
    if input.just_pressed(KeyCode::KeyT) {
        for other_ship in &mut other_ships {
            log::info!("Target Locked");

            let sprite = Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.1),
                flip_x: false,
                flip_y: false,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            };

            commands.spawn(SpriteBundle {
                sprite,
                transform: *other_ship.0,
                ..Default::default()
            });

            return;
        }
    }
}
