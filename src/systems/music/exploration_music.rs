use bevy::{
    audio::PlaybackSettings,
    prelude::{AssetServer, AudioBundle, Commands, Res},
};

use crate::resources::music::Music;

pub fn play_exploration_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    music: Res<Music>,
) {
    commands.spawn(AudioBundle {
        source: asset_server.load(music.exploration_music.to_string()),
        settings: PlaybackSettings::LOOP,
    });
}
