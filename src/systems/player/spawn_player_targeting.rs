use bevy::{
    ecs::{
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

use crate::{
    components::weapons::target::Target,
    query_data::starship_query::{StarshipQuery, StarshipQueryItem},
    query_filters::player_starship_filter::PlayerStarshipFilter,
};

pub fn spawn_player_targeting(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    other_ships: Query<StarshipQuery>,
    mut existing_target: Query<&mut Target>,
    player_location: Query<&Transform, PlayerStarshipFilter>,
) {
    if input.just_pressed(KeyCode::KeyT) {
        let Ok(player_location) = player_location.get_single() else {
            return;
        };

        let mut closest_ship: Option<StarshipQueryItem> = None;
        let mut distance = 999.0; //init value never used
        let mut ship_count = 0;
        //get closest target ship
        for other_ship in other_ships.into_iter() {
            ship_count += 1;
            let new_distance =
                (other_ship.transform.translation - player_location.translation).length();
            if new_distance < distance {
                distance = new_distance;
                if distance <= 500.0 {
                    closest_ship = Some(other_ship);
                }
            } else {
                continue;
            }
        }

        if let Some(closest_ship) = closest_ship {
            log::info!("{ship_count} ships to target");
            log::info!("Closest ship is {distance} units away");

            //if target already exists update target else create new target
            let Ok(mut existing_target) = existing_target.get_single_mut() else {
                let target = Target {
                    target_entity: closest_ship.entity,
                };

                log::info!("Targeting");

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
                        transform: *closest_ship.transform,
                        ..Default::default()
                    })
                    .insert(target);

                log::info!("Target Locked");
                return;
            };

            existing_target.target_entity = closest_ship.entity;

            log::info!("Target Locked");
        } else {
            log::info!("No valid targets")
        }
    }
}
