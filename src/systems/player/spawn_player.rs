use bevy::{
    prelude::{AssetServer, Commands, Res, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle}, transform::components::Transform, math::Quat,
};

use crate::{
    assets::images::space_ships::SpaceShipSprite,
    components::{player_starship::PlayerStarship, starship::Starship},
};

pub fn spawn_player_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = PlayerStarship {
        ship: Starship {
            ship: SpaceShipSprite::SteelFactionShip1,
            velocity: 30.0,
            rotation: f32::to_radians(10.0),
            size: Vec2 { x: 100.0, y: 100.0 },
            acceleration: 0.1,
            current_velocity: 0.0,
            transform: Transform{ translation: Vec3{x:0.,y:0.,z:0.}, rotation: Quat::from_axis_angle(Vec3{x:0.,y:0.,z:0.}, 0.), scale: Vec3{x:1.,y:1.,z:1.} }
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
