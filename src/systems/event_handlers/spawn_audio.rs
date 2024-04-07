use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, AudioSink, AudioSinkPlayback},
    ecs::{
        event::EventReader,
        system::{Commands, Query, Res},
    },
};

use crate::events::{
    pause_audio_event::PauseAudioEvent, play_audio_event::PlayAudioEvent,
    spawn_audio_event::SpawnAudioEvent,
};

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

pub fn pause_audio(
    audio_controllers: Query<&AudioSink>,
    mut event_pause_audio: EventReader<PauseAudioEvent>,
) {
    for _ in event_pause_audio.read() {
        for audio_controller in audio_controllers.iter() {
            audio_controller.pause();
        }
    }
}

pub fn play_audio(
    audio_controllers: Query<&AudioSink>,
    mut event_play_audio: EventReader<PlayAudioEvent>,
) {
    for _ in event_play_audio.read() {
        for audio_controller in audio_controllers.iter() {
            audio_controller.play();
        }
    }
}
