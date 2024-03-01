use bevy::{
    ecs::{
        entity::Entity,
        query::{With, Without},
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

pub fn spawn_player_targeting<'a>(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    other_ships: Query<(&Transform, &Starship, Entity), Without<PlayerStarship>>,
    mut existing_target: Query<&mut Target>,
    player_location: Query<&Transform, (With<PlayerStarship>, Without<Starship>)>,
) {
    if input.just_pressed(KeyCode::KeyT) {
        let Ok(player_location) = player_location.get_single() else {
            return;
        };

        let mut closest_ship: Option<(&Transform, &Starship, Entity)> = None;
        let mut distance = 999.0;
        let mut ship_count = 0;
        //get closest target ship
        for other_ship in other_ships.into_iter() {
            ship_count += 1;
            let new_distance = (other_ship.0.translation - player_location.translation).length();
            if new_distance < distance {
                distance = new_distance;
                closest_ship = Some(other_ship);
            } else {
                continue;
            }
        }

        log::info!("{ship_count} ships to target");
        log::info!("Closest ship is {distance} units away");

        match closest_ship {
            Some(closest_target) => {
                //if target already exists update target else create new target
                let Ok(mut existing_target) = existing_target.get_single_mut() else {
                    let target = Target {
                        target_entity: closest_target.2,
                    };

                    let sprite = Sprite {
                        color: Color::rgba(1.0, 1.0, 1.0, 0.1),
                        flip_x: false,
                        flip_y: false,
                        custom_size: Some(Vec2::new(100.0, 100.0)),
                        ..Default::default()
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
                            transform: *closest_target.0,
                            ..Default::default()
                        })
                        .insert(target);

                    return;
                };

                existing_target.target_entity = closest_target.2;

                log::info!("Target Locked");
            }
            None => {}
        }
    }
}
