use bevy::{
    audio::{PlaybackMode, PlaybackSettings, Volume},
    ecs::{entity::Entity, event::EventWriter},
};

use crate::{
    assets::sounds::starships::engines::EngineSound,
    events::{audio_events::SpawnAudioEvent, logging_event::LoggingEvent},
};

pub fn player_engine_rumble_sound(
    mut logging_event_writer: EventWriter<LoggingEvent>,
    mut spawn_audio_event_writer: EventWriter<SpawnAudioEvent>,
) {
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
