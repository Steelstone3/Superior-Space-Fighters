use crate::{
    events::{
        collision_events::PlayerMineCollisionEvent, despawn_sprite_event::DespawnSpriteEvent,
        logging_event::LoggingEvent,
    },
    queries::{
        player_mine_queries::{MutablePlayerMineEntityTransformQuery, PlayerMineFilter},
        starship_queries::{MutableStarshipTransformQuery, StarshipFilter},
    },
};
use bevy::{ecs::event::EventWriter, prelude::Query};

pub fn player_mine_collision_with_starship(
    mut player_mines: Query<MutablePlayerMineEntityTransformQuery, PlayerMineFilter>,
    mut starships: Query<MutableStarshipTransformQuery, StarshipFilter>,
    mut player_mine_collision_event: EventWriter<PlayerMineCollisionEvent>,
    mut logging_event: EventWriter<LoggingEvent>,
    mut despawn_sprite_event: EventWriter<DespawnSpriteEvent>,
) {
    for mut player_mine in &mut player_mines {
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

                player_mine
                    .player_mine
                    .mine
                    .lifetime_weapon
                    .weapon
                    .damage
                    .calculate_damage();
                starship
                    .starship
                    .take_damage(player_mine.player_mine.mine.lifetime_weapon.weapon.damage);

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
