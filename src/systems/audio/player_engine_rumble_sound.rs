use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    ecs::system::{Commands, Res},
    utils::tracing,
};

use crate::assets::sounds::starships::engines::EngineSound;

pub fn player_engine_rumble_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    //TODO move to logging event
    tracing::info!("Engine rumble sound playing");
    let source = asset_server.load(EngineSound::default().to_string());
    commands.spawn(AudioBundle {
        source,
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.75),
            paused: false,
            ..Default::default()
        },
    });
}
