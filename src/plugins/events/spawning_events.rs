use crate::events::{
    despawn_sprite_event::DespawnSpriteEvent, spawn_audio_event::SpawnAudioEvent,
    spawn_sprite_event::SpawnSpriteEvent,
};
use bevy::app::Plugin;

pub struct SpawningEvents;

impl Plugin for SpawningEvents {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SpawnSpriteEvent>()
            .add_event::<DespawnSpriteEvent>()
            .add_event::<SpawnAudioEvent>();
    }
}
