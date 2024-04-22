use crate::{
    events::{despawn_sprite_event::DespawnSpriteEvent, logging_event::LoggingEvent},
    queries::starship_queries::AIStarshipEntityQuery,
};
use bevy::ecs::{event::EventWriter, system::Query};

pub fn despawn_destoryed_starships(
    mut despawn_sprite_event: EventWriter<DespawnSpriteEvent>,
    mut logging_event: EventWriter<LoggingEvent>,
    starships: Query<AIStarshipEntityQuery>,
) {
    for starship in &starships {
        if starship.starship.is_destroyed() {
            despawn_sprite_event.send(DespawnSpriteEvent {
                entity: starship.entity,
            });

            logging_event.send(LoggingEvent {
                message: "Enemy Starship Destroyed".to_string(),
            });
        }
    }
}
