use crate::{
    components::starship::Starship,
    systems::controllers::random_generator::{generate_seed, random_range_f32, random_range_i32},
};
use bevy::{
    prelude::{AssetServer, Commands, Res, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};
use rand::random;

pub fn spawn_random_station(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship = Starship {
        asset: random(),
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
            transform: bevy::prelude::Transform {
                translation: Vec3::new(
                    random_range_f32(generate_seed(), -320.0, 320.0),
                    random_range_f32(generate_seed(), -240.0, 240.0),
                    3.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ship);
}
