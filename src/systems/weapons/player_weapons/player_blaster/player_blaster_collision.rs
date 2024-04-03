use crate::{
    events::{
        collision_events::PlayerBlasterCollisionEvent, despawn_sprite_event::DespawnSpriteEvent,
    },
    queries::{
        player_blaster_queries::{MutablePlayerBlasterEntityTransformQuery, PlayerBlasterFilter},
        starship_queries::{MutableStarshipEntityTransformQuery, StarshipFilter},
    },
};
use bevy::{ecs::event::EventWriter, prelude::Query};

// TODO multi-thread
pub fn player_blaster_collision_with_starship(
    mut player_blasters: Query<MutablePlayerBlasterEntityTransformQuery, PlayerBlasterFilter>,
    mut starships: Query<MutableStarshipEntityTransformQuery, StarshipFilter>,
    mut player_blaster_collision_event: EventWriter<PlayerBlasterCollisionEvent>,
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

                despawn_sprite_event.send(DespawnSpriteEvent {
                    entity: player_blaster.entity,
                });
            }
        }
    }
}
