use crate::{
    components::starships::starship::Starship,
    events::spawn_starship_sprite_event::SpawnStarshipSpriteEvent,
    systems::controllers::random_generator::{generate_seed, random_value_f32, random_value_i32},
};
use bevy::{
    ecs::event::EventWriter,
    prelude::{Quat, Vec3},
};
use rand::random;

pub fn spawn_random_ships(mut ev_spawn_starships: EventWriter<SpawnStarshipSpriteEvent>) {
    for _ in 0..random_value_i32(generate_seed(), 1..10) {
        let ship = Starship::new(random(), random());
        let texture = ship.faction_starship.to_string();

        let spawn_starship = SpawnStarshipSpriteEvent {
            sprite_path: texture,
            size: ship.size,
            translation: Vec3::new(
                random_value_f32(generate_seed(), -320.0..320.0),
                random_value_f32(generate_seed(), -320.0..320.0),
                3.0,
            ),
            ship: ship,
            rotation: Quat::from_axis_angle(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                random_value_f32(generate_seed(), 0.0..360.0),
            ),
        };

        ev_spawn_starships.send(spawn_starship);
    }
}
