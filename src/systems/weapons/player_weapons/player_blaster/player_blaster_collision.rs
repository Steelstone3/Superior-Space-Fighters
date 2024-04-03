use crate::{
    events::{
        collision_events::PlayerBlasterCollisionEvent, despawn_sprite_event::DespawnSpriteEvent,
        logging_event::LoggingEvent,
    },
    queries::{
        player_blaster_queries::{MutablePlayerBlasterEntityTransformQuery, PlayerBlasterFilter},
        starship_queries::{MutableStarshipTransformQuery, StarshipFilter},
    },
};
use bevy::{ecs::event::EventWriter, prelude::Query};

pub fn player_blaster_collision_with_starship(
    mut player_blasters: Query<MutablePlayerBlasterEntityTransformQuery, PlayerBlasterFilter>,
    mut starships: Query<MutableStarshipTransformQuery, StarshipFilter>,
    mut player_blaster_collision_event: EventWriter<PlayerBlasterCollisionEvent>,
    mut logging_event: EventWriter<LoggingEvent>,
    mut despawn_sprite_event: EventWriter<DespawnSpriteEvent>,
) {
    for mut player_blaster in &mut player_blasters {
        for mut starship in &mut starships {
            let distance_to_starship =
                (player_blaster.transform.translation - starship.transform.translation).length();

            let is_collision = distance_to_starship < starship.starship.size.x
                || distance_to_starship < starship.starship.size.y;

            if is_collision {
                player_blaster_collision_event.send(PlayerBlasterCollisionEvent {});

                logging_event.send(LoggingEvent {
                    message: "Blaster collision with starship".to_string(),
                });

                player_blaster
                    .player_blaster
                    .blaster
                    .ranged_weapon
                    .weapon
                    .damage
                    .calculate_damage();
                starship.starship.take_damage(
                    player_blaster
                        .player_blaster
                        .blaster
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
                    entity: player_blaster.entity,
                });
            }
        }
    }
}
