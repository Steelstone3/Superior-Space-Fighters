use bevy::{
    asset::AssetServer,
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    ecs::system::{Commands, Query, Res},
    input::{keyboard::KeyCode, ButtonInput},
};

use crate::components::starships::player_starship::PlayerStarship;

pub fn player_engine_rumble_sound(
    mut commands: Commands,
    players: Query<&PlayerStarship>,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.pressed(KeyCode::KeyW) {
        return;
    }

    let Ok(player) = players.get_single() else {
        return;
    };

    commands.spawn(AudioBundle {
        source: asset_server.load(player.ship.engine.to_string()),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.5),
            paused: false,
            ..Default::default()
        },
    });
}
