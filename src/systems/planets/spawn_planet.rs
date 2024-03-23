use crate::{
    components::planet::Planet,
    events::spawn_sprite_event::SpawnSpriteEvent,
    systems::controllers::random_generator::{generate_seed, random_value_f32, random_value_i32},
};
use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::{Quat, Vec3},
};
use rand::random;

pub fn spawn_random_planets(
    mut ev_spawn_sprite: EventWriter<SpawnSpriteEvent>,
    mut commands: Commands,
) {
    for _ in 0..random_value_i32(generate_seed(), 1..10) {
        let size = random_value_f32(generate_seed(), 25.0..500.0);
        let planet = Planet::new(random(), size);
        let size = planet.size;
        let texture = planet.planet.to_string();
        let translation = Vec3::new(
            random_value_f32(generate_seed(), -1920.0..1920.0),
            random_value_f32(generate_seed(), -1920.0..1920.0),
            1.0,
        );

        let entity = commands.spawn(planet).id();

        let sprite_event = SpawnSpriteEvent {
            sprite_path: texture,
            size,
            translation,
            entity,
            rotation: Quat::default(),
        };

        ev_spawn_sprite.send(sprite_event);
    }
}
