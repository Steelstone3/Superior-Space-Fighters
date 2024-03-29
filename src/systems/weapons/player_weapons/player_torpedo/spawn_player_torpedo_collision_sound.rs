use bevy::{
    audio::{PlaybackMode, PlaybackSettings, Volume},
    ecs::{
        component::Component,
        event::{EventReader, EventWriter},
        system::Commands,
    },
};

use crate::{
    assets::sounds::starships::weapons::impacts::ImpactSound,
    events::{collision_events::PlayerTorpedoCollisionEvent, spawn_audio_event::SpawnAudioEvent},
};

#[derive(Component)]
struct PlayerTorpedoCollisionSound;

pub fn spawn_player_torpedo_collision_sound(
    mut commands: Commands,
    mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
    mut player_torpedo_collision_event: EventReader<PlayerTorpedoCollisionEvent>,
) {
    for _ in player_torpedo_collision_event.read() {
        let player_blaster_collision_sound = PlayerTorpedoCollisionSound;

        spawn_audio_event.send(SpawnAudioEvent {
            audio_path: ImpactSound::default().to_string(),
            playback_settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                volume: Volume::new(0.2),
                ..Default::default()
            },
            entity: commands.spawn(player_blaster_collision_sound).id(),
        });
    }
}
