use bevy::{
    asset::AssetServer,
    audio::AudioBundle,
    ecs::system::{Commands, Query, Res},
    input::{keyboard::KeyCode, ButtonInput},
};

use crate::components::player_starship::PlayerStarship;

pub fn player_engine_rumble_sound(
    mut commands: Commands,
    players: Query<&PlayerStarship>,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::KeyW) {
        return;
    }

    let Ok(player) = players.get_single() else {
        return;
    };

    commands.spawn(AudioBundle {
        source: asset_server.load(player.ship.engine.to_string()),
        ..Default::default()
    });
}
