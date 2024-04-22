use bevy::{
    audio::{PlaybackMode, PlaybackSettings, Volume},
    ecs::{component::Component, event::EventWriter, system::Commands},
};

use crate::{
    assets::sounds::starships::weapons::impacts::ImpactSound, events::audio_events::SpawnAudioEvent,
};

#[derive(Component)]
struct PlayerBlasterCollisionSound;

pub fn spawn_player_blaster_collision_sound(
    mut commands: Commands,
    mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
) {
    let player_blaster_collision_sound = PlayerBlasterCollisionSound;

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
