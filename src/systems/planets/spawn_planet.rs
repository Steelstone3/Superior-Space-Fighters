use crate::{
    components::planet::Planet,
    events::spawn_planet_sprite_event::SpawnPlanetSpriteEvent,
    systems::controllers::random_generator::{generate_seed, random_value_f32, random_value_i32},
};
use bevy::{ecs::event::EventWriter, math::Vec3};
use rand::random;

pub fn spawn_random_planets(mut ev_spawn_sprite: EventWriter<SpawnPlanetSpriteEvent>) {
    for _ in 0..random_value_i32(generate_seed(), 1..10) {
        let size = random_value_f32(generate_seed(), 25.0..500.0);
        let planet = Planet::new(random(), size);
        let texture = planet.planet.to_string();
        let translation = Vec3::new(
            random_value_f32(generate_seed(), -1920.0..1920.0),
            random_value_f32(generate_seed(), -1920.0..1920.0),
            1.0,
        );

        let sprite_event = SpawnPlanetSpriteEvent {
            sprite_path: texture,
            size: planet.size,
            translation,
            planet,
        };
        ev_spawn_sprite.send(sprite_event);
    }
}
