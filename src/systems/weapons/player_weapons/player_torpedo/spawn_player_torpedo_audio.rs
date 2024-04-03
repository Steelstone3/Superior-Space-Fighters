use crate::{
    assets::sounds::starships::weapons::torpedos::TorpedoSound,
    events::{combat_events::FirePlayerTorpedoEvent, spawn_audio_event::SpawnAudioEvent},
};
use bevy::{
    audio::PlaybackSettings,
    ecs::{
        component::Component,
        event::{EventReader, EventWriter},
    },
    prelude::Commands,
};

#[derive(Component)]
struct PlayerTorpedoSound;

pub fn spawn_player_torpedo_sound(
    mut commands: Commands,
    mut fire_player_torpedo_event: EventReader<FirePlayerTorpedoEvent>,
    mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
) {
    for _ in fire_player_torpedo_event.read() {
        let sound_path = TorpedoSound::default().to_string();
        let entity = commands.spawn(PlayerTorpedoSound).id();

        let event = SpawnAudioEvent {
            entity,
            audio_path: sound_path,
            playback_settings: PlaybackSettings::default(),
        };

        spawn_audio_event.send(event);
    }
}
