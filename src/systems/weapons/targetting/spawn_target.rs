use crate::components::{
    starships::{player_starship::PlayerStarship, starship::Starship},
    weapons::weapon_types::target::Target,
};
use bevy::{
    input::ButtonInput,
    prelude::{AssetServer, Commands, KeyCode, Query, Res, Transform},
    sprite::{Sprite, SpriteBundle},
    utils::tracing,
};

pub fn spawn_target(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    player_starships: Query<(&Transform, &PlayerStarship)>,
    starships: Query<(&Transform, &Starship)>,
) {
    if !input.just_pressed(KeyCode::KeyT) {
        return;
    }

    let Ok(player_starship) = player_starships.get_single() else {
        return;
    };

    tracing::info!("Spawning Target");

    let maximum_distance = 2000.0;
    let mut closest_ship: Option<(&Transform, &Starship)> = None;
    let mut distance = 2000.0;

    for starship in starships.into_iter() {
        let new_distance = (starship.0.translation - player_starship.0.translation).length();

        if new_distance < distance {
            distance = new_distance;

            if distance <= maximum_distance {
                closest_ship = Some(starship);
                tracing::info!(
                    "Closest Ship {:?}",
                    closest_ship.unwrap().1.faction_starship.to_string()
                );
            }
        } else {
            continue;
        }
    }

    let target = Target::default();

    let texture = asset_server.load(target.lock_on_target.to_string());

    match closest_ship {
        Some(closest_ship) => {
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(closest_ship.1.size),
                        ..Default::default()
                    },
                    transform: *closest_ship.0,
                    texture,
                    ..Default::default()
                })
                .insert(target);
        }
        None => {}
    }
}
