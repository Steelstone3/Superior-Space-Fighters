use bevy::{
    ecs::{
        entity::Entity,
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

use crate::components::{
    player_starship::PlayerStarship, starship::Starship, weapons::target::Target,
};

pub fn spawn_player_targeting(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut other_ships: Query<(&Transform, &Starship, Entity), Without<PlayerStarship>>,
    mut existing_target: Query<&mut Target>,
) {
    if input.just_pressed(KeyCode::KeyT) {
        // for other_ship in &mut other_ships {
        if let Some(other_ship) = (&mut other_ships).into_iter().next() {
            let Ok(mut target) = existing_target.get_single_mut() else {
                let target = Target {
                    target_entity: other_ship.2,
                };

                let sprite = Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.1),
                    flip_x: false,
                    flip_y: false,
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..Default::default()
                };

                commands
                    .spawn(SpriteBundle {
                        sprite,
                        transform: *other_ship.0,
                        ..Default::default()
                    })
                    .insert(target);

                return;
            };

            target.target_entity = other_ship.2;

            log::info!("Target Locked");
        }
    }
}
