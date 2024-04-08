use crate::{
    assets::sounds::starships::weapons::mines::MineSound,
    events::{audio_events::SpawnAudioEvent, combat_events::FirePlayerMineEvent},
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
struct PlayerMineSound;

pub fn spawn_player_mine_sound(
    mut commands: Commands,
    mut fire_player_mine_event: EventReader<FirePlayerMineEvent>,
    mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
) {
    for _ in fire_player_mine_event.read() {
        let sound_path = MineSound::default().to_string();
        let entity = commands.spawn(PlayerMineSound).id();

        let event = SpawnAudioEvent {
            entity,
            audio_path: sound_path,
            playback_settings: PlaybackSettings::default(),
        };

        spawn_audio_event.send(event);
    }
}
