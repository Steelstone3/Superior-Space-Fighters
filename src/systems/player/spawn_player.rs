use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Quat,
    prelude::Vec3,
    transform::components::Transform,
};

use crate::{
    components::starships::player_starship::PlayerStarship,
    events::spawn_sprite_event::SpawnSpriteEvent,
};

pub fn spawn_player_ship(
    mut spawn_sprite_event_writer: EventWriter<SpawnSpriteEvent>,
    mut commands: Commands,
) {
    let player_starship = PlayerStarship::default();
    let texture = player_starship.ship.faction_starship.to_string();
    let size = player_starship.ship.size;
    let entity = commands.spawn(player_starship).id();

    let event = SpawnSpriteEvent {
        sprite_path: texture,
        size,
        entity,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 4.0),
            rotation: Quat::default(),
            ..Default::default()
        },
    };

    spawn_sprite_event_writer.send(event);
}
