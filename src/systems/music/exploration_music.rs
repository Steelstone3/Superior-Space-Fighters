use crate::{events::audio_events::SpawnAudioEvent, resources::music::Music};
use bevy::{
    audio::{PlaybackMode, PlaybackSettings, Volume},
    ecs::event::EventWriter,
    prelude::{Commands, Res},
};

pub fn play_exploration_music(
    mut commands: Commands,
    music: Res<Music>,
    mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
) {
    let entity = commands.spawn_empty().id();
    let event = SpawnAudioEvent {
        audio_path: music.exploration_music.to_string(),
        playback_settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.75),
            ..Default::default()
        },
        entity,
    };

    spawn_audio_event.send(event);
}
