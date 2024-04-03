use crate::{
    events::{
        collision_events::PlayerExoticCollisionEvent, despawn_sprite_event::DespawnSpriteEvent,
        logging_event::LoggingEvent,
    },
    queries::{
        player_exotic_queries::{MutablePlayerExoticEntityTransformQuery, PlayerExoticFilter},
        starship_queries::{MutableStarshipEntityTransformQuery, StarshipFilter},
    },
};
use bevy::{ecs::event::EventWriter, prelude::Query};

pub fn player_exotic_collision_with_starship(
    mut player_exotics: Query<MutablePlayerExoticEntityTransformQuery, PlayerExoticFilter>,
    mut starships: Query<MutableStarshipEntityTransformQuery, StarshipFilter>,
    mut player_exotic_collision_event: EventWriter<PlayerExoticCollisionEvent>,
    mut logging_event: EventWriter<LoggingEvent>,
    mut despawn_sprite_event: EventWriter<DespawnSpriteEvent>,
) {
    for mut player_exotic in &mut player_exotics {
        for mut starship in &mut starships {
            let distance_to_starship =
                (player_exotic.transform.translation - starship.transform.translation).length();

            let is_collision = distance_to_starship < starship.starship.size.x
                || distance_to_starship < starship.starship.size.y;

            if is_collision {
                player_exotic_collision_event.send(PlayerExoticCollisionEvent {});

                logging_event.send(LoggingEvent {
                    message: "Exotic collision with starship".to_string(),
                });

                player_exotic
                    .player_exotic
                    .exotic
                    .ranged_weapon
                    .weapon
                    .damage
                    .calculate_damage();
                starship.starship.take_damage(
                    player_exotic
                        .player_exotic
                        .exotic
                        .ranged_weapon
                        .weapon
                        .damage,
                );

                logging_event.send(LoggingEvent {
                    message: format!(
                        "Enemy Starship | Shield: {:?} | Health: {:?} |",
                        starship.starship.shield.current, starship.starship.hull.current
                    ),
                });

                despawn_sprite_event.send(DespawnSpriteEvent {
                    entity: player_exotic.entity,
                });
            }
        }
    }
}
