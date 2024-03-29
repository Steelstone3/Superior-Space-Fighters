use bevy::{
    audio::PlaybackSettings,
    ecs::{entity::Entity, event::Event},
};

#[derive(Event)]
pub struct SpawnAudioEvent {
    pub audio_path: String,
    pub playback_settings: PlaybackSettings,
    pub entity: Entity,
}
