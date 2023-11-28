use crate::{
    components::{planet::Planet, player_blaster},
    systems::controllers::random_generator::{
        generate_seed, random_value_f32, random_value_i32, random_value_with_excluded_range_f32,
    },
};
use bevy::{
    prelude::{AssetServer, Commands, Res, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};
use rand::random;

pub fn spawn_random_planets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let planet_locations: [Vec2; 10] = [
        Vec2::default(),
        Vec2::default(),
        Vec2::default(),
        Vec2::default(),
        Vec2::default(),
        Vec2::default(),
        Vec2::default(),
        Vec2::default(),
        Vec2::default(),
        Vec2::default(),
    ];

    for _ in 0..random_value_i32(generate_seed(), 1..10) {
        let size = random_value_f32(generate_seed(), 25.0..500.0);

        let planet = Planet {
            planet: random(),
            size: Vec2 { x: size, y: size },
        };

        let texture = asset_server.load(planet.planet.to_string());

        let planet_x_locations = planet_locations
            .iter()
            .map(|vector| vector.x - 500.0..vector.x + 500.0)
            .collect();

        let planet_y_locations = planet_locations
            .iter()
            .map(|vector| vector.y - 500.0..vector.y + 500.0)
            .collect();

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(planet.size),
                    ..Default::default()
                },
                texture,
                transform: bevy::prelude::Transform {
                    translation: Vec3::new(
                        random_value_with_excluded_range_f32(-1920.0..1920.0, planet_x_locations),
                        random_value_with_excluded_range_f32(-1920.0..1920.0, planet_y_locations),
                        1.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(planet);
    }
}
