use crate::{
    assets::sounds::starships::weapons::exotics::ExoticSound,
    events::{combat_events::FirePlayerExoticEvent, spawn_audio_event::SpawnAudioEvent},
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
struct PlayerExoticSound;

pub fn spawn_player_exotic_sound(
    mut commands: Commands,
    mut fire_player_exotic_event: EventReader<FirePlayerExoticEvent>,
    mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
) {
    for _ in fire_player_exotic_event.read() {
        let sound_path = ExoticSound::default().to_string();
        let entity = commands.spawn(PlayerExoticSound).id();

        let event = SpawnAudioEvent {
            entity,
            audio_path: sound_path,
            playback_settings: PlaybackSettings::default(),
        };

        spawn_audio_event.send(event);
    }
}
