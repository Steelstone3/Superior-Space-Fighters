use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        system::{Commands, Res},
    },
};

use crate::{
    assets::sounds::starships::engines::EngineSound,
    events::{
        audio_events::SpawnAudioEvent, game_state_events::NewGameEvent, logging_event::LoggingEvent,
    },
};

pub fn player_engine_rumble_sound(
    mut logging_event_writer: EventWriter<LoggingEvent>,
    mut new_game_event_reader: EventReader<NewGameEvent>,
    mut spawn_audio_event_writer: EventWriter<SpawnAudioEvent>,
) {
    for _ in new_game_event_reader.read() {
        logging_event_writer.send(LoggingEvent {
            message: "Engine rumble sound playing".to_string(),
        });

        spawn_audio_event_writer.send(SpawnAudioEvent {
            audio_path: EngineSound::default().to_string(),
            playback_settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(0.75),
                paused: false,
                ..Default::default()
            },
            entity: Entity::PLACEHOLDER,
        });
    }
}
