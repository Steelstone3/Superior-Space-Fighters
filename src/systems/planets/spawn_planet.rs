use crate::{
    components::planet::Planet,
    events::game_state_events::NewGameEvent,
    systems::controllers::random_generator::{generate_seed, random_value_f32, random_value_i32},
};
use bevy::{
    ecs::event::EventReader,
    prelude::{AssetServer, Commands, Res, Vec3},
    sprite::{Sprite, SpriteBundle},
};
use rand::random;

pub fn spawn_random_planets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut new_game_event_reader: EventReader<NewGameEvent>,
) {
    for _ in new_game_event_reader.read() {
        for _ in 0..random_value_i32(generate_seed(), 1..10) {
            let size = random_value_f32(generate_seed(), 25.0..500.0);

            let planet = Planet::new(random(), size);

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
                            random_value_f32(generate_seed(), -1920.0..1920.0),
                            random_value_f32(generate_seed(), -1920.0..1920.0),
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(planet);
        }
    }
}
