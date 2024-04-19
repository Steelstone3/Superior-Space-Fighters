use crate::{
    events::{
        collision_events::PlayerBlasterCollisionEvent, despawn_sprite_event::DespawnSpriteEvent,
        logging_event::LoggingEvent,
    },
    queries::{
        player_blaster_queries::{PlayerBlasterEntityTransformQuery, PlayerBlasterFilter},
        starship_queries::{MutableAIStarshipTransformQuery, StarshipFilter},
    },
    systems::controllers::random_generator::generate_seed,
};
use bevy::{ecs::event::EventWriter, prelude::Query};

pub fn player_blaster_collision_with_starship(
    player_blasters: Query<PlayerBlasterEntityTransformQuery, PlayerBlasterFilter>,
    mut starships: Query<MutableAIStarshipTransformQuery, StarshipFilter>,
    mut player_blaster_collision_event: EventWriter<PlayerBlasterCollisionEvent>,
    mut logging_event: EventWriter<LoggingEvent>,
    mut despawn_sprite_event: EventWriter<DespawnSpriteEvent>,
) {
    for player_blaster in &player_blasters {
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

                let damage = player_blaster
                    .player_blaster
                    .blaster
                    .ranged_weapon
                    .weapon
                    .damage
                    .calculate_damage(generate_seed());
                starship.starship.take_damage(damage);

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
