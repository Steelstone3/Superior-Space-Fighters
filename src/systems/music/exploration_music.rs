use crate::{
    events::{audio_events::SpawnAudioEvent, game_state_events::NewGameEvent},
    resources::music::MusicResource,
};
use bevy::{
    audio::{PlaybackMode, PlaybackSettings, Volume},
    ecs::event::{EventReader, EventWriter},
    prelude::{Commands, Res},
};

pub fn play_exploration_music(
    mut commands: Commands,
    music: Res<MusicResource>,
    mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
    mut new_game_event_reader: EventReader<NewGameEvent>,
) {
    for _ in new_game_event_reader.read() {
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
}
