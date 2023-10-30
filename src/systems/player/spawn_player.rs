use bevy::{
    prelude::{AssetServer, Commands, Res, Vec2},
    sprite::{Sprite, SpriteBundle},
};

use crate::{assets::images::space_ships::SpaceShipSprite, components::player::Player};

pub fn spawn_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = Player {
        ship: SpaceShipSprite::SteelFactionShip1,
        speed: 400.0,
        size: Vec2 { x: 100.0, y: 100.0 },
    };

    let texture = asset_server.load(player.ship.to_string());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(player.size),
                ..Default::default()
            },
            texture,
            ..Default::default()
        })
        .insert(player);
}
