use crate::{
    assets::sounds::starships::weapons::impacts::ImpactSound,
    events::{audio_events::SpawnAudioEvent, collision_events::PlayerExoticCollisionEvent},
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
struct PlayerExoticCollisionSound;

pub fn spawn_player_exoitc_collision_sound(
    mut commands: Commands,
    mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
    mut player_exoitc_collision_event: EventReader<PlayerExoticCollisionEvent>,
) {
    for _ in player_exoitc_collision_event.read() {
        let player_exotic_collision_sound = PlayerExoticCollisionSound;

        spawn_audio_event.send(SpawnAudioEvent {
            audio_path: ImpactSound::default().to_string(),
            playback_settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                volume: Volume::new(0.2),
                ..Default::default()
            },
            entity: commands.spawn(player_exotic_collision_sound).id(),
        });
    }
}
