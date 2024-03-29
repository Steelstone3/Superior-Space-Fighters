use crate::resources::music::Music;
use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    prelude::{Commands, Res},
};

pub fn play_exploration_music(
    mut commands: Commands,
    music: Res<Music>,
    asset_server: Res<AssetServer>,
    // mut spawn_audio_event: EventWriter<SpawnAudioEvent>,
) {
    // let entity = commands.spawn_empty().id();
    // let event = SpawnAudioEvent {
    //     audio_path: music.exploration_music.to_string(),
    //     playback_settings: PlaybackSettings {
    //         mode: PlaybackMode::Loop,
    //         volume: Volume::new(0.75),
    //         ..Default::default()
    //     },
    //     entity,
    // };

    // spawn_audio_event.send(event);

    commands.spawn(AudioBundle {
        source: asset_server.load(music.exploration_music.to_string()),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.75),
            ..Default::default()
        },
    });
}
