use std::sync::{Arc, Mutex};

use crate::{
    components::starship::Starship,
    systems::controllers::random_generator::{generate_seed, random_range_f32, random_range_i32},
};
use bevy::{
    prelude::{AssetServer, Commands, Quat, Res, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};
use rand::random;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn spawn_random_ships(commands: Commands, asset_server: Res<AssetServer>) {
    let number_of_ships = random_range_i32(generate_seed(), 1, 10);

    // Create a thread bottleneck
    let arc_commands = Arc::new(Mutex::new(commands));

    (1..number_of_ships).into_par_iter().for_each(|_| {
        let ship = Starship {
            ship: random(),
            velocity: 30.0,
            rotation: f32::to_radians(10.0),
            size: Vec2 { x: 100.0, y: 100.0 },
            acceleration: 0.1,
            current_velocity: 0.0,
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                rotation: Quat::from_axis_angle(
                    Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 0.,
                    },
                    0.,
                ),
                scale: Vec3 {
                    x: 1.,
                    y: 1.,
                    z: 1.,
                },
            },
        };

        let texture = asset_server.load(ship.ship.to_string());

        // One thread at a time can access locked_commands (the bottleneck)
        let mut locked_commands = arc_commands.lock().unwrap();
        locked_commands
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
                    rotation: Quat::from_axis_angle(
                        Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        random_range_f32(generate_seed(), 0.0, 360.0),
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(ship);
    })
}
