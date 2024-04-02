use crate::events::despawn_sprite_event::DespawnSpriteEvent;
use bevy::ecs::{event::EventReader, system::Commands};

pub fn despawn_sprite(
    mut despawn_sprite_events: EventReader<DespawnSpriteEvent>,
    mut commands: Commands,
) {
    for despawn_sprite_event in despawn_sprite_events.read() {
        if let Some(_entity) = commands.get_entity(despawn_sprite_event.entity) {
            commands.entity(despawn_sprite_event.entity).despawn()
        }
    }
}
