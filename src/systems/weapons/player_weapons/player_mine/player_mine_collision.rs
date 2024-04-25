use crate::{
    events::{
        collision_events::PlayerMineCollisionEvent, despawn_sprite_event::DespawnSpriteEvent,
        logging_event::LoggingEvent,
    },
    queries::{
        player_mine_queries::{PlayerMineEntityTransformQuery, PlayerMineFilter},
        starship_queries::{AIStarshipFilter, MutableAIStarshipTransformQuery},
    },
    systems::controllers::random_generator::generate_seed,
};
use bevy::{ecs::event::EventWriter, prelude::Query};

pub fn player_mine_collision_with_starship(
    player_mines: Query<PlayerMineEntityTransformQuery, PlayerMineFilter>,
    mut starships: Query<MutableAIStarshipTransformQuery, AIStarshipFilter>,
    mut player_mine_collision_event: EventWriter<PlayerMineCollisionEvent>,
    mut logging_event: EventWriter<LoggingEvent>,
    mut despawn_sprite_event: EventWriter<DespawnSpriteEvent>,
) {
    for player_mine in &player_mines {
        for mut starship in &mut starships {
            let distance_to_starship =
                (player_mine.transform.translation - starship.transform.translation).length();

            let is_collision = distance_to_starship < starship.starship.size.x
                || distance_to_starship < starship.starship.size.y;

            if is_collision {
                player_mine_collision_event.send(PlayerMineCollisionEvent {});

                logging_event.send(LoggingEvent {
                    message: "Mine collision with starship".to_string(),
                });

                let damage = player_mine
                    .player_mine
                    .mine
                    .lifetime_weapon
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
                    entity: player_mine.entity,
                });
            }
        }
    }
}
