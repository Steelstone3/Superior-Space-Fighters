use crate::{
    assets::images::starships::factions::SpaceShipSprite,
    components::{player_starship::PlayerStarship, starship::Starship},
};
use bevy::{
    math::Quat,
    prelude::{AssetServer, Commands, Res, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

pub fn spawn_player_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = PlayerStarship {
        ship: Starship {
            ship: SpaceShipSprite::OuterReachMiningGuildShip1,
            velocity: 30.0,
            rotation: f32::to_radians(10.0),
            size: Vec2 { x: 100.0, y: 100.0 },
            acceleration: 0.1,
            current_velocity: 0.0,
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                rotation: Quat::from_rotation_z(0.0),
                scale: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
        },
    };

    let texture = asset_server.load(player.ship.ship.to_string());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(player.ship.size),
                ..Default::default()
            },
            texture,
            transform: bevy::prelude::Transform {
                translation: Vec3::new(0.0, 0.0, 4.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(player);
}
