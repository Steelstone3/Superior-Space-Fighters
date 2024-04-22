use crate::{
    assets::sounds::starships::weapons::blasters::BlasterSound,
    events::audio_events::SpawnAudioEvent,
};
use bevy::{
    audio::PlaybackSettings,
    ecs::{component::Component, event::EventWriter},
    prelude::Commands,
};

#[derive(Component)]
struct PlayerBlasterSound;

pub fn spawn_player_blaster_sound(
    mut commands: Commands,
    mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
) {
    let sound_path = BlasterSound::default().to_string();
    let entity = commands.spawn(PlayerBlasterSound).id();

    let event = SpawnAudioEvent {
        entity,
        audio_path: sound_path,
        playback_settings: PlaybackSettings::default(),
    };

    spawn_audio_event.send(event);
}
