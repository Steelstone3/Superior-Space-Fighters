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
    events::{
        collision_events::PlayerBlasterCollisionEvent, logging_event::LoggingEvent,
        spawn_audio_event::SpawnAudioEvent,
    },
};

#[derive(Component)]
struct PlayerBlasterCollisionSound;

pub fn spawn_player_blaster_collision_sound(
    mut commands: Commands,
    mut logging_event: EventWriter<LoggingEvent>,
    mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
    mut player_blaster_collision_event: EventReader<PlayerBlasterCollisionEvent>,
) {
    for _ in player_blaster_collision_event.read() {
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

        logging_event.send(LoggingEvent {
            message: "Blaster collision with starship".to_string(),
        });

        // TODO No idea how to get THE starship in here
        // logging_event.send(LoggingEvent {
        //     message: format!(
        //         "Enemy Starship | Shield: {:?} | Health: {:?} |",
        //         100.0,
        //         100.0 //starship.starship.shield.current, starship.starship.hull.current
        //     ),
        // });
    }
}
