use crate::{
    assets::sounds::starships::weapons::exotics::ExoticSound, events::audio_events::SpawnAudioEvent,
};
use bevy::{
    audio::PlaybackSettings,
    ecs::{component::Component, event::EventWriter},
    prelude::Commands,
};

#[derive(Component)]
struct PlayerExoticSound;

pub fn spawn_player_exotic_sound(
    mut commands: Commands,
    mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
) {
    let sound_path = ExoticSound::default().to_string();
    let entity = commands.spawn(PlayerExoticSound).id();

    let event = SpawnAudioEvent {
        entity,
        audio_path: sound_path,
        playback_settings: PlaybackSettings::default(),
    };

    spawn_audio_event.send(event);
}
