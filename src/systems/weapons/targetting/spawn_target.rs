use crate::components::{starships::starship::Starship, weapons::weapon_types::target::Target};
use bevy::{
    input::ButtonInput,
    prelude::{AssetServer, Commands, KeyCode, Query, Res, Transform},
    sprite::{Sprite, SpriteBundle},
};

#[allow(dead_code)]
pub fn spawn_target(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    player_starships: Query<(&Transform, &Starship)>,
    starships: Query<(&Transform, &Starship)>,
) {
    if !input.just_pressed(KeyCode::KeyT) {
        return;
    }

    let Ok(_player_starship) = player_starships.get_single() else {
        return;
    };

    for _starship in starships.into_iter() {}

    let target = Target::default();

    let texture = asset_server.load(target.lock_on_target.to_string());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(target.lock_on_target_size),
                ..Default::default()
            },
            // transform: player_transform,
            texture,
            ..Default::default()
        })
        .insert(target);
}
