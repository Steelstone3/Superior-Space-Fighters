use crate::{
    assets::sounds::starships::weapons::impacts::ImpactSound,
    events::{collision_events::PlayerMineCollisionEvent, spawn_audio_event::SpawnAudioEvent},
};
use bevy::{
    audio::{PlaybackMode, PlaybackSettings, Volume},
    ecs::{
        component::Component,
        event::{EventReader, EventWriter},
        system::Commands,
    },
};

#[derive(Component)]
struct PlayerMineCollisionSound;

pub fn spawn_player_mine_collision_sound(
    mut commands: Commands,
    mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
    mut player_mine_collision_event: EventReader<PlayerMineCollisionEvent>,
) {
    for _ in player_mine_collision_event.read() {
        let player_mine_collision_sound = PlayerMineCollisionSound;

        spawn_audio_event.send(SpawnAudioEvent {
            audio_path: ImpactSound::default().to_string(),
            playback_settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                volume: Volume::new(0.2),
                ..Default::default()
            },
            entity: commands.spawn(player_mine_collision_sound).id(),
        });
    }
}
