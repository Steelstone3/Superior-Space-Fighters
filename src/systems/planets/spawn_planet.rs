use crate::{
    components::station::Station,
    systems::controllers::random_generator::{generate_seed, random_range_f32},
};
use bevy::{
    prelude::{AssetServer, Commands, Res, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};
use rand::random;

pub fn spawn_random_planet(mut commands: Commands, asset_server: Res<AssetServer>) {
    let station = Station {
        asset: random(),
        size: Vec2 { x: 500.0, y: 500.0 },
    };

    let texture = asset_server.load(station.asset.to_string());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(station.size),
                ..Default::default()
            },
            texture,
            transform: bevy::prelude::Transform {
                translation: Vec3::new(
                    random_range_f32(generate_seed(), -320.0, 320.0),
                    random_range_f32(generate_seed(), -240.0, 240.0),
                    2.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(station);
}
