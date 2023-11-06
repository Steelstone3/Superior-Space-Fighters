use crate::{
    components::planet::Planet,
    systems::controllers::random_generator::{generate_seed, random_range_f32, random_range_i32},
};
use bevy::{
    prelude::{AssetServer, Commands, Res, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};
use rand::random;

pub fn spawn_random_planets(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..random_range_i32(generate_seed(), 1, 5) {
        let size = random_range_f32(generate_seed(), 10.0, 500.0);

        let planet = Planet {
            planet: random(),
            size: Vec2 { x: size, y: size },
        };

        let texture = asset_server.load(planet.planet.to_string());

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(planet.size),
                    ..Default::default()
                },
                texture,
                transform: bevy::prelude::Transform {
                    translation: Vec3::new(
                        random_range_f32(generate_seed(), -320.0, 320.0),
                        random_range_f32(generate_seed(), -240.0, 240.0),
                        1.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(planet);
    }
}
