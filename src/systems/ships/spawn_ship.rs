use bevy::{
    prelude::{AssetServer, Commands, Res, Vec2},
    sprite::{Sprite, SpriteBundle},
};

use crate::{assets::images::space_ships::SpaceShipSprite, components::starship::Starship};

pub fn spawn_random_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship = Starship {
        asset: SpaceShipSprite::SteelFactionShip1,
        velocity: 30.0,
        rotation: f32::to_radians(10.0),
        size: Vec2 { x: 100.0, y: 100.0 },
        acceleration: 0.1,
        current_velocity: 0.0,
    };

    let texture = asset_server.load(ship.asset.to_string());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(ship.size),
                ..Default::default()
            },
            texture,
            ..Default::default()
        })
        .insert(ship);
}
