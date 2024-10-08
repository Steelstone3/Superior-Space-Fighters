use bevy::{
    asset::AssetServer,
    ecs::{
        event::EventReader,
        system::{Commands, Res},
    },
    sprite::{Sprite, SpriteBundle},
};

use crate::events::{
    despawn_sprite_event::DespawnSpriteEvent, spawn_sprite_event::SpawnSpriteEvent,
};

pub fn spawn_sprite(
    mut spawn_sprite_events: EventReader<SpawnSpriteEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for spawn_sprite_event in spawn_sprite_events.read() {
        //If passed entity exists
        if let Some(mut entity) = commands.get_entity(spawn_sprite_event.entity) {
            let texture = asset_server.load(&spawn_sprite_event.sprite_path);

            entity.insert(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(spawn_sprite_event.size),
                    ..Default::default()
                },
                texture,
                transform: spawn_sprite_event.transform,
                ..Default::default()
            });
        }
    }
}

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
