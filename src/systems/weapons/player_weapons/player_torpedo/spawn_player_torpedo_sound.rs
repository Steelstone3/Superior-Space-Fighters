use crate::{
    assets::sounds::starships::weapons::torpedos::TorpedoSound,
    events::audio_events::SpawnAudioEvent,
};
use bevy::{
    audio::PlaybackSettings,
    ecs::{component::Component, event::EventWriter},
    prelude::Commands,
};

#[derive(Component)]
struct PlayerTorpedoSound;

pub fn spawn_player_torpedo_sound(
    mut commands: Commands,
    mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
) {
    let sound_path = TorpedoSound::default().to_string();
    let entity = commands.spawn(PlayerTorpedoSound).id();

    let event = SpawnAudioEvent {
        entity,
        audio_path: sound_path,
        playback_settings: PlaybackSettings::default(),
    };

    spawn_audio_event.send(event);
}
