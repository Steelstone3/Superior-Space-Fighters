use bevy::{
    asset::AssetServer,
    ecs::{
        event::EventReader,
        system::{Commands, Res},
    },
    sprite::{Sprite, SpriteBundle},
};

use crate::events::spawn_sprite_event::SpawnSpriteEvent;

pub fn spawn_sprites(
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
                transform: bevy::prelude::Transform {
                    translation: spawn_sprite_event.translation,
                    rotation: spawn_sprite_event.rotation,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
