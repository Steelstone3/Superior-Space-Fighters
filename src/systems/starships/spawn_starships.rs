use crate::{
    components::starships::starship::Starship,
    events::{game_state_events::NewGameEvent, spawn_sprite_event::SpawnSpriteEvent},
    systems::controllers::random_generator::{generate_seed, random_value_f32, random_value_i32},
};
use bevy::{
    ecs::{
        event::{EventReader, EventWriter},
        system::Commands,
    },
    prelude::{Quat, Vec3},
    transform::components::Transform,
};
use rand::random;

pub fn spawn_random_starships(
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    mut commands: Commands,
    mut new_game_event_reader: EventReader<NewGameEvent>,
) {
    for _ in new_game_event_reader.read() {
        for _ in 0..random_value_i32(generate_seed(), 1..10) {
            let ship = Starship::new(random(), random());
            let texture = ship.faction_starship.to_string();
            let size = ship.size;
            let entity = commands.spawn(ship).id();

            let event = SpawnSpriteEvent {
                sprite_path: texture,
                size,
                entity,
                transform: Transform {
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
            };

            spawn_sprite_event.send(event);
        }
    }
}
