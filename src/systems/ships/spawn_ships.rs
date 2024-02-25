use crate::{
    components::starship::Starship,
    systems::controllers::random_generator::{generate_seed, random_value_f32, random_value_i32},
};
use bevy::{
    prelude::{AssetServer, Commands, Quat, Res, Vec3},
    sprite::{Sprite, SpriteBundle},
};
use rand::random;

pub fn spawn_random_ships(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..random_value_i32(generate_seed(), 1..10) {
        let ship = Starship::new(random(), random());

        let texture = asset_server.load(ship.faction_starship.to_string());

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(ship.size),
                    ..Default::default()
                },
                texture,
                transform: bevy::prelude::Transform {
                    translation: Vec3::new(
                        random_value_f32(generate_seed(), -320.0..320.0),
                        random_value_f32(generate_seed(), -320.0..320.0),
                        3.0,
                    ),
                    rotation: Quat::from_axis_angle(
                        Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        random_value_f32(generate_seed(), 0.0..360.0),
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(ship);
    }
}
