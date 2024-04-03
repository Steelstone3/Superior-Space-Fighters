use bevy::{ecs::event::EventWriter, prelude::Query};

use crate::{
    events::{
        collision_events::PlayerTorpedoCollisionEvent, despawn_sprite_event::DespawnSpriteEvent,
        logging_event::LoggingEvent,
    },
    queries::{
        player_torpedo_queries::{MutablePlayerTorpedoEntityTransformQuery, PlayerTorpedoFilter},
        starship_queries::{MutableStarshipTransformQuery, StarshipFilter},
    },
    systems::controllers::random_generator::generate_seed,
};

pub fn player_torpedo_collision_with_starship(
    mut player_torpedoes: Query<MutablePlayerTorpedoEntityTransformQuery, PlayerTorpedoFilter>,
    mut starships: Query<MutableStarshipTransformQuery, StarshipFilter>,
    mut player_torpedo_collision_event: EventWriter<PlayerTorpedoCollisionEvent>,
    mut logging_event: EventWriter<LoggingEvent>,
    mut despawn_sprite_event: EventWriter<DespawnSpriteEvent>,
) {
    for player_torpedo in &mut player_torpedoes {
        for mut starship in &mut starships {
            let distance_to_starship =
                (player_torpedo.transform.translation - starship.transform.translation).length();

            let is_collision = distance_to_starship < starship.starship.size.x
                || distance_to_starship < starship.starship.size.y;

            if is_collision {
                player_torpedo_collision_event.send(PlayerTorpedoCollisionEvent {});

                logging_event.send(LoggingEvent {
                    message: "Torpedo collision with starship".to_string(),
                });

                let damage = player_torpedo
                    .player_torpedo
                    .torpedo
                    .lock_on_weapon
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
                    entity: player_torpedo.entity,
                });
            }
        }
    }
}
