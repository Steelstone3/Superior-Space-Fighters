use bevy::{
    prelude::{AssetServer, Commands, Res, Vec2},
    sprite::{Sprite, SpriteBundle},
};

use crate::components::player::Player;

pub fn spawn_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("images/space_ships/steel_faction/ship_1.png");

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            texture,
            ..Default::default()
        })
        .insert(Player { speed: 400.0 });
}
