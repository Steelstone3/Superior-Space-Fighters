use crate::{
    components::station::Station,
    systems::controllers::random_generator::{generate_seed, random_value_f32},
};
use bevy::{
    prelude::{AssetServer, Commands, Res, Vec3},
    sprite::{Sprite, SpriteBundle},
};
use rand::random;

pub fn spawn_random_station(mut commands: Commands, asset_server: Res<AssetServer>) {
    let station = Station::new(random(), 500.0);
    let texture = asset_server.load(station.station.to_string());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(station.size),
                ..Default::default()
            },
            texture,
            transform: bevy::prelude::Transform {
                translation: Vec3::new(
                    random_value_f32(generate_seed(), -320.0..320.0),
                    random_value_f32(generate_seed(), -320.0..320.0),
                    2.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(station);
}
