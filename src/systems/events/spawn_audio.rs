use bevy::{
    asset::AssetServer,
    audio::AudioBundle,
    ecs::{
        event::EventReader,
        system::{Commands, Res},
    },
};

use crate::events::spawn_audio_event::SpawnAudioEvent;

#[allow(dead_code)]
pub fn spawn_audio(
    mut spawn_audio_events: EventReader<SpawnAudioEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for spawn_audio_event in spawn_audio_events.read() {
        //If passed entity exists
        if let Some(mut entity) = commands.get_entity(spawn_audio_event.entity) {
            let source = asset_server.load(&spawn_audio_event.audio_path);
            let settings = spawn_audio_event.playback_settings;

            entity.insert(AudioBundle { source, settings });
        }
    }
}
