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
    mut ev_spawn_sprite: EventReader<SpawnSpriteEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_spawn_sprite.read() {
        //If passed entity exists
        if let Some(mut entity) = commands.get_entity(ev.entity) {
            let texture = asset_server.load(&ev.sprite_path);

            entity.insert(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(ev.size),
                    ..Default::default()
                },
                texture,
                transform: bevy::prelude::Transform {
                    translation: ev.translation,
                    rotation: ev.rotation,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
