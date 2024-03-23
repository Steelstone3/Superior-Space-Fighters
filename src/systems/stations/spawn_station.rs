use crate::{
    components::station::Station,
    events::spawn_sprite_event::SpawnSpriteEvent,
    systems::controllers::random_generator::{generate_seed, random_value_f32},
};

use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::{Quat, Vec3},
};
use rand::random;

pub fn spawn_random_station(
    mut station_event: EventWriter<SpawnSpriteEvent>,
    mut commands: Commands,
) {
    let station = Station::new(random(), 500.0);
    let texture = station.station.to_string();
    let size = station.size;

    let entity = commands.spawn(station).id();

    let event = SpawnSpriteEvent {
        sprite_path: texture,
        size,
        translation: Vec3::new(
            random_value_f32(generate_seed(), -320.0..320.0),
            random_value_f32(generate_seed(), -320.0..320.0),
            2.0,
        ),
        entity,
        rotation: Quat::default(),
    };

    station_event.send(event);
}
